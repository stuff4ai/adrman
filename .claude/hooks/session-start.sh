#!/bin/bash
set -euo pipefail

cd "$CLAUDE_PROJECT_DIR"

# rustup installs the toolchain pinned in rust-toolchain.toml on first use.
rustup show >/dev/null

cargo fetch --locked

# Warm the incremental build cache so the first fmt/clippy/test/run in the
# session doesn't pay full compile time.
cargo build --workspace --all-targets --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
