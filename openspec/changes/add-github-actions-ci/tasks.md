## 1. OpenSpec artifacts

- [x] 1.1 Create proposal, design, and spec delta for `github-actions-ci`.
- [x] 1.2 Validate the OpenSpec change.

## 2. GitHub Actions workflow

- [x] 2.1 Add `.github/workflows/ci.yml` with triggers, permissions, concurrency, timeout, and Rust checks.
- [x] 2.2 Use `actions/checkout@v7` and install Rust via `rustup show` from `rust-toolchain.toml`.

## 3. Dependabot configuration

- [x] 3.1 Add `.github/dependabot.yml` for weekly GitHub Actions and Cargo updates with open-PR limits.

## 4. Verification

- [x] 4.1 Run local `cargo fmt --check`, `cargo clippy`, and `cargo test` to confirm CI commands pass.
