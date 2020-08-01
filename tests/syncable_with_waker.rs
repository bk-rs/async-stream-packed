#[cfg(feature = "syncable_with_waker")]
mod syncable_with_waker_tests {
    use std::io::{self, Read, Seek, Write};
    use std::task::Poll;

    use futures_lite::future::{self, block_on};
    use futures_lite::io::Cursor;

    use async_stream_packed::{syncable_with_waker::WakerKind, SyncableWithWakerAsyncStream};

    #[test]
    fn cursor() -> io::Result<()> {
        block_on(async {
            let mut stream = future::poll_fn(|cx| {
                let cursor = Cursor::new(Vec::<u8>::new());
                let stream = SyncableWithWakerAsyncStream::new(cursor, cx.waker());

                Poll::Ready(stream)
            })
            .await;

            // test Write
            stream.get_mut().set_position(0);
            future::poll_fn(|cx| {
                stream.set_waker_with_kind(cx.waker(), WakerKind::Write);

                assert_eq!(stream.write(b"foo").ok(), Some(3));
                assert!(stream.flush().is_ok());

                Poll::Ready(())
            })
            .await;
            assert_eq!(stream.get_mut().get_ref(), b"foo");

            // test Seek
            stream.get_mut().set_position(0);
            future::poll_fn(|cx| {
                stream.set_waker_with_kind(cx.waker(), WakerKind::Read);

                assert_eq!(stream.seek(io::SeekFrom::Start(2)).ok(), Some(2));

                let mut buf = vec![0; 2];
                assert_eq!(stream.read(&mut buf).ok(), Some(1));

                assert_eq!(buf, b"o\0");

                Poll::Ready(())
            })
            .await;
            assert_eq!(stream.get_mut().get_ref(), b"foo");

            // test Read
            stream.get_mut().set_position(0);
            future::poll_fn(|cx| {
                stream.set_waker_with_kind(cx.waker(), WakerKind::Read);

                let mut buf = vec![0; 4];
                assert_eq!(stream.read(&mut buf).ok(), Some(3));

                assert_eq!(buf, b"foo\0");

                Poll::Ready(())
            })
            .await;
            assert_eq!(stream.get_mut().get_ref(), b"foo");

            Ok(())
        })
    }
}
