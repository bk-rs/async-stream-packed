# async-stream-packed

* [Cargo package](https://crates.io/crates/async-stream-packed)

## Examples

### smol 

* [unionable](demos/smol/src/unionable.rs)
* [async-tls client upgrader](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/smol/src/async_tls_client.rs)
* [async-native-tls client upgrader](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/smol/src/async_native_tls_client.rs)
* [imap client](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/smol/src/imap_client.rs)
* [smtp client](https://github.com/bk-rs/async-stream-tls-upgrader/blob/master/demos/smol/src/smtp_client.rs)

## Dev

```
cargo test --all-features --all -- --nocapture && \
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features
cargo test-all-features --all
```

```
cargo tarpaulin --all-features
```
