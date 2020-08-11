#[cfg(feature = "upgradable")]
mod gradable_tests {
    use std::io;

    use async_trait::async_trait;
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;
    use futures_lite::io::{AsyncRead, AsyncWrite};

    use async_stream_packed::{Downgrader, GradableAsyncStream, Upgrader};

    //
    //
    //
    struct SimpleGrader {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleGrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            Ok(stream)
        }
    }

    #[async_trait]
    impl<S> Downgrader<S> for SimpleGrader
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
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
    struct SimpleGraderWithNotDowngradeRequired {}

    #[async_trait]
    impl<S> Upgrader<S> for SimpleGraderWithNotDowngradeRequired
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            Ok(stream)
        }
    }

    #[async_trait]
    impl<S> Downgrader<S> for SimpleGraderWithNotDowngradeRequired
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        async fn downgrade(
            &mut self,
            _output: <SimpleGrader as Upgrader<S>>::Output,
        ) -> io::Result<S> {
            unreachable!()
        }
        fn downgrade_required(&self) -> bool {
            false
        }
    }

    #[test]
    fn downgrade_required() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream =
                GradableAsyncStream::new(cursor, SimpleGraderWithNotDowngradeRequired {});
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.downgrade_required(), false);
            let err = stream.downgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "not allow");

            //
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = GradableAsyncStream::with_upgraded_stream(cursor);
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.downgrade_required(), false);
            let err = stream.downgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "downgrade not required");

            //
            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = GradableAsyncStream::with_upgraded_stream_and_grader(
                cursor,
                SimpleGraderWithNotDowngradeRequired {},
            );
            assert_eq!(stream.is_upgraded(), true);
            assert_eq!(stream.downgrade_required(), false);
            let err = stream.downgrade().await.err().unwrap();
            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "downgrade not required");

            Ok(())
        })
    }

    //
    //
    //
    #[derive(Default)]
    struct SimpleGraderWithOnceUpgradeAndOnceDowngrade {
        upgrade_count: usize,
        downgrade_count: usize,
    }

    #[async_trait]
    impl<S> Upgrader<S> for SimpleGraderWithOnceUpgradeAndOnceDowngrade
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        type Output = S;
        async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
            match self.upgrade_count {
                0 => {
                    self.upgrade_count += 1;
                    Ok(stream)
                }
                _ => unreachable!(),
            }
        }
        fn upgrade_required(&self) -> bool {
            match self.upgrade_count {
                0 => true,
                _ => false,
            }
        }
    }

    #[async_trait]
    impl<S> Downgrader<S> for SimpleGraderWithOnceUpgradeAndOnceDowngrade
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        async fn downgrade(
            &mut self,
            output: <SimpleGrader as Upgrader<S>>::Output,
        ) -> io::Result<S> {
            match self.downgrade_count {
                0 => {
                    self.downgrade_count += 1;
                    Ok(output)
                }
                _ => unreachable!(),
            }
        }
        fn downgrade_required(&self) -> bool {
            match self.downgrade_count {
                0 => true,
                _ => false,
            }
        }
    }

    #[test]
    fn once_upgrade_and_once_downgrade() -> io::Result<()> {
        block_on(async {
            let grader = SimpleGraderWithOnceUpgradeAndOnceDowngrade::default();
            assert_eq!(grader.upgrade_count, 0);
            assert_eq!(grader.downgrade_count, 0);

            let cursor = Cursor::new(Vec::<u8>::new());
            let mut stream = GradableAsyncStream::new(cursor, grader);

            assert_eq!(stream.is_upgraded(), false);
            stream.upgrade().await?;
            assert_eq!(stream.is_upgraded(), true);
            stream.downgrade().await?;
            assert_eq!(stream.is_upgraded(), false);
            assert_eq!(stream.upgrade_required(), false);

            Ok(())
        })
    }
}
