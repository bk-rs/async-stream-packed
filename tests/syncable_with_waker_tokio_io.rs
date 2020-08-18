#[cfg(all(
    feature = "syncable_with_waker",
    not(feature = "futures_io"),
    feature = "tokio_io"
))]
mod syncable_with_waker_tokio_io_tests {
    #![allow(unused_imports)]
    use async_stream_packed::{syncable_with_waker::WakerKind, SyncableWithWakerAsyncStream};
}
