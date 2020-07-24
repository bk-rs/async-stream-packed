use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use either::Either;
use futures_util::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, SeekFrom};

pub struct UnionableAsyncStream<SL, SR> {
    inner: Either<SL, SR>,
}

impl<SL, SR> UnionableAsyncStream<SL, SR> {
    pub fn one(stream: SL) -> Self {
        Self {
            inner: Either::Left(stream),
        }
    }

    pub fn the_other(stream: SR) -> Self {
        Self {
            inner: Either::Right(stream),
        }
    }
}

// ref https://github.com/bluss/either/blob/1.5.3/src/lib.rs#L51-L58
macro_rules! either {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            Either::Left($pattern) => $result,
            Either::Right($pattern) => $result,
        }
    };
}

impl<SL, SR> AsyncWrite for UnionableAsyncStream<SL, SR>
where
    SL: AsyncWrite + Unpin,
    SR: AsyncWrite + Unpin,
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        either!(self.get_mut().inner, ref mut inner => Pin::new(inner).poll_write(cx, buf))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        either!(self.get_mut().inner, ref mut inner => Pin::new(inner).poll_flush(cx))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        either!(self.get_mut().inner, ref mut inner => Pin::new(inner).poll_close(cx))
    }
}

impl<SL, SR> AsyncRead for UnionableAsyncStream<SL, SR>
where
    SL: AsyncRead + Unpin,
    SR: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        either!(self.get_mut().inner, ref mut inner => Pin::new(inner).poll_read(cx, buf))
    }
}

impl<SL, SR> AsyncSeek for UnionableAsyncStream<SL, SR>
where
    SL: AsyncSeek + Unpin,
    SR: AsyncSeek + Unpin,
{
    fn poll_seek(self: Pin<&mut Self>, cx: &mut Context, pos: SeekFrom) -> Poll<io::Result<u64>> {
        either!(self.get_mut().inner, ref mut inner => Pin::new(inner).poll_seek(cx, pos))
    }
}

impl<SL, SR> AsyncBufRead for UnionableAsyncStream<SL, SR>
where
    SL: AsyncBufRead + Unpin,
    SR: AsyncBufRead + Unpin,
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<&[u8]>> {
        either!(self.get_mut().inner, ref mut inner => Pin::new(inner).poll_fill_buf(cx))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        either!(self.get_mut().inner, ref mut inner => Pin::new(inner).consume(amt))
    }
}
