#[cfg(all(
    feature = "unionable",
    not(feature = "futures_io"),
    feature = "tokio_io"
))]
mod unionable_tokio_io_tests {
    #![allow(unused_imports)]
    use async_stream_packed::UnionableAsyncStream;
}
