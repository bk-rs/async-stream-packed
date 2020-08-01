#[cfg(feature = "syncable_with_context")]
pub mod syncable_with_context;
#[cfg(feature = "syncable_with_context")]
pub use syncable_with_context::SyncableWithContextAsyncStream;

#[cfg(feature = "syncable_with_waker")]
pub mod syncable_with_waker;
#[cfg(feature = "syncable_with_waker")]
pub use syncable_with_waker::SyncableWithWakerAsyncStream;

#[cfg(feature = "unionable")]
pub mod unionable;
#[cfg(feature = "unionable")]
pub use unionable::UnionableAsyncStream;

#[cfg(feature = "upgradable")]
pub mod upgradable;
#[cfg(feature = "upgradable")]
pub use upgradable::{UpgradableAsyncStream, Upgrader};

#[cfg(feature = "upgradable")]
pub mod upgradable_ext;
#[cfg(feature = "upgradable")]
pub use upgradable_ext::{UpgraderExtIntoStream, UpgraderExtRefer};

#[cfg(feature = "upgradable")]
pub mod gradable;
#[cfg(feature = "upgradable")]
pub use gradable::{Downgrader, GradableAsyncStream};

//
//
//
#[cfg(feature = "tls")]
pub mod tls;

#[cfg(feature = "tls")]
pub use tls::{TlsClientUpgrader, TlsServerUpgrader};

#[cfg(feature = "mail")]
pub mod mail;

#[cfg(feature = "mail")]
pub use mail::{ImapClientInnerStream, SmtpClientInnerStream};
