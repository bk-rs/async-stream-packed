# async-stream-packed

* [Cargo package](https://crates.io/crates/async-stream-packed)

## Examples

### smol 

* [unionable](demos/smol/src/unionable.rs)

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
