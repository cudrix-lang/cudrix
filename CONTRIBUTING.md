# Contributing to Cudrix

Thank you for your interest in the Cudrix Programming Language.

Cudrix is a long-term programming language development project committed to building a robust, reliable, and maintainable compiler and cross-platform development toolchain.

Community interest, constructive feedback, and technical discussion are welcome.

## Current Contribution Policy

Cudrix is currently developed through a centralized architectural review and implementation process.

At this stage, we welcome:

- Bug reports and reproducible defects.
- Technical feedback on existing compiler functionality.
- Suggestions for language features and developer tooling.
- Architectural proposals and design discussions.
- Documentation feedback and corrections.

**Unsolicited source-code pull requests are not currently being accepted.**

Please open a GitHub Issue to discuss a proposed change before undertaking implementation work.

The contribution process may expand as the language and compiler mature.

## Reporting Bugs

When reporting an issue, please include:

- A clear description of the problem.
- Steps to reproduce the behavior.
- The expected and actual results.
- Relevant Cudrix source examples, if applicable.
- The applicable commit or development milestone.
- Relevant operating system and toolchain information.

Please avoid including credentials, private information, or unrelated system details in public reports.

## Feature Requests and Architectural Discussions

Cudrix development emphasizes correctness, deterministic behavior, coherent language design, and long-term maintainability.

When proposing a feature or architectural change, please explain:

- The problem or requirement being addressed.
- The proposed behavior or design.
- The expected benefits.
- Potential compatibility or architectural implications.
- Relevant alternatives or tradeoffs.

Proposals are evaluated against the project's overall direction and development priorities.

Discussion or acceptance of an idea does not automatically authorize implementation or guarantee its inclusion in the language.

## Development Standards

Changes to the official Cudrix implementation follow an established development process:

1. Define a focused development milestone or approved change.
2. Review architectural implications before implementation.
3. Implement changes on a dedicated feature branch.
4. Preserve existing behavior unless changes are explicitly approved.
5. Include appropriate unit tests and public API coverage.
6. Run all established quality checks.
7. Review and commit the verified implementation.
8. Integrate approved changes into `main` through a controlled merge process.

The current quality checks are:

```bash
cargo fmt --check
cargo build
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Project Governance

The Cudrix maintainer retains responsibility for:

- Language specifications and design decisions.
- Compiler architecture and implementation direction.
- Development priorities and milestone acceptance.
- Review and approval of changes to the official repository.
- Release planning and project governance.

Technical feedback and constructive discussion are encouraged, but final decisions remain with the project maintainer.

## Licensing

Cudrix is dual-licensed under the MIT License or Apache License 2.0, at your option.

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for the applicable terms.

Any future process for accepting external code contributions will include clear licensing and contribution requirements.

## Project Information

For project updates and the development roadmap, visit [cudrix.org](https://cudrix.org).
