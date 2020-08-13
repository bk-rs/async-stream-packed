#[cfg(all(
    feature = "upgradable",
    feature = "futures_io",
    not(feature = "tokio_io")
))]
mod http_tunnel_futures_io_tests {
    #![allow(unused_imports)]
    use async_stream_packed::HttpTunnelClientGrader;
}
