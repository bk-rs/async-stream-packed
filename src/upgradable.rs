use std::io;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures_io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, SeekFrom};

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
    Upgraded(SU::Output, Option<SU>),
    None,
}

#[async_trait]
pub trait Upgrader<S> {
    type Output;
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output>;
    fn upgrade_required(&self) -> bool {
        true
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

    pub fn with_upgraded_stream(stream: SU::Output) -> Self {
        Self {
            inner: Inner::Upgraded(stream, None),
        }
    }

    pub fn with_upgraded_stream_and_grader(stream: SU::Output, grader: SU) -> Self {
        Self {
            inner: Inner::Upgraded(stream, Some(grader)),
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
                self.inner = Inner::Upgraded(stream, Some(upgrader));
                Ok(())
            }
            Inner::Upgraded(_, _) => Err(io::Error::new(io::ErrorKind::Other, "not allow")),
            Inner::None => panic!("never"),
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

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_close(cx),
            Inner::Upgraded(s, _) => Pin::new(s).poll_close(cx),
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
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context, pos: SeekFrom) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        let inner = &mut this.inner;

        match inner {
            Inner::Pending(s, _) => Pin::new(s).poll_seek(cx, pos),
            Inner::Upgraded(s, _) => Pin::new(s).poll_seek(cx, pos),
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
