use std::io;
use std::mem;

use async_trait::async_trait;

use crate::upgradable::{Inner, UpgradableAsyncStream, Upgrader};

pub type GradableAsyncStream<S, SU> = UpgradableAsyncStream<S, SU>;

#[async_trait]
pub trait Downgrader<S, SU>: Upgrader<S> {
    async fn downgrade(output: Self::Output, upgrader: Option<SU>) -> io::Result<(S, Option<SU>)>;
}

impl<S, SU> GradableAsyncStream<S, SU>
where
    SU: Upgrader<S> + Downgrader<S, SU>,
{
    pub async fn downgrade(&mut self) -> io::Result<()> {
        match mem::replace(&mut self.inner, Inner::None) {
            Inner::Pending((_, _)) => Err(io::Error::new(
                io::ErrorKind::Other,
                "do upgrade first or don't downgrade agent",
            )),
            Inner::Upgraded((stream, upgrader)) => {
                let (stream, upgrader) = SU::downgrade(stream, upgrader).await?;
                self.inner = Inner::Pending((stream, upgrader));
                Ok(())
            }
            Inner::None => panic!("never"),
        }
    }
}
