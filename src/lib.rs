cfg_if::cfg_if! {
    if #[cfg(all(feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod syncable_with_context;
        pub use syncable_with_context::SyncableWithContextAsyncStream;
    } else if #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod syncable_with_context;
        pub use syncable_with_context::SyncableWithContextAsyncStream;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "syncable_with_waker", feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod syncable_with_waker;
        pub use syncable_with_waker::SyncableWithWakerAsyncStream;
    } else if #[cfg(all(feature = "syncable_with_waker", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod syncable_with_waker;
        pub use syncable_with_waker::SyncableWithWakerAsyncStream;
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

        //
        pub mod tls;
        pub use tls::{TlsClientUpgrader, TlsServerUpgrader};

        pub mod http_tunnel;
        pub use http_tunnel::HttpTunnelClientGrader;

        pub mod http;
        pub use http::{HttpClientInnerStream, HttpClientProxy};

        pub mod imap;
        pub use imap::ImapClientInnerStream;

        pub mod smtp;
        pub use smtp::SmtpClientInnerStream;
    } else if #[cfg(all(feature = "upgradable", not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod upgradable;
        pub use upgradable::{UpgradableAsyncStream, Upgrader};

        pub mod upgradable_ext;
        pub use upgradable_ext::{UpgraderExtIntoStream, UpgraderExtRefer};

        pub mod gradable;
        pub use gradable::{Downgrader, GradableAsyncStream};

        //
        pub mod tls;
        pub use tls::{TlsClientUpgrader, TlsServerUpgrader};

        pub mod http_tunnel;
        pub use http_tunnel::HttpTunnelClientGrader;

        pub mod http;
        pub use http::{HttpClientInnerStream, HttpClientProxy};

        pub mod imap;
        pub use imap::ImapClientInnerStream;

        pub mod smtp;
        pub use smtp::SmtpClientInnerStream;
    }
}
