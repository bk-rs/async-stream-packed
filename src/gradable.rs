use std::io;

use async_trait::async_trait;

use crate::upgradable::{Inner, UpgradableAsyncStream, Upgrader};

pub type GradableAsyncStream<S, SU> = UpgradableAsyncStream<S, SU>;

#[async_trait]
pub trait Downgrader<S>: Upgrader<S> {
    fn get_ref(output: &Self::Output) -> &S;
    fn get_mut(output: &mut Self::Output) -> &mut S;
    fn into_inner(output: Self::Output) -> io::Result<Option<S>>;
}

impl<S, SU> GradableAsyncStream<S, SU>
where
    SU: Upgrader<S> + Downgrader<S>,
{
    pub fn get_ref(&self) -> &S {
        match &self.inner {
            Inner::Pending((s, _)) => &s,
            Inner::Upgraded(s) => SU::get_ref(s),
            Inner::None => panic!("never"),
        }
    }

    pub fn get_mut(&mut self) -> &mut S {
        match &mut self.inner {
            Inner::Pending((s, _)) => s,
            Inner::Upgraded(s) => SU::get_mut(s),
            Inner::None => panic!("never"),
        }
    }

    pub fn into_inner(self) -> io::Result<Option<S>> {
        match self.inner {
            Inner::Pending((s, _)) => Ok(Some(s)),
            Inner::Upgraded(s) => SU::into_inner(s),
            Inner::None => panic!("never"),
        }
    }
}
