#[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))]
mod syncable_with_context_tokio_io_tests {
    #![allow(unused_imports)]
    use async_stream_packed::SyncableWithContextAsyncStream;
}
