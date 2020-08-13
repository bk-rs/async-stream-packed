# async-stream-packed

* [Cargo package](https://crates.io/crates/async-stream-packed)

## Examples

### async-net

* [unionable](demos/async-net/src/unionable.rs)
* [async-tls client upgrader](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/async-net/src/async_tls_client.rs)
* [async-native-tls client upgrader](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/async-net/src/async_native_tls_client.rs)
* [imap client](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/async-net/src/imap_client.rs)
* [smtp client](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/async-net/src/smtp_client.rs)

### tokio

* [unionable](demos/tokio/src/unionable.rs)

## Dev

```
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features
cargo test-all-features
```

```
cargo tarpaulin --all-features
```
