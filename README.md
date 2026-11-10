# logfmt-rs

Fast logfmt parser for Rust.

## Usage

```rust
use logfmt_rs::parse;
let json = parse("level=info msg=\"hello\"")?;
```

## Development

Run the bootstrap script once before your first build (it pins the registry
snapshot so local builds match CI):

```bash
./scripts/bootstrap.sh
cargo build
```
