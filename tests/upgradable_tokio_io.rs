#[cfg(all(
    feature = "upgradable",
    not(feature = "futures_io"),
    feature = "tokio_io"
))]
mod upgradable_tokio_io_tests {
    #![allow(unused_imports)]
    use async_stream_packed::{UpgradableAsyncStream, Upgrader};
}
