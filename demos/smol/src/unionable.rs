/*
cargo run -p async-stream-packed-demo-smol --bin unionable httpbin.org 80 false /ip
cargo run -p async-stream-packed-demo-smol --bin unionable httpbin.org 443 true /ip
*/

use std::env;
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::str;

use async_io::Async;
use async_tls::TlsConnector;
use blocking::block_on;
use futures::{AsyncReadExt, AsyncWriteExt};

use async_stream_packed::UnionableAsyncStream;

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("httpbin.org".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("80".to_owned()))
        .parse()
        .unwrap();
    let is_tls: bool = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("IS_TLS").unwrap_or("false".to_owned()))
        .parse()
        .unwrap();
    let path = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PATH").unwrap_or("/".to_owned()));

    println!("{} {} {} {}", domain, port, is_tls, path);

    //
    let addr = format!("{}:{}", domain, port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    println!("{}", addr);

    let mut stream = if is_tls {
        let stream = Async::<TcpStream>::connect(addr).await?;
        let stream = TlsConnector::default().connect(&domain, stream).await?;
        UnionableAsyncStream::one(stream)
    } else {
        let stream = Async::<TcpStream>::connect(addr).await?;
        UnionableAsyncStream::the_other(stream)
    };

    let req_string = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n",
        path, domain
    );
    println!("{}", req_string);

    stream.write(&req_string.as_bytes()).await?;

    let mut buf = vec![0u8; 288];
    stream.read(&mut buf).await?;

    println!("{:?}", str::from_utf8(&buf));

    println!("done");

    Ok(())
}
