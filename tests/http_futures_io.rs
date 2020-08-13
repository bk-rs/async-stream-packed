#[cfg(all(feature = "http", feature = "futures_io", not(feature = "tokio_io")))]
mod http_futures_io_tests {
    use std::io;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use async_channel::{unbounded, Sender, TryRecvError};
    use async_trait::async_trait;
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;
    use futures_lite::{AsyncRead, AsyncWrite};
    use futures_lite::{AsyncReadExt, AsyncWriteExt};

    use async_stream_packed::{
        Downgrader, HttpClientInnerStream, HttpClientProxy, HttpTunnelClientGrader,
        TlsClientUpgrader, Upgrader,
    };

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

    struct SimpleHttpTunnelTlsUpgrader {
        sender: Sender<String>,
    }
    impl SimpleHttpTunnelTlsUpgrader {
        fn new(sender: Sender<String>) -> Self {
            Self { sender }
        }
    }

    #[async_trait]
    impl<S> TlsClientUpgrader<S> for SimpleHttpTunnelTlsUpgrader where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static
    {
    }

    #[async_trait]
    impl<S> Upgrader<S> for SimpleHttpTunnelTlsUpgrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = SimpleTlsStream<S>;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            self.sender
                .send("call SimpleHttpTunnelTlsUpgrader.upgrade".to_owned())
                .await
                .unwrap();

            Ok(SimpleTlsStream {
                inner: stream,
                sender: self.sender.clone(),
                owner: "SimpleHttpTunnelTlsUpgrader".to_owned(),
            })
        }
    }

    //
    //
    //
    struct SimpleHttpTunnelStream<S> {
        inner: S,
        sender: Sender<String>,
    }

    impl<S> SimpleHttpTunnelStream<S> {
        fn into_inner(self) -> S {
            self.inner
        }
    }

    impl<S> AsyncWrite for SimpleHttpTunnelStream<S>
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
                    .send("call SimpleHttpTunnelStream.poll_write".to_owned())
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

    impl<S> AsyncRead for SimpleHttpTunnelStream<S>
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
                    .send("call SimpleHttpTunnelStream.poll_read".to_owned())
                    .await
                    .unwrap()
            });

            Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
        }
    }

    struct SimpleHttpTunnelClientGrader {
        sender: Sender<String>,
    }
    impl SimpleHttpTunnelClientGrader {
        fn new(sender: Sender<String>) -> Self {
            Self { sender }
        }
    }

    #[async_trait]
    impl<S> HttpTunnelClientGrader<S> for SimpleHttpTunnelClientGrader where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static
    {
    }

    #[async_trait]
    impl<S> Upgrader<S> for SimpleHttpTunnelClientGrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = SimpleHttpTunnelStream<S>;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            self.sender
                .send("call SimpleHttpTunnelClientGrader.upgrade".to_owned())
                .await
                .unwrap();

            Ok(SimpleHttpTunnelStream {
                inner: stream,
                sender: self.sender.clone(),
            })
        }
    }

    #[async_trait]
    impl<S> Downgrader<S> for SimpleHttpTunnelClientGrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        async fn downgrade(&mut self, stream: <Self as Upgrader<S>>::Output) -> io::Result<S> {
            self.sender
                .send("call SimpleHttpTunnelClientGrader.downgrade".to_owned())
                .await
                .unwrap();

            Ok(stream.into_inner())
        }
    }

    //
    //
    //
    #[test]
    fn case1() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream =
                HttpClientInnerStream::<_, (), (), ()>::new(cursor, None, None).await?;
            assert!(stream.is_case1());

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            Ok(())
        })
    }

    #[test]
    fn case2() -> io::Result<()> {
        block_on(async {
            let (sender, receiver) = unbounded::<String>();

            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = HttpClientInnerStream::<_, (), (), _>::new(
                cursor,
                None,
                Some(SimpleTlsClientUpgrader::new(sender.clone())),
            )
            .await?;
            assert!(stream.is_case2());

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

    #[test]
    fn case3() -> io::Result<()> {
        block_on(async {
            let (sender, receiver) = unbounded::<String>();

            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = HttpClientInnerStream::<_, (), _, ()>::new(
                cursor,
                Some(HttpClientProxy::http(SimpleHttpTunnelClientGrader::new(
                    sender.clone(),
                ))),
                None,
            )
            .await?;
            assert!(stream.is_case3());

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.upgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.downgrade"
            );
            assert_eq!(receiver.try_recv().err(), Some(TryRecvError::Empty));

            Ok(())
        })
    }

    #[test]
    fn case4() -> io::Result<()> {
        block_on(async {
            let (sender, receiver) = unbounded::<String>();

            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = HttpClientInnerStream::<_, (), _, _>::new(
                cursor,
                Some(HttpClientProxy::http(SimpleHttpTunnelClientGrader::new(
                    sender.clone(),
                ))),
                Some(SimpleTlsClientUpgrader::new(sender.clone())),
            )
            .await?;
            assert!(stream.is_case4());

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.upgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.downgrade"
            );
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

    #[test]
    fn case5() -> io::Result<()> {
        block_on(async {
            let (sender, receiver) = unbounded::<String>();

            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = HttpClientInnerStream::<_, _, _, ()>::new(
                cursor,
                Some(HttpClientProxy::https(
                    SimpleHttpTunnelTlsUpgrader::new(sender.clone()),
                    SimpleHttpTunnelClientGrader::new(sender.clone()),
                )),
                None,
            )
            .await?;
            assert!(stream.is_case5());

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelTlsUpgrader.upgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.upgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.downgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsStream.poll_write via SimpleHttpTunnelTlsUpgrader"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsStream.poll_read via SimpleHttpTunnelTlsUpgrader"
            );
            assert_eq!(receiver.try_recv().err(), Some(TryRecvError::Empty));

            Ok(())
        })
    }

    #[test]
    fn case6() -> io::Result<()> {
        block_on(async {
            let (sender, receiver) = unbounded::<String>();

            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = HttpClientInnerStream::<_, _, _, _>::new(
                cursor,
                Some(HttpClientProxy::https(
                    SimpleHttpTunnelTlsUpgrader::new(sender.clone()),
                    SimpleHttpTunnelClientGrader::new(sender.clone()),
                )),
                Some(SimpleTlsClientUpgrader::new(sender.clone())),
            )
            .await?;
            assert!(stream.is_case6());

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelTlsUpgrader.upgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.upgrade"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleHttpTunnelClientGrader.downgrade"
            );
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
                "call SimpleTlsStream.poll_write via SimpleHttpTunnelTlsUpgrader"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsStream.poll_read via SimpleTlsClientUpgrader"
            );
            assert_eq!(
                receiver.recv().await.unwrap(),
                "call SimpleTlsStream.poll_read via SimpleHttpTunnelTlsUpgrader"
            );
            assert_eq!(receiver.try_recv().err(), Some(TryRecvError::Empty));

            Ok(())
        })
    }
}
