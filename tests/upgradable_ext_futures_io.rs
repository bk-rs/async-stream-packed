#[cfg(all(
    feature = "upgradable",
    feature = "futures_io",
    not(feature = "tokio_io")
))]
mod upgradable_ext_futures_io_tests {
    use std::io;

    use async_trait::async_trait;
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;
    use futures_lite::io::{AsyncRead, AsyncWrite};

    use async_stream_packed::{
        UpgradableAsyncStream, Upgrader, UpgraderExtIntoStream, UpgraderExtRefer,
    };

    struct SimpleUpgrader {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleUpgrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            Ok(stream)
        }
    }

    impl<S> UpgraderExtRefer<S> for SimpleUpgrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        fn get_ref(output: &Self::Output) -> &S {
            output
        }
        fn get_mut(output: &mut Self::Output) -> &mut S {
            output
        }
    }

    impl<S> UpgraderExtIntoStream<S> for SimpleUpgrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        fn into_stream(output: Self::Output) -> io::Result<S> {
            Ok(output)
        }
    }

    #[test]
    fn refer() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = UpgradableAsyncStream::new(cursor, SimpleUpgrader {});

            assert_eq!(stream.get_ref().get_ref(), &b"foo");
            assert_eq!(stream.get_mut().get_mut(), &mut b"foo");

            stream.upgrade().await?;

            assert_eq!(stream.get_ref().get_ref(), &b"foo");
            assert_eq!(stream.get_mut().get_mut(), &mut b"foo");

            //
            let cursor = Cursor::new(b"foo".to_vec());
            let mut stream = UpgradableAsyncStream::with_upgraded_stream(cursor);

            assert_eq!(stream.get_ref().get_ref(), &b"foo");
            assert_eq!(stream.get_mut().get_mut(), &mut b"foo");

            Ok(())
        })
    }

    #[test]
    fn into_stream() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo".to_vec());
            let stream = UpgradableAsyncStream::new(cursor, SimpleUpgrader {});

            assert_eq!(stream.into_stream()?.get_ref(), &b"foo");

            //
            let cursor = Cursor::new(b"foo".to_vec());
            let stream = UpgradableAsyncStream::with_upgraded_stream(cursor);

            assert_eq!(stream.into_stream()?.get_ref(), &b"foo");

            Ok(())
        })
    }

    //
    //
    //
    struct SimpleUpgraderWithoutIntoStream {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleUpgraderWithoutIntoStream
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            Ok(stream)
        }
    }

    #[test]
    fn try_into_stream() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo".to_vec());
            let stream = UpgradableAsyncStream::new(cursor, SimpleUpgrader {});

            assert_eq!(stream.try_into_stream()?.get_ref(), &b"foo");

            //
            let cursor = Cursor::new(b"foo".to_vec());
            let stream = UpgradableAsyncStream::with_upgraded_stream(cursor);

            let err = stream.try_into_stream().err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "unimplemented");

            Ok(())
        })
    }
}
