#[cfg(feature = "gradable")]
mod gradable_tests {
    use std::io;

    use async_trait::async_trait;
    use futures_executor::block_on;
    use futures_util::io::Cursor;

    use async_stream_packed::{Downgrader, GradableAsyncStream, Upgrader};

    struct SimpleGrader {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleGrader
    where
        S: Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            Ok(stream)
        }
    }

    #[async_trait]
    impl<S> Downgrader<S, SimpleGrader> for SimpleGrader
    where
        S: Send + 'static,
    {
        async fn downgrade(
            output: <SimpleGrader as Upgrader<S>>::Output,
            upgrader: Option<SimpleGrader>,
        ) -> io::Result<(S, Option<SimpleGrader>)> {
            Ok((output, upgrader))
        }
    }

    #[test]
    fn downgrade() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = GradableAsyncStream::new(cursor, SimpleGrader {});
            assert_eq!(stream.is_upgraded(), false);
            stream.upgrade().await?;
            assert_eq!(stream.is_upgraded(), true);
            stream.downgrade().await?;
            assert_eq!(stream.is_upgraded(), false);

            Ok(())
        })
    }
}
