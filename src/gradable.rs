use std::io;
use std::mem;

use async_trait::async_trait;

use crate::upgradable::{Inner, UpgradableAsyncStream, Upgrader};

pub type GradableAsyncStream<S, SU> = UpgradableAsyncStream<S, SU>;

#[async_trait]
pub trait Downgrader<S>: Upgrader<S> {
    async fn downgrade(&mut self, output: Self::Output) -> io::Result<S>;
    fn downgrade_required(&self) -> bool {
        true
    }
}

#[async_trait]
impl<S> Downgrader<S> for ()
where
    S: Send + 'static,
{
    async fn downgrade(&mut self, _: <Self as Upgrader<S>>::Output) -> io::Result<S> {
        unreachable!()
    }
    fn downgrade_required(&self) -> bool {
        false
    }
}

impl<S, SU> GradableAsyncStream<S, SU>
where
    SU: Upgrader<S> + Downgrader<S>,
{
    pub fn downgrade_required(&self) -> bool {
        match &self.inner {
            Inner::Pending(_, _) => false,
            Inner::Upgraded(_, grader) => grader.downgrade_required(),
            Inner::None => panic!("never"),
        }
    }

    pub async fn downgrade(&mut self) -> io::Result<()> {
        match mem::replace(&mut self.inner, Inner::None) {
            Inner::Pending(_, _) => Err(io::Error::new(io::ErrorKind::Other, "not allow")),
            Inner::Upgraded(stream, mut grader) => {
                if !grader.downgrade_required() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "downgrade not required",
                    ));
                }

                let stream = grader.downgrade(stream).await?;
                self.inner = Inner::Pending(stream, grader);
                Ok(())
            }
            Inner::None => panic!("never"),
        }
    }
}
