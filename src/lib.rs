cfg_if::cfg_if! {
    if #[cfg(all(feature = "syncable_with_context", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod syncable_with_context;
        pub use syncable_with_context::SyncableWithContextAsyncStream;
    } else if #[cfg(all(feature = "syncable_with_context", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod syncable_with_context;
        pub use syncable_with_context::SyncableWithContextAsyncStream;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "syncable_with_waker", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod syncable_with_waker;
        pub use syncable_with_waker::SyncableWithWakerAsyncStream;
    } else if #[cfg(all(feature = "syncable_with_waker", not(feature = "futures_io"), feature = "tokio_io"))] {
        // Not support
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "unionable", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod unionable;
        pub use unionable::UnionableAsyncStream;
    } else if #[cfg(all(feature = "unionable", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod unionable;
        pub use unionable::UnionableAsyncStream;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "upgradable", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod upgradable;
        pub use upgradable::{UpgradableAsyncStream, Upgrader};

        pub mod upgradable_ext;
        pub use upgradable_ext::{UpgraderExtIntoStream, UpgraderExtRefer};

        pub mod gradable;
        pub use gradable::{Downgrader, GradableAsyncStream};
    } else if #[cfg(all(feature = "upgradable", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod upgradable;
        pub use upgradable::{UpgradableAsyncStream, Upgrader};

        pub mod upgradable_ext;
        pub use upgradable_ext::{UpgraderExtIntoStream, UpgraderExtRefer};

        pub mod gradable;
        pub use gradable::{Downgrader, GradableAsyncStream};
    }
}

//
//
//
cfg_if::cfg_if! {
    if #[cfg(all(feature = "tls", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod tls;
        pub use tls::{TlsClientUpgrader, TlsServerUpgrader};
    } else if #[cfg(all(feature = "tls", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod tls;
        pub use tls::{TlsClientUpgrader, TlsServerUpgrader};
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "http", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod http;
        pub use http::{HttpClientInnerStream, HttpClientProxy, HttpTunnelClientGrader};
    } else if #[cfg(all(feature = "http", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod http;
        pub use http::{HttpClientInnerStream, HttpClientProxy, HttpTunnelClientGrader};
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "mail", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod mail;
        pub use mail::{ImapClientInnerStream, SmtpClientInnerStream};
    } else if #[cfg(all(feature = "mail", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod mail;
        pub use mail::{ImapClientInnerStream, SmtpClientInnerStream};
    }
}
