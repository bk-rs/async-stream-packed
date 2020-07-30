#[cfg(feature = "upgradable")]
mod upgradable_tests {
    use std::io;

    use async_trait::async_trait;
    use futures_executor::block_on;
    use futures_util::io::Cursor;

    use async_stream_packed::{UpgradableAsyncStream, Upgrader};

    struct SimpleUpgrader {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleUpgrader
    where
        S: Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            Ok(stream)
        }
    }

    #[test]
    fn upgrade() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = UpgradableAsyncStream::new(cursor, SimpleUpgrader {});
            assert_eq!(stream.is_upgraded(), false);
            stream.upgrade().await?;
            assert_eq!(stream.is_upgraded(), true);

            Ok(())
        })
    }

    #[test]
    fn with_upgraded_stream() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let stream = UpgradableAsyncStream::<_, SimpleUpgrader>::with_upgraded_stream(cursor);
            assert_eq!(stream.is_upgraded(), true);

            Ok(())
        })
    }
}
