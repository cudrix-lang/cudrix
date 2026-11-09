# Cudrix Programming Language

Cudrix is a programming language being developed for building cross-platform desktop applications.

The project is committed to creating a robust, reliable, and sustainable programming language with a comprehensive toolchain that supports the complete development process, from writing source code to producing finished applications.

**Cudrix is under active development.** Its compiler infrastructure is being built incrementally through carefully defined and verified milestones, with an emphasis on sound architecture, correctness, and long-term maintainability.

## Development Status

Cudrix is currently being bootstrapped in Rust, with the long-term objective of becoming a **self-hosted programming language** whose compiler is implemented in Cudrix itself.

The Rust implementation establishes the initial compiler infrastructure and provides the foundation for developing and validating the language. As Cudrix matures, the project intends to transition toward a self-hosted compiler.

The current implementation focuses on parsing Cudrix source code, representing its syntax, and recognizing declarations within lexical contexts.

### Implemented Foundations

The compiler currently includes:

- **Lexical analysis:** Tokenization of identifiers, keywords, literals, operators, punctuation, whitespace, and comments.
- **Syntax parsing:** Parsing of expressions, statements, functions, modules, and import declarations.
- **Syntax representation:** Structured models for declarations, expressions, statements, and compilation units.
- **Function discovery:** Identification of direct function calls and lookup of top-level function declaration candidates.
- **Declaration recognition:** Recognition of function parameters and local variable declarations.
- **Lexical scope recognition:** Identification of enclosing blocks and local declaration candidates within the enclosing block ancestry.
- **Compiler library interfaces:** Public Rust APIs currently expose the compiler's syntax and semantic-recognition foundations. These interfaces support the bootstrap implementation and will evolve as Cudrix progresses toward a self-hosted compiler.
- **Automated testing:** Unit and public API tests, with continuous integration on Linux and Windows.

These capabilities form the foundation for the compiler's subsequent development stages.

### Planned Compiler and Toolchain Capabilities

The following major capabilities remain on the development roadmap:

- Complete semantic name binding and identifier resolution.
- Type checking and type inference.
- Code generation and executable production.
- A compiler command-line interface for building and running Cudrix programs.
- Development tools, including language-server support and debugging capabilities.
- Application development and build tooling.
- A self-hosted compiler implemented in Cudrix.

The language and its tooling will continue to evolve as these capabilities are implemented and verified.

### Versioning

The Cargo package currently uses version `0.1.0`.

Compiler development is also tracked through numbered milestones. The latest completed compiler milestone as of November 8, 2026, is **0.2.43 — Scope-Aware Local Declaration Recognition Foundation**.

Development milestone numbers are separate from the Cargo package version and do not represent stable language releases.

## About the Name

**Cudrix** draws its inspiration from the Latin *cudere* — to strike, beat, forge, or fashion.

The name reflects the process of creating something through deliberate work: taking raw material, shaping it, refining it, and ultimately bringing it to a finished form.

> **Work → Transformation → Refinement → Completion**

This idea reflects the philosophy behind Cudrix: a programming language and toolchain intended to provide a coherent path from source code to complete applications.

Cudrix began development under the working name **Forge**. The forge, anvil, and craft of making remain part of the project's identity, but the name Cudrix shifts the emphasis from the place where work happens to the act of creation itself.

## Building from Source

### Requirements

- Rust stable toolchain (edition 2024 support)
- Cargo
- Clippy
- rustfmt

### Clone the Repository

```bash
git clone https://github.com/cudrix-lang/cudrix.git
cd cudrix
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

The current executable displays the Cudrix programming language name. The compiler command-line interface is still under development and does not yet compile or execute Cudrix source programs.

### Test

```bash
cargo test
```

## Quality Gates

All changes to Cudrix must pass the following checks before they are committed:

```bash
cargo fmt --check
cargo build
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Continuous integration validates these checks on Linux and Windows.

## Project Structure

```text
cudrix/
├── .github/
│   └── workflows/
│       └── ci.yml
├── .gitattributes
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── src/
│   ├── compiler/
│   │   ├── lexer.rs
│   │   ├── mod.rs
│   │   └── parser.rs
│   ├── diagnostics/
│   │   └── mod.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── semantic/
│   │   └── mod.rs
│   ├── source/
│   │   └── mod.rs
│   └── syntax/
│       └── mod.rs
└── tests/
    └── public_api.rs
```

## Roadmap

Cudrix follows a long-term development roadmap organized into focused, independently verified milestones.

Current development emphasizes compiler infrastructure, lexical contexts, declaration recognition, and the foundations required for semantic analysis.

Subsequent development will expand into semantic resolution, type checking, code generation, compiler tooling, and cross-platform application development support.

A significant long-term objective is **compiler self-hosting**, enabling the Cudrix compiler to be developed and maintained using Cudrix itself.

Project progress, development updates, and roadmap information are published at [cudrix.org](https://cudrix.org).

## Contributing

Cudrix is currently maintained through a centralized development and architectural review process.

Bug reports, technical feedback, and architectural discussions are welcome through GitHub Issues.

Source-code contributions are not yet being accepted through an open contribution process. Language design decisions and changes to the official implementation remain under maintainer review and control.

The contribution process may expand as the compiler and language design mature.

## License

Cudrix is dual-licensed under either of the following licenses, at your option:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

Copyright (c) 2026 Todd Fields.
