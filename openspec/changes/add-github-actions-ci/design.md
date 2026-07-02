## Context

adrman is a Rust Cargo workspace (`adrman-cli`, `adrman-core`) with a pinned toolchain in `rust-toolchain.toml` (stable channel, `clippy`, `rustfmt`, `rust-src`). The repository uses GitHub Flow but currently has no `.github/` automation. Contributors and agents rely on local `cargo` commands documented in `AGENTS.md`.

## Goals / Non-Goals

**Goals:**
- Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, and `cargo test --workspace --all-features --locked` on every PR to `main`, every push to `main`, and on demand.
- Use one `ubuntu-latest` job with minimal permissions and a short timeout.
- Cancel superseded workflow runs for the same workflow and ref.
- Keep Actions and Cargo dependencies current via weekly Dependabot with limited open PRs.
- Use current stable third-party Action versions and respect `rust-toolchain.toml`.

**Non-Goals:**
- Multi-OS matrix builds, caching layers, artifact uploads, coverage, or release workflows.

## Decisions

1. Single job named `ci` on `ubuntu-latest`.
   - Rationale: cheapest useful signal for a small CLI workspace; matches project scope.
   - Alternative considered: split fmt/clippy/test into parallel jobs. Rejected because startup overhead dominates for this repo size.

2. Use `actions/checkout@v7` and install Rust via rustup using `rust-toolchain.toml`.
   - Rationale: `rustup show` installs and activates the toolchain declared in the repository file (channel, profile, components) without duplicating values in the workflow.
   - Alternative considered: `dtolnay/rust-toolchain@stable`. Rejected because `@stable` selects the stable channel from the action revision and does not guarantee the file's component/profile set.

3. Workflow permissions `contents: read` only.
   - Rationale: CI only needs to read the repository; no packages, deployments, or write access required.

4. Concurrency group `${{ github.workflow }}-${{ github.ref }}` with `cancel-in-progress: true`.
   - Rationale: stops stale PR push runs quickly and saves runner minutes.

5. Job timeout `15` minutes.
   - Rationale: short enough to fail fast on hangs; ample for current workspace test suite.

6. Dependabot weekly schedule with `open-pull-requests-limit: 3` per ecosystem.
   - Rationale: tighter than GitHub's default of five to reduce review noise; separate entries for `github-actions` and `cargo`.

## Risks / Trade-offs

- [Risk] CI only on Linux may miss platform-specific issues. → Mitigation: acceptable non-goal; local development and future changes can add matrix if needed.
- [Risk] `--locked` fails when `Cargo.lock` is out of sync. → Mitigation: intentional guardrail; contributors must commit lockfile updates.
- [Risk] Dependabot PR volume despite limits. → Mitigation: weekly cadence and per-ecosystem cap of five open PRs.
