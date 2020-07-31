/*
Ref https://github.com/sdroege/async-tungstenite/blob/0.7.1/src/compat.rs
*/

use std::io::{self, Read, Seek, Write};
use std::pin::Pin;
use std::sync::Arc;

use futures_io::{AsyncRead, AsyncSeek, AsyncWrite};
use futures_util::task::{waker_ref, ArcWake, AtomicWaker, Context, Poll, Waker};

pub struct SyncableWithWakerAsyncStream<S> {
    inner: S,
    read_waker: Arc<WakerInner>,
    write_waker: Arc<WakerInner>,
}

#[derive(Default)]
struct WakerInner {
    waker: AtomicWaker,
}

impl ArcWake for WakerInner {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.waker.wake();
    }
}

pub enum WakerKind {
    Read,
    Write,
}

impl<S> SyncableWithWakerAsyncStream<S> {
    pub fn new(inner: S, waker: &Waker) -> Self {
        let this = Self {
            inner,
            read_waker: Default::default(),
            write_waker: Default::default(),
        };

        this.read_waker.waker.register(waker);
        this.write_waker.waker.register(waker);

        this
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

    pub fn set_waker(&self, waker: &Waker) {
        self.read_waker.waker.register(waker);
        self.write_waker.waker.register(waker);
    }

    pub fn set_waker_with_kind(&self, waker: &Waker, kind: WakerKind) {
        match kind {
            WakerKind::Read => self.read_waker.waker.register(waker),
            WakerKind::Write => self.write_waker.waker.register(waker),
        }
    }
}

impl<S> SyncableWithWakerAsyncStream<S>
where
    S: Unpin,
{
    fn with_context<F, T>(&mut self, kind: WakerKind, f: F) -> Poll<io::Result<T>>
    where
        F: FnOnce(&mut Context, Pin<&mut S>) -> Poll<io::Result<T>>,
    {
        let waker = match kind {
            WakerKind::Read => waker_ref(&self.read_waker),
            WakerKind::Write => waker_ref(&self.write_waker),
        };
        let mut context = Context::from_waker(&waker);
        f(&mut context, Pin::new(&mut self.inner))
    }
}

impl<S> Write for SyncableWithWakerAsyncStream<S>
where
    S: AsyncWrite + Unpin,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.with_context(WakerKind::Write, |cx, stream| stream.poll_write(cx, buf)) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.with_context(WakerKind::Write, |cx, stream| stream.poll_flush(cx)) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<S> Read for SyncableWithWakerAsyncStream<S>
where
    S: AsyncRead + Unpin,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.with_context(WakerKind::Read, |cx, stream| stream.poll_read(cx, buf)) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

impl<S> Seek for SyncableWithWakerAsyncStream<S>
where
    S: AsyncSeek + Unpin,
{
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match self.with_context(WakerKind::Read, |cx, stream| stream.poll_seek(cx, pos)) {
            Poll::Ready(ret) => ret,
            Poll::Pending => Err(io::ErrorKind::WouldBlock.into()),
        }
    }
}

// lifetime problem with BufRead
