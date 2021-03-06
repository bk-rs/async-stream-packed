#[cfg(all(
    feature = "upgradable",
    feature = "futures_io",
    not(feature = "tokio_io")
))]
mod upgradable_futures_io_tests {
    use std::io;

    use async_trait::async_trait;
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;
    use futures_lite::{AsyncRead, AsyncWrite};

    use async_stream_packed::{UpgradableAsyncStream, Upgrader};

    //
    //
    //
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

    #[test]
    fn new() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = UpgradableAsyncStream::new(cursor, SimpleUpgrader {});
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.upgrade_required(), true);
            stream.upgrade().await?;
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.upgrade_required(), false);

            Ok(())
        })
    }

    #[test]
    fn with_stream() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let stream = UpgradableAsyncStream::new(cursor, ());
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.upgrade_required(), false);

            Ok(())
        })
    }

    #[test]
    fn with_upgraded_stream() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let stream = UpgradableAsyncStream::with_upgraded_stream(cursor);
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.upgrade_required(), false);

            Ok(())
        })
    }

    //
    //
    //
    struct SimpleUpgraderWithNotUpgradeRequired {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleUpgraderWithNotUpgradeRequired
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, _stream: S) -> io::Result<Self::Output> {
            unreachable!()
        }
        fn upgrade_required(&self) -> bool {
            false
        }
    }

    #[test]
    fn upgrade_required() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream =
                UpgradableAsyncStream::new(cursor, SimpleUpgraderWithNotUpgradeRequired {});
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.upgrade_required(), false);
            let err = stream.upgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "upgrade not required");

            //
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = UpgradableAsyncStream::with_upgraded_stream(cursor);
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.upgrade_required(), false);
            let err = stream.upgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "not allow");

            Ok(())
        })
    }
}
