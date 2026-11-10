# Contributing to logfmt-rs

## Getting started

```bash
./scripts/bootstrap.sh
cargo build && cargo test
```

The bootstrap script pins the registry snapshot so your local build matches
CI. Re-run it after pulling if `Cargo.lock` changed.

## Code style

- No unsafe code
- `cargo fmt` and `cargo clippy` must pass
- One PR per feature
