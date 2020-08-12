#[cfg(all(
    feature = "syncable_with_context",
    feature = "futures_io",
    not(feature = "tokio_io")
))]
mod syncable_with_context_futures_io_tests {
    use std::io::{self, BufRead, Read, Seek, Write};
    use std::task::Poll;

    use futures_lite::future::{self, block_on};
    use futures_lite::io::Cursor;

    use async_stream_packed::SyncableWithContextAsyncStream;

    #[test]
    fn cursor() -> io::Result<()> {
        block_on(async {
            let mut cursor = Cursor::new(Vec::<u8>::new());

            // test Write
            cursor.set_position(0);
            future::poll_fn(|cx| {
                let mut stream = SyncableWithContextAsyncStream::new(&mut cursor, cx);

                assert_eq!(stream.write(b"foo").ok(), Some(3));
                assert!(stream.flush().is_ok());

                Poll::Ready(())
            })
            .await;
            assert_eq!(cursor.get_ref(), b"foo");

            // test BufRead and Seek
            cursor.set_position(0);
            future::poll_fn(|cx| {
                let mut stream = SyncableWithContextAsyncStream::new(&mut cursor, cx);

                assert_eq!(stream.fill_buf().ok(), Some(&b"foo"[..]));

                stream.consume(1);
                assert_eq!(stream.fill_buf().ok(), Some(&b"oo"[..]));

                stream.consume(0);

                assert_eq!(stream.seek(io::SeekFrom::Start(2)).ok(), Some(2));
                assert_eq!(stream.fill_buf().ok(), Some(&b"o"[..]));

                Poll::Ready(())
            })
            .await;
            assert_eq!(cursor.get_ref(), b"foo");

            // test Read
            cursor.set_position(0);
            future::poll_fn(|cx| {
                let mut stream = SyncableWithContextAsyncStream::new(&mut cursor, cx);

                let mut buf = vec![0; 4];
                assert_eq!(stream.read(&mut buf).ok(), Some(3));

                assert_eq!(buf, b"foo\0");

                Poll::Ready(())
            })
            .await;
            assert_eq!(cursor.get_ref(), b"foo");

            Ok(())
        })
    }
}
