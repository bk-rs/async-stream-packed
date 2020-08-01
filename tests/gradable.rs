#[cfg(feature = "gradable")]
mod gradable_tests {
    use std::io;

    use async_trait::async_trait;
    use futures_executor::block_on;
    use futures_util::io::Cursor;

    use async_stream_packed::{Downgrader, GradableAsyncStream, Upgrader};

    //
    //
    //
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
    impl<S> Downgrader<S> for SimpleGrader
    where
        S: Send + 'static,
    {
        async fn downgrade(
            &mut self,
            output: <SimpleGrader as Upgrader<S>>::Output,
        ) -> io::Result<S> {
            Ok(output)
        }
    }

    #[test]
    fn with_upgraded_stream_and_grader() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream =
                GradableAsyncStream::with_upgraded_stream_and_grader(cursor, SimpleGrader {});
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.upgrade_required(), false);
            assert_eq!(stream.downgrade_required(), true);
            stream.downgrade().await?;
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.upgrade_required(), true);
            assert_eq!(stream.downgrade_required(), false);

            Ok(())
        })
    }

    #[test]
    fn downgrade() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = GradableAsyncStream::new(cursor, SimpleGrader {});
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.upgrade_required(), true);
            assert_eq!(stream.downgrade_required(), false);
            stream.upgrade().await?;
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.upgrade_required(), false);
            assert_eq!(stream.downgrade_required(), true);
            stream.downgrade().await?;
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.upgrade_required(), true);
            assert_eq!(stream.downgrade_required(), false);

            Ok(())
        })
    }

    //
    //
    //
    struct SimpleGraderWithCannotDowngrade {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleGraderWithCannotDowngrade
    where
        S: Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            Ok(stream)
        }
    }

    #[async_trait]
    impl<S> Downgrader<S> for SimpleGraderWithCannotDowngrade
    where
        S: Send + 'static,
    {
        async fn downgrade(
            &mut self,
            output: <SimpleGrader as Upgrader<S>>::Output,
        ) -> io::Result<S> {
            Ok(output)
        }
        fn downgrade_required(&self) -> bool {
            false
        }
    }

    #[test]
    fn downgrade_required() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = GradableAsyncStream::new(cursor, SimpleGraderWithCannotDowngrade {});
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.downgrade_required(), false);
            let err = stream.downgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "not allow");

            //
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream =
                GradableAsyncStream::<_, SimpleGraderWithCannotDowngrade>::with_upgraded_stream(
                    cursor,
                );
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.downgrade_required(), false);
            let err = stream.downgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "missing grader");

            //
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = GradableAsyncStream::with_upgraded_stream_and_grader(
                cursor,
                SimpleGraderWithCannotDowngrade {},
            );
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.downgrade_required(), false);
            let err = stream.downgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "downgrade not required");

            Ok(())
        })
    }
}
