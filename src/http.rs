use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, SeekFrom};

use crate::gradable::Downgrader;
use crate::tls::TlsClientUpgrader;
use crate::upgradable::{UpgradableAsyncStream, Upgrader};

pub trait HttpTunnelGrader<S>: Upgrader<S> + Downgrader<S> {}

impl<S> HttpTunnelGrader<S> for () where S: Send + 'static {}

//
//
//
/*
Cases:

1. curl http://httpbin.org/ip -v

2. curl https://httpbin.org/ip -v

3. curl -x http://127.0.0.1:8118 http://httpbin.org/ip -v

4. curl -x http://127.0.0.1:8118 https://httpbin.org/ip -v

5. curl -x https://proxy.lvh.me:9118 http://httpbin.org/ip -v

6. curl -x https://proxy.lvh.me:9118 https://httpbin.org/ip -v --proxy-insecure

7. curl -x socks5://127.0.0.1:1080 http://httpbin.org/ip -v

8. curl -x socks5://127.0.0.1:1080 https://httpbin.org/ip -v

*/

pub struct HttpClientInnerStream<S> {
    inner: S,
}
impl<S> HttpClientInnerStream<S> {
    pub(crate) fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> HttpClientInnerStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    // Resolve cases: 1, 3
    pub fn tcp(stream: S) -> HttpClientInnerStream<S> {
        HttpClientInnerStream::new(stream)
    }

    // Resolve cases: 2, 5
    pub async fn tcp_then_tls<TU>(
        stream: S,
        tls_upgrader: TU,
    ) -> io::Result<HttpClientInnerStream<UpgradableAsyncStream<S, TU>>>
    where
        TU: TlsClientUpgrader<S>,
    {
        let mut stream = UpgradableAsyncStream::new(stream, tls_upgrader);
        stream.upgrade().await?;

        Ok(HttpClientInnerStream::new(stream))
    }

    // Resolve cases: 4
    pub async fn tcp_then_http_tunnel_then_tls<HTG, TU>(
        stream: S,
        mut http_tunnel_grader: HTG,
        tls_upgrader: TU,
    ) -> io::Result<HttpClientInnerStream<UpgradableAsyncStream<S, TU>>>
    where
        HTG: HttpTunnelGrader<S>,
        TU: TlsClientUpgrader<S>,
    {
        let http_tunnel_stream = http_tunnel_grader.upgrade(stream).await?;
        let stream = http_tunnel_grader.downgrade(http_tunnel_stream).await?;

        let mut stream = UpgradableAsyncStream::new(stream, tls_upgrader);
        stream.upgrade().await?;

        Ok(HttpClientInnerStream::new(stream))
    }

    // Resolve cases: 6
    pub async fn tcp_then_tls_then_http_tunnel_then_tls<HTTU, HTG, TU>(
        stream: S,
        http_tunnel_tls_upgrader: HTTU,
        mut http_tunnel_grader: HTG,
        tls_upgrader: TU,
    ) -> io::Result<HttpClientInnerStream<UpgradableAsyncStream<HTTU::Output, TU>>>
    where
        HTTU: TlsClientUpgrader<S>,
        HTG: HttpTunnelGrader<UpgradableAsyncStream<S, HTTU>>,
        TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S>,
    {
        let mut stream = UpgradableAsyncStream::new(stream, http_tunnel_tls_upgrader);
        stream.upgrade().await?;

        let http_tunnel_stream = http_tunnel_grader.upgrade(stream).await?;
        let stream = http_tunnel_grader.downgrade(http_tunnel_stream).await?;

        let stream = stream.try_into_upgraded_stream()?;

        let mut stream = UpgradableAsyncStream::new(stream, tls_upgrader);
        stream.upgrade().await?;

        Ok(HttpClientInnerStream::new(stream))
    }
}

impl<S> AsyncWrite for HttpClientInnerStream<S>
where
    S: AsyncWrite + Unpin,
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_close(cx)
    }
}

impl<S> AsyncRead for HttpClientInnerStream<S>
where
    S: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_read(cx, buf)
    }
}

impl<S> AsyncSeek for HttpClientInnerStream<S>
where
    S: AsyncSeek + Unpin,
{
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context, pos: SeekFrom) -> Poll<io::Result<u64>> {
        Pin::new(&mut self.get_mut().inner).poll_seek(cx, pos)
    }
}

impl<S> AsyncBufRead for HttpClientInnerStream<S>
where
    S: AsyncBufRead + Unpin,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<&[u8]>> {
        Pin::new(&mut self.get_mut().inner).poll_fill_buf(cx)
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.get_mut().inner).consume(amt)
    }
}
