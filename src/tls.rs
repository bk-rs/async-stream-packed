#[cfg(feature = "upgradable")]
mod upgradable_ {
    use crate::upgradable::Upgrader;

    pub trait TlsClientUpgrader<S>: Upgrader<S> {}

    impl<S> TlsClientUpgrader<S> for () where S: Send + 'static {}

    pub trait TlsServerUpgrader<S>: Upgrader<S> {}

    impl<S> TlsServerUpgrader<S> for () where S: Send + 'static {}
}

#[cfg(feature = "upgradable")]
pub use upgradable_::*;
