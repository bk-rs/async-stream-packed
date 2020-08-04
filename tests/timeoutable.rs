#[cfg(feature = "timeoutable")]
mod timeoutable_tests {
    use std::io;
    use std::net::{TcpListener, TcpStream};
    use std::time::{Duration, Instant};

    use async_io::Async;
    use futures_lite::future::block_on;

    use async_stream_packed::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};

    #[test]
    fn sample() -> io::Result<()> {
        block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0")?;
            let addr = listener.local_addr()?;

            let tcp_stream_c = TcpStream::connect(addr)?;
            let tcp_stream_s = listener
                .incoming()
                .next()
                .expect("Get next incoming failed")?;

            let mut tcp_stream_c = Async::<TcpStream>::new(tcp_stream_c)?;
            let mut tcp_stream_s = Async::<TcpStream>::new(tcp_stream_s)?;

            tcp_stream_s
                .write_with_timeout(b"foo", Duration::from_secs(1))
                .await?;

            let mut buf = vec![0u8; 5];
            let n = tcp_stream_c
                .read_with_timeout(&mut buf, Duration::from_secs(1))
                .await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            let instant = Instant::now();
            let two_secs = Duration::from_secs(2);
            let three_secs = Duration::from_secs(3);
            let err = tcp_stream_c
                .read_with_timeout(&mut buf, Duration::from_secs(2))
                .await
                .err()
                .unwrap();
            assert!(instant.elapsed() >= two_secs);
            assert!(instant.elapsed() < three_secs);
            assert_eq!(err.kind(), io::ErrorKind::TimedOut);
            assert_eq!(err.to_string(), "read timeout");

            Ok(())
        })
    }
}
