use std::io::{self, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_x_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

use crate::gradable::Downgrader;
use crate::tls::TlsClientUpgrader;
use crate::upgradable::{UpgradableAsyncStream, Upgrader};

pub trait HttpTunnelClientGrader<S>: Upgrader<S> + Downgrader<S> {}

impl<S> HttpTunnelClientGrader<S> for () where S: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

pub trait HttpTunnelServerGrader<S>: Upgrader<S> + Downgrader<S> {}

impl<S> HttpTunnelServerGrader<S> for () where S: AsyncRead + AsyncWrite + Unpin + Send + 'static {}

//
//
//
pub enum HttpClientInnerStream<S, HTTU, HTG, TU>
where
    S: AsyncRead + AsyncWrite + Unpin,
    HTTU: TlsClientUpgrader<S> + Unpin,
    HTTU::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<S> + Unpin,
    <HTG as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<HTTU::Output> + Unpin,
    <HTG as Upgrader<HTTU::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
    TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S> + Unpin,
    <TU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    <TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
{
    // curl http://httpbin.org/ip -v
    Case1(S),

    // curl https://httpbin.org/ip -v
    Case2(<TU as Upgrader<S>>::Output),

    // curl -x http://127.0.0.1:8118 http://httpbin.org/ip -v
    Case3(S),

    // curl -x http://127.0.0.1:8118 https://httpbin.org/ip -v
    Case4(<TU as Upgrader<S>>::Output),

    // curl -x https://proxy.lvh.me:9118 http://httpbin.org/ip -v
    Case5(<HTTU as Upgrader<S>>::Output),

    // curl -x https://proxy.lvh.me:9118 https://httpbin.org/ip -v --proxy-insecure
    Case6(<TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output),

    // curl -x socks5://127.0.0.1:1080 http://httpbin.org/ip -v
    Case7,

    // curl -x socks5://127.0.0.1:1080 https://httpbin.org/ip -v
    Case8,

    Never(HTG),
}

impl<S, HTTU, HTG, TU> HttpClientInnerStream<S, HTTU, HTG, TU>
where
    S: AsyncRead + AsyncWrite + Unpin,
    HTTU: TlsClientUpgrader<S> + Unpin,
    HTTU::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<S> + Unpin,
    <HTG as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<HTTU::Output> + Unpin,
    <HTG as Upgrader<HTTU::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
    TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S> + Unpin,
    <TU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    <TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
{
    pub fn is_case1(&self) -> bool {
        match self {
            HttpClientInnerStream::Case1(_) => true,
            _ => false,
        }
    }

    pub fn is_case2(&self) -> bool {
        match self {
            HttpClientInnerStream::Case2(_) => true,
            _ => false,
        }
    }

    pub fn is_case3(&self) -> bool {
        match self {
            HttpClientInnerStream::Case3(_) => true,
            _ => false,
        }
    }

    pub fn is_case4(&self) -> bool {
        match self {
            HttpClientInnerStream::Case4(_) => true,
            _ => false,
        }
    }

    pub fn is_case5(&self) -> bool {
        match self {
            HttpClientInnerStream::Case5(_) => true,
            _ => false,
        }
    }

    pub fn is_case6(&self) -> bool {
        match self {
            HttpClientInnerStream::Case6(_) => true,
            _ => false,
        }
    }
}

impl<S, HTTU, HTG, TU> HttpClientInnerStream<S, HTTU, HTG, TU>
where
    S: AsyncRead + AsyncWrite + Unpin,
    HTTU: TlsClientUpgrader<S> + Unpin,
    HTTU::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<S> + Unpin,
    <HTG as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<HTTU::Output> + Unpin,
    <HTG as Upgrader<HTTU::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
    TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S> + Unpin,
    <TU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    <TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn new(
        stream: S,
        proxy: Option<HttpClientProxy<S, HTTU, HTG>>,
        tls_upgrader: Option<TU>,
    ) -> io::Result<Self> {
        if let Some(proxy) = proxy {
            match proxy.inner {
                HttpClientProxyInner::Http(http_tunnel_grader) => {
                    let mut stream = UpgradableAsyncStream::new(stream, http_tunnel_grader);
                    stream.upgrade().await?;
                    stream.downgrade().await?;
                    let stream = stream.try_into_stream()?;

                    if let Some(tls_upgrader) = tls_upgrader {
                        let mut stream = UpgradableAsyncStream::new(stream, tls_upgrader);
                        stream.upgrade().await?;
                        let stream = stream.try_into_upgraded_stream()?;
                        return Ok(Self::Case4(stream));
                    }
                    return Ok(Self::Case3(stream));
                }
                HttpClientProxyInner::Https(http_tunnel_tls_upgrader, http_tunnel_grader) => {
                    let mut stream = UpgradableAsyncStream::new(stream, http_tunnel_tls_upgrader);
                    stream.upgrade().await?;
                    let stream = stream.try_into_upgraded_stream()?;

                    let mut stream = UpgradableAsyncStream::new(stream, http_tunnel_grader);
                    stream.upgrade().await?;
                    stream.downgrade().await?;
                    let stream = stream.try_into_stream()?;

                    if let Some(tls_upgrader) = tls_upgrader {
                        let mut stream = UpgradableAsyncStream::new(stream, tls_upgrader);
                        stream.upgrade().await?;
                        let stream = stream.try_into_upgraded_stream()?;
                        return Ok(Self::Case6(stream));
                    }
                    return Ok(Self::Case5(stream));
                }
                HttpClientProxyInner::Never(_) => panic!("never"),
            }
        }

        if let Some(tls_upgrader) = tls_upgrader {
            let mut stream = UpgradableAsyncStream::new(stream, tls_upgrader);
            stream.upgrade().await?;
            let stream = stream.try_into_upgraded_stream()?;

            return Ok(Self::Case2(stream));
        }

        return Ok(Self::Case1(stream));
    }
}

