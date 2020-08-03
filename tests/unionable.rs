#[cfg(feature = "unionable")]
mod unionable_tests {
    use std::io;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use futures_io::{AsyncRead, AsyncWrite};
    use futures_lite::future::block_on;
    use futures_lite::io::{empty, Cursor, Empty};
    use futures_lite::{AsyncReadExt, AsyncWriteExt};

    use async_stream_packed::UnionableAsyncStream;

    //
    //
    //
    struct WritableEmpty {
        inner: Empty,
    }

    impl AsyncWrite for WritableEmpty {
        fn poll_write(
            self: Pin<&mut Self>,
            _cx: &mut Context,
            _buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            Poll::Ready(Ok(0))
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    impl AsyncRead for WritableEmpty {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
        }
    }

    //
    //
    //
    #[test]
    fn sample() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = if true {
                UnionableAsyncStream::one(cursor)
            } else {
                UnionableAsyncStream::the_other(WritableEmpty { inner: empty() })
            };

            stream.write(b"").await?;

            let mut buf = vec![0u8; 5];
            let n = stream.read(&mut buf).await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            Ok(())
        })
    }
}
