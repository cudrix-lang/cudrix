# Cudrix Programming Language

Cudrix is a programming language for building cross-platform desktop applications.

Cudrix is currently in early development.

## Development

Cudrix is being bootstrapped in Rust.

### Requirements

- Rust stable toolchain
- Cargo
- Clippy
- rustfmt

### Build

```bash
cargo build

```

### Test

```bash

cargo test

```

## Quality Gates

All changes to Cudrix must pass the following checks before they are committed:

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

```
```
cudrix
├─ Cargo.lock
├─ Cargo.toml
├─ README.md
├─ src
│  ├─ compiler
│  │  ├─ lexer.rs
│  │  ├─ mod.rs
│  │  └─ parser.rs
│  ├─ diagnostics
│  │  └─ mod.rs
│  ├─ lib.rs
│  ├─ main.rs
│  └─ source
│     └─ mod.rs
└─ tests
   └─ public_api.rs

```