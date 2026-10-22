# Crypto Seed Generator

Rust CLI and library generating two valid 12-word BIP39 phrases whose concatenations
in both orders are valid 24-word phrases.

- `src/main.rs`: CLI, parallel search, worker lifecycle, and search tests.
- `src/mnemonics/`: mnemonic traits and BIP39 implementation/tests.
- `src/random.rs`: composite entropy source; preserve its security properties.
- `src/seed.rs`: seed byte container.
- `README.md`: installation and user-facing usage.
- `PROJECT_DECISIONS.md`: consult before changing parallel search or test scope.

Checks: `cargo fmt --check`, `cargo test --locked`, `cargo clippy --locked --all-targets`.
Scoped workers require Rust 1.63 or newer. Keep changes focused and never log real
seed phrases or credentials. Tests use public example phrases only.
