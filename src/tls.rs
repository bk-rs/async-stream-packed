#[cfg(feature = "upgradable")]
mod upgradable_ {
    use crate::upgradable::Upgrader;

    pub trait TlsClientUpgrader<S>: Upgrader<S> {}

    pub trait TlsServerUpgrader<S>: Upgrader<S> {}
}

#[cfg(feature = "upgradable")]
pub use upgradable_::*;
