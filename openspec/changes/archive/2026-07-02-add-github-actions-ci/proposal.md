## Why

adrman has no automated checks on pull requests or `main`, so formatting drift, clippy warnings, and test regressions can land unnoticed. A minimal GitHub Actions workflow and Dependabot configuration gives contributors fast feedback without the cost of matrix builds, coverage, or release automation.

## What Changes

- Add `.github/workflows/ci.yml` with a single `ubuntu-latest` job that runs `cargo fmt --check`, `cargo clippy`, and `cargo test` for the workspace.
- Trigger CI on pull requests targeting `main`, pushes to `main`, and manual `workflow_dispatch`.
- Use least-privilege workflow permissions, concurrency cancellation for outdated runs, and a short job timeout.
- Honor the existing `rust-toolchain.toml` for Rust channel and components.
- Add `.github/dependabot.yml` for weekly GitHub Actions and Cargo dependency updates with a low open-PR limit.
- Non-goals for this change:
  - No OS matrix builds.
  - No artifact uploads.
  - No coverage reporting.
  - No release automation.

## Capabilities

### New Capabilities
- `github-actions-ci`: Continuous integration workflow and dependency update automation for the repository.

### Modified Capabilities
- None.

## Impact

- Affected files:
  - `.github/workflows/ci.yml` (new).
  - `.github/dependabot.yml` (new).
- Tooling/workflow:
  - Pull requests and `main` receive automated Rust quality gates.
  - Dependabot opens limited weekly update PRs for Actions and Cargo.
- Documentation:
  - No user-facing CLI behavior changes; workflow expectations are captured in OpenSpec.
