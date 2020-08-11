use std::io;

use futures_io::{AsyncRead, AsyncWrite};

use crate::upgradable::{Inner, UpgradableAsyncStream, Upgrader};

//
//
//
pub trait UpgraderExtRefer<S>: Upgrader<S> {
    fn get_ref(output: &Self::Output) -> &S;
    fn get_mut(output: &mut Self::Output) -> &mut S;
}

impl<S> UpgraderExtRefer<S> for ()
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    fn get_ref(output: &<Self as Upgrader<S>>::Output) -> &S {
        output
    }
    fn get_mut(output: &mut <Self as Upgrader<S>>::Output) -> &mut S {
        output
    }
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
pub trait UpgraderExtIntoStream<S>: Upgrader<S> {
    fn into_stream(output: Self::Output) -> io::Result<S>;
}

impl<S> UpgraderExtIntoStream<S> for ()
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    fn into_stream(output: <Self as Upgrader<S>>::Output) -> io::Result<S> {
        Ok(output)
    }
}

impl<S, SU> UpgradableAsyncStream<S, SU>
where
    SU: UpgraderExtIntoStream<S>,
{
    pub fn into_stream(self) -> io::Result<S> {
        match self.inner {
            Inner::Pending(s, _) => Ok(s),
            Inner::Upgraded(s, _) => SU::into_stream(s),
            Inner::None => panic!("never"),
        }
    }
}

//
//
//
impl<S, SU> UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S>,
{
    pub fn try_into_stream(self) -> io::Result<S> {
        match self.inner {
            Inner::Pending(s, _) => Ok(s),
            Inner::Upgraded(_, _) => Err(io::Error::new(io::ErrorKind::Other, "unimplemented")),
            Inner::None => panic!("never"),
        }
    }
}

impl<S, SU> UpgradableAsyncStream<S, SU>
where
    SU: Upgrader<S>,
{
    pub fn try_into_upgraded_stream(self) -> io::Result<SU::Output> {
        match self.inner {
            Inner::Pending(_, _) => Err(io::Error::new(io::ErrorKind::Other, "unimplemented")),
            Inner::Upgraded(s, _) => Ok(s),
            Inner::None => panic!("never"),
        }
    }
}
