# Cudrix Programming Language

Cudrix is a programming language for building cross-platform desktop applications.

Cudrix is currently in early development.

## About the Name

**Cudrix** draws its inspiration from the Latin *cudere* — to strike, beat, forge, or fashion.

The name reflects the process of creating something through deliberate work: taking raw material, shaping it, refining it, and ultimately bringing it to a finished form.

> **Work → Transformation → Refinement → Completion**

This idea reflects the philosophy behind Cudrix: a programming language and toolchain intended to provide a coherent path from source code to complete applications.

Cudrix began development under the working name **Forge**. The forge, anvil, and craft of making remain part of the project's identity, but the name Cudrix shifts the emphasis from the place where work happens to the act of creation itself.

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