use std::io::{self, BufRead, Read, Seek, Write};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};

pub struct SyncableWithContextAsyncStream<'a, 'b, S> {
    inner: S,
    cx: &'a mut Context<'b>,
}

impl<'a, 'b, S> SyncableWithContextAsyncStream<'a, 'b, S> {
    pub fn new(inner: S, cx: &'a mut Context<'b>) -> Self {
        Self { inner, cx }
    }

    pub fn get_ref(&self) -> &S {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<'a, 'b, S> Write for SyncableWithContextAsyncStream<'a, 'b, S>
where
    S: AsyncWrite + Unpin,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match Pin::new(&mut self.inner).poll_write(self.cx, buf) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match Pin::new(&mut self.inner).poll_flush(self.cx) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<'a, 'b, S> Read for SyncableWithContextAsyncStream<'a, 'b, S>
where
    S: AsyncRead + Unpin,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match Pin::new(&mut self.inner).poll_read(self.cx, buf) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<'a, 'b, S> Seek for SyncableWithContextAsyncStream<'a, 'b, S>
where
    S: AsyncSeek + Unpin,
{
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match Pin::new(&mut self.inner).poll_seek(self.cx, pos) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<'a, 'b, S> BufRead for SyncableWithContextAsyncStream<'a, 'b, S>
where
    S: AsyncBufRead + Unpin,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match Pin::new(&mut self.inner).poll_fill_buf(self.cx) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }

    fn consume(&mut self, amt: usize) {
        Pin::new(&mut self.inner).consume(amt)
    }
}
