#[cfg(feature = "upgradable")]
mod upgradable_ {
    use crate::upgradable::Upgrader;

    pub trait TlsClientUpgrader<S>: Upgrader<S> {}

    pub trait TlsServerUpgrader<S>: Upgrader<S> {}
}

#[cfg(feature = "upgradable")]
pub use upgradable_::*;

#[cfg(feature = "gradable")]
mod gradable_ {
    use crate::gradable::Downgrader;
    use crate::tls::{TlsClientUpgrader, TlsServerUpgrader};

    pub trait TlsClientDowngrader<S>: Downgrader<S> + TlsClientUpgrader<S> {}

    pub trait TlsServerDowngrader<S>: Downgrader<S> + TlsServerUpgrader<S> {}
}

#[cfg(feature = "gradable")]
pub use gradable_::*;
