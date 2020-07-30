#[cfg(feature = "upgradable_ext")]
mod upgradable_ext_tests {
    use std::io;

    use async_trait::async_trait;
    use futures_executor::block_on;
    use futures_util::io::Cursor;

    use async_stream_packed::{UpgradableAsyncStream, Upgrader};
    use async_stream_packed::{UpgraderExtRefer, UpgraderExtTryIntoS};

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

    impl<S> UpgraderExtRefer<S> for SimpleUpgrader
    where
        S: Send + 'static,
    {
        fn get_ref(output: &Self::Output) -> &S {
            output
        }
        fn get_mut(output: &mut Self::Output) -> &mut S {
            output
        }
    }

    impl<S> UpgraderExtTryIntoS<S> for SimpleUpgrader
    where
        S: Send + 'static,
    {
        fn try_into_s(output: Self::Output) -> io::Result<S> {
            Ok(output)
        }
    }

    #[test]
    fn refer() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo");
            let mut stream = UpgradableAsyncStream::new(cursor, SimpleUpgrader {});

            assert_eq!(stream.get_ref().get_ref(), &b"foo");
            assert_eq!(stream.get_mut().get_mut(), &mut b"foo");

            stream.upgrade().await?;

            assert_eq!(stream.get_ref().get_ref(), &b"foo");
            assert_eq!(stream.get_mut().get_mut(), &mut b"foo");

            Ok(())
        })
    }

    #[test]
    fn try_into_s() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"foo");
            let stream = UpgradableAsyncStream::new(cursor, SimpleUpgrader {});

            assert_eq!(stream.try_into_s()?.get_ref(), &b"foo");

            let cursor = Cursor::new(b"foo");
            let stream = UpgradableAsyncStream::<_, SimpleUpgrader>::with_upgraded_stream(cursor);

            assert_eq!(stream.try_into_s()?.get_ref(), &b"foo");

            Ok(())
        })
    }
}
