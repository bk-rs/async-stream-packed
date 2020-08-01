use std::io;

use crate::upgradable::{Inner, UpgradableAsyncStream, Upgrader};

//
//
//
pub trait UpgraderExtRefer<S>: Upgrader<S> {
    fn get_ref(output: &Self::Output) -> &S;
    fn get_mut(output: &mut Self::Output) -> &mut S;
}

impl<S, SU> UpgradableAsyncStream<S, SU>
where
    SU: UpgraderExtRefer<S>,
{
    pub fn get_ref(&self) -> &S {
        match &self.inner {
            Inner::Pending(s, _) => &s,
            Inner::Upgraded(s, _) => SU::get_ref(s),
            Inner::None => panic!("never"),
        }
    }

    pub fn get_mut(&mut self) -> &mut S {
        match &mut self.inner {
            Inner::Pending(s, _) => s,
            Inner::Upgraded(s, _) => SU::get_mut(s),
            Inner::None => panic!("never"),
        }
    }
}

//
//
//
pub trait UpgraderExtTryIntoS<S>: Upgrader<S> {
    fn try_into_s(output: Self::Output) -> io::Result<S>;
}

impl<S, SU> UpgradableAsyncStream<S, SU>
where
    SU: UpgraderExtTryIntoS<S>,
{
    pub fn try_into_s(self) -> io::Result<S> {
        match self.inner {
            Inner::Pending(s, _) => Ok(s),
            Inner::Upgraded(s, _) => SU::try_into_s(s),
            Inner::None => panic!("never"),
        }
    }
}
