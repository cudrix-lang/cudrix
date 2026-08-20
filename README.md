# Forge Programming Language

Forge is a programming language for building cross-platform desktop applications.

Forge is currently in early development.

## Development

Forge is being bootstrapped in Rust.

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

All changes to Forge must pass the following checks before they are committed:

```bash
cargo build
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check

```

```
forge
├─ Cargo.lock
├─ Cargo.toml
├─ README.md
├─ src
│  ├─ compiler
│  │  ├─ lexer.rs
│  │  └─ mod.rs
│  ├─ diagnostics
│  │  └─ mod.rs
│  ├─ lib.rs
│  ├─ main.rs
│  └─ source
│     └─ mod.rs
└─ tests
   └─ public_api.rs

```