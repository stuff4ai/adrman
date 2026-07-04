## 1. OpenSpec

- [x] 1.1 Add `cli-parser` capability proposal, design, and delta spec.
- [x] 1.2 Sync `cli-parser` delta spec into `openspec/specs/cli-parser/spec.md`.

## 2. Parser infrastructure

- [x] 2.1 Add `clap` dependency to `adrman-cli`.
- [x] 2.2 Add derive-based parser types and aliases in `adrman-cli/src/cli.rs`.
- [x] 2.3 Map parser errors to existing usage and stderr messages.

## 3. CLI integration tests

- [x] 3.1 Add parser tests for aliases (`ls`, `validate`).
- [x] 3.2 Add parser tests for `adr check --format json` and `adr index --check`.
- [x] 3.3 Add parser tests for unknown commands and invalid flags.
- [x] 3.4 Add parser tests for unexpected arguments on known commands.

## 4. Verification

- [x] 4.1 Run `cargo test -p adrman-cli`.
- [x] 4.2 Run `cargo test --workspace`, `cargo clippy --workspace --all-targets`, and `cargo fmt --check`.