//
//
//
macro_rules! case {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            HttpClientInnerStream::Case1($pattern) => $result,
            HttpClientInnerStream::Case2($pattern) => $result,
            HttpClientInnerStream::Case3($pattern) => $result,
            HttpClientInnerStream::Case4($pattern) => $result,
            HttpClientInnerStream::Case5($pattern) => $result,
            HttpClientInnerStream::Case6($pattern) => $result,
            HttpClientInnerStream::Case7 => unreachable!(),
            HttpClientInnerStream::Case8 => unreachable!(),
            HttpClientInnerStream::Never(_) => panic!("never"),
        }
    };
}

impl<S, HTTU, HTG, TU> AsyncWrite for HttpClientInnerStream<S, HTTU, HTG, TU>
where
    S: AsyncRead + AsyncWrite + Unpin,
    HTTU: TlsClientUpgrader<S> + Unpin,
    HTTU::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<S> + Unpin,
    <HTG as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<HTTU::Output> + Unpin,
    <HTG as Upgrader<HTTU::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
    TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S> + Unpin,
    <TU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    <TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_write(cx, buf))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_flush(cx))
    }

    #[cfg(all(feature = "futures_io", not(feature = "tokio_io")))]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_close(cx))
    }

    #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_shutdown(cx))
    }
}

impl<S, HTTU, HTG, TU> AsyncRead for HttpClientInnerStream<S, HTTU, HTG, TU>
where
    S: AsyncRead + AsyncWrite + Unpin,
    HTTU: TlsClientUpgrader<S> + Unpin,
    HTTU::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<S> + Unpin,
    <HTG as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    HTG: HttpTunnelClientGrader<HTTU::Output> + Unpin,
    <HTG as Upgrader<HTTU::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
    TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S> + Unpin,
    <TU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin,
    <TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_read(cx, buf))
    }
}

impl<S, HTTU, HTG, TU> AsyncSeek for HttpClientInnerStream<S, HTTU, HTG, TU>
where
    S: AsyncRead + AsyncWrite + Unpin + AsyncSeek,
    HTTU: TlsClientUpgrader<S> + Unpin,
    HTTU::Output: AsyncRead + AsyncWrite + Unpin + AsyncSeek,
    HTG: HttpTunnelClientGrader<S> + Unpin,
    <HTG as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin + AsyncSeek,
    HTG: HttpTunnelClientGrader<HTTU::Output> + Unpin + AsyncSeek,
    <HTG as Upgrader<HTTU::Output>>::Output: AsyncRead + AsyncWrite + Unpin + AsyncSeek,
    TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S> + Unpin,
    <TU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin + AsyncSeek,
    <TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output:
        AsyncRead + AsyncWrite + Unpin + AsyncSeek,
{
    #[cfg(all(feature = "futures_io", not(feature = "tokio_io")))]
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context, pos: SeekFrom) -> Poll<io::Result<u64>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_seek(cx, pos))
    }

    #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
    fn start_seek(
        self: Pin<&mut Self>,
        cx: &mut Context,
        position: SeekFrom,
    ) -> Poll<io::Result<()>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).start_seek(cx, position))
    }

    #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<u64>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_complete(cx))
    }
}

impl<S, HTTU, HTG, TU> AsyncBufRead for HttpClientInnerStream<S, HTTU, HTG, TU>
where
    S: AsyncRead + AsyncWrite + Unpin + AsyncBufRead,
    HTTU: TlsClientUpgrader<S> + Unpin,
    HTTU::Output: AsyncRead + AsyncWrite + Unpin + AsyncBufRead,
    HTG: HttpTunnelClientGrader<S> + Unpin,
    <HTG as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin + AsyncBufRead,
    HTG: HttpTunnelClientGrader<HTTU::Output> + Unpin + AsyncBufRead,
    <HTG as Upgrader<HTTU::Output>>::Output: AsyncRead + AsyncWrite + Unpin + AsyncBufRead,
    TU: TlsClientUpgrader<HTTU::Output> + Upgrader<S> + Unpin,
    <TU as Upgrader<S>>::Output: AsyncRead + AsyncWrite + Unpin + AsyncBufRead,
    <TU as Upgrader<<HTTU as Upgrader<S>>::Output>>::Output:
        AsyncRead + AsyncWrite + Unpin + AsyncBufRead,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<&[u8]>> {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).poll_fill_buf(cx))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        case!(self.get_mut(), ref mut inner => Pin::new(inner).consume(amt))
    }
}

//
//
//
pub struct HttpClientProxy<S, TU, HTG> {
    inner: HttpClientProxyInner<S, TU, HTG>,
}

enum HttpClientProxyInner<S, TU, HTG> {
    Http(HTG),
    Https(TU, HTG),
    // TODO, socks5
    #[allow(dead_code)]
    Never(S),
}

impl<S, TU, HTG> HttpClientProxy<S, TU, HTG>
where
    HTG: HttpTunnelClientGrader<S>,
{
    pub fn http(http_tunnel_grader: HTG) -> Self {
        Self {
            inner: HttpClientProxyInner::Http(http_tunnel_grader),
        }
    }
}

impl<S, TU, HTG> HttpClientProxy<S, TU, HTG>
where
    TU: TlsClientUpgrader<S>,
    HTG: HttpTunnelClientGrader<TU::Output>,
{
    pub fn https(tls_upgrader: TU, http_tunnel_grader: HTG) -> Self {
        Self {
            inner: HttpClientProxyInner::Https(tls_upgrader, http_tunnel_grader),
        }
    }
}
