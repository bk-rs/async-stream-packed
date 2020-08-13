#[cfg(all(
    feature = "upgradable",
    feature = "futures_io",
    not(feature = "tokio_io")
))]
mod mail_futures_io_tests {
    use std::io;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use async_channel::{unbounded, Sender, TryRecvError};
    use async_trait::async_trait;
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;
    use futures_lite::{AsyncRead, AsyncWrite};
    use futures_lite::{AsyncReadExt, AsyncWriteExt};

    use async_stream_packed::{SmtpClientInnerStream, TlsClientUpgrader, Upgrader};

    //
    //
    //
    struct SimpleTlsStream<S> {
        inner: S,
        sender: Sender<String>,
        owner: String,
    }

    impl<S> AsyncWrite for SimpleTlsStream<S>
    where
        S: AsyncWrite + Unpin,
    {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            block_on(async {
                self.sender
                    .send(format!(
                        "call SimpleTlsStream.poll_write via {}",
                        self.owner
                    ))
                    .await
                    .unwrap()
            });

            Pin::new(&mut self.get_mut().inner).poll_write(cx, buf)
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
            Pin::new(&mut self.get_mut().inner).poll_flush(cx)
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
            Pin::new(&mut self.get_mut().inner).poll_close(cx)
        }
    }

    impl<S> AsyncRead for SimpleTlsStream<S>
    where
        S: AsyncRead + Unpin,
    {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            block_on(async {
                self.sender
                    .send(format!("call SimpleTlsStream.poll_read via {}", self.owner))
                    .await
                    .unwrap()
            });

            Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
        }
    }

    struct SimpleTlsClientUpgrader {
        sender: Sender<String>,
    }
    impl SimpleTlsClientUpgrader {
        fn new(sender: Sender<String>) -> Self {
            Self { sender }
        }
    }

    #[async_trait]
    impl<S> TlsClientUpgrader<S> for SimpleTlsClientUpgrader where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static
    {
    }

    #[async_trait]
    impl<S> Upgrader<S> for SimpleTlsClientUpgrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = SimpleTlsStream<S>;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            self.sender
                .send("call SimpleTlsClientUpgrader.upgrade".to_owned())
                .await
                .unwrap();

            Ok(SimpleTlsStream {
                inner: stream,
                sender: self.sender.clone(),
                owner: "SimpleTlsClientUpgrader".to_owned(),
            })
        }
    }

    //
    //
    //
    #[test]
    fn smtp_case1() -> io::Result<()> {
        block_on(async {
            let (sender, receiver) = unbounded::<String>();

            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = SmtpClientInnerStream::with_imap_client(
                cursor,
                SimpleTlsClientUpgrader::new(sender.clone()),
            );

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            stream.upgrade().await?;

            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsClientUpgrader.upgrade"
            );
            assert_eq!(receiver.try_recv().err(), Some(TryRecvError::Empty));

            Ok(())
        })
    }

    #[test]
    fn smtp_case2() -> io::Result<()> {
        block_on(async {
            let (sender, receiver) = unbounded::<String>();

            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = SmtpClientInnerStream::with_imap_client(
                cursor,
                SimpleTlsClientUpgrader::new(sender.clone()),
            );
            stream.upgrade().await?;

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsClientUpgrader.upgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsStream.poll_write via SimpleTlsClientUpgrader"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsStream.poll_read via SimpleTlsClientUpgrader"
            );
            assert_eq!(receiver.try_recv().err(), Some(TryRecvError::Empty));

            Ok(())
        })
    }
}
