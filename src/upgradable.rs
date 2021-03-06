use std::io::{self, SeekFrom};
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures_x_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

pub struct UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S>,
{
    pub(crate) inner: Inner<S, SU>,
}

pub(crate) enum Inner<S, SU>
where
    SU: Upgrader<S>,
{
    Pending(S, SU),
    Upgraded(SU::Output, SU),
    None,
}

#[async_trait]
pub trait Upgrader<S> {
    type Output: AsyncRead + AsyncWrite + Unpin;
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output>;
    fn upgrade_required(&self) -> bool {
        true
    }
}

#[async_trait]
impl<S> Upgrader<S> for ()
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = S;
    async fn upgrade(&mut self, _: S) -> io::Result<Self::Output> {
        unreachable!()
    }
    fn upgrade_required(&self) -> bool {
        false
    }
}

impl<S, SU> UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S>,
{
    pub fn new(stream: S, upgrader: SU) -> Self {
        Self {
            inner: Inner::Pending(stream, upgrader),
        }
    }

    pub fn with_upgraded_stream_and_upgrader(stream: SU::Output, upgrader: SU) -> Self {
        Self {
            inner: Inner::Upgraded(stream, upgrader),
        }
    }

    pub fn is_upgraded(&self) -> bool {
        match &self.inner {
            Inner::Upgraded(_, _) => true,
            _ => false,
        }
    }

    pub fn upgrade_required(&self) -> bool {
        match &self.inner {
            Inner::Pending(_, upgrader) => upgrader.upgrade_required(),
            Inner::Upgraded(_, _) => false,
            Inner::None => panic!("never"),
        }
    }

    pub async fn upgrade(&mut self) -> io::Result<()> {
        match mem::replace(&mut self.inner, Inner::None) {
            Inner::Pending(stream, mut upgrader) => {
                if !upgrader.upgrade_required() {
                    return Err(io::Error::new(io::ErrorKind::Other, "upgrade not required"));
                }
                let stream = upgrader.upgrade(stream).await?;
                self.inner = Inner::Upgraded(stream, upgrader);
                Ok(())
            }
            Inner::Upgraded(_, _) => Err(io::Error::new(io::ErrorKind::Other, "not allow")),
            Inner::None => panic!("never"),
        }
    }
}

impl<S> UpgradableAsyncStream<S, ()>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    pub fn with_stream(stream: S) -> Self {
        Self {
            inner: Inner::Pending(stream, ()),
        }
    }

    pub fn with_upgraded_stream(stream: S) -> Self {
        Self {
            inner: Inner::Upgraded(stream, ()),
        }
    }
}

impl<S, SU> AsyncWrite for UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S> + Unpin,
    S: AsyncWrite + Unpin,
    SU::Output: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_write(cx, buf),
            Inner::Upgraded(s, _) => Pin::new(s).poll_write(cx, buf),
            Inner::None => panic!("never"),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_flush(cx),
            Inner::Upgraded(s, _) => Pin::new(s).poll_flush(cx),
            Inner::None => panic!("never"),
        }
    }

    #[cfg(all(feature = "futures_io", not(feature = "tokio_io")))]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        let inner = &mut this.inner;
        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_close(cx),
            Inner::Upgraded(s, _) => Pin::new(s).poll_close(cx),
            Inner::None => panic!("never"),
        }
    }

    #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        let inner = &mut this.inner;
        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_shutdown(cx),
            Inner::Upgraded(s, _) => Pin::new(s).poll_shutdown(cx),
            Inner::None => panic!("never"),
        }
    }
}

impl<S, SU> AsyncRead for UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S> + Unpin,
    S: AsyncRead + Unpin,
    SU::Output: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_read(cx, buf),
            Inner::Upgraded(s, _) => Pin::new(s).poll_read(cx, buf),
            Inner::None => panic!("never"),
        }
    }
}

impl<S, SU> AsyncSeek for UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S> + Unpin,
    S: AsyncSeek + Unpin,
    SU::Output: AsyncSeek + Unpin,
{
    #[cfg(all(feature = "futures_io", not(feature = "tokio_io")))]
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context, pos: SeekFrom) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_seek(cx, pos),
            Inner::Upgraded(s, _) => Pin::new(s).poll_seek(cx, pos),
            Inner::None => panic!("never"),
        }
    }

    #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
    fn start_seek(
        self: Pin<&mut Self>,
        cx: &mut Context,
        position: SeekFrom,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).start_seek(cx, position),
            Inner::Upgraded(s, _) => Pin::new(s).start_seek(cx, position),
            Inner::None => panic!("never"),
        }
    }

    #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_complete(cx),
            Inner::Upgraded(s, _) => Pin::new(s).poll_complete(cx),
            Inner::None => panic!("never"),
        }
    }
}

impl<S, SU> AsyncBufRead for UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S> + Unpin,
    S: AsyncBufRead + Unpin,
    SU::Output: AsyncBufRead + Unpin,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<&[u8]>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_fill_buf(cx),
            Inner::Upgraded(s, _) => Pin::new(s).poll_fill_buf(cx),
            Inner::None => panic!("never"),
        }
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).consume(amt),
            Inner::Upgraded(s, _) => Pin::new(s).consume(amt),
            Inner::None => panic!("never"),
        }
    }
}
