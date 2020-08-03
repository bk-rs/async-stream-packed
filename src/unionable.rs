use std::ops::{Deref, DerefMut};

use futures_io::{AsyncRead, AsyncWrite};
use futures_util::future::Either;

pub struct UnionableAsyncStream<SL, SR> {
    inner: Either<SL, SR>,
}

impl<SL, SR> Deref for UnionableAsyncStream<SL, SR> {
    type Target = Either<SL, SR>;

    fn deref(&self) -> &Either<SL, SR> {
        &self.inner
    }
}

impl<SL, SR> DerefMut for UnionableAsyncStream<SL, SR> {
    fn deref_mut(&mut self) -> &mut Either<SL, SR> {
        &mut self.inner
    }
}

impl<SL, SR> UnionableAsyncStream<SL, SR>
where
    SL: AsyncRead + AsyncWrite + Unpin,
    SR: AsyncRead + AsyncWrite + Unpin,
{
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
