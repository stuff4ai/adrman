# github-actions-ci Specification

## Purpose
Define continuous integration and dependency update automation for the adrman Rust workspace via GitHub Actions and Dependabot.
## Requirements
### Requirement: CI workflow triggers
The repository SHALL provide a GitHub Actions workflow that runs on pull requests targeting `main`, pushes to `main`, and manual `workflow_dispatch`.

#### Scenario: Pull request targets main
- **WHEN** a pull request is opened or updated with base branch `main`
- **THEN** the CI workflow runs

#### Scenario: Push to main
- **WHEN** a commit is pushed to the `main` branch
- **THEN** the CI workflow runs

#### Scenario: Manual dispatch
- **WHEN** a maintainer triggers the workflow via `workflow_dispatch`
- **THEN** the CI workflow runs

### Requirement: Single Ubuntu CI job
The CI workflow SHALL use exactly one job on `ubuntu-latest` without an operating-system matrix.

#### Scenario: Workflow job configuration
- **WHEN** the CI workflow executes
- **THEN** it runs a single job on `ubuntu-latest`
- **AND** does not define a strategy matrix for operating systems

### Requirement: Least-privilege workflow permissions
The CI workflow SHALL declare workflow-level permissions limited to reading repository contents.

#### Scenario: Workflow permissions scope
- **WHEN** the CI workflow file is inspected
- **THEN** `permissions.contents` is `read`
- **AND** no broader default permissions are required for the workflow

### Requirement: Outdated run cancellation
The CI workflow SHALL cancel in-progress runs for the same workflow and ref when a newer run starts.

#### Scenario: Concurrent pushes on the same ref
- **WHEN** a new CI run starts for a ref that already has an in-progress run of the same workflow
- **THEN** the older in-progress run is cancelled

### Requirement: Short job timeout
The CI job SHALL define a timeout no longer than 20 minutes.

#### Scenario: Job timeout configured
- **WHEN** the CI workflow file is inspected
- **THEN** the job sets `timeout-minutes` to a value of 20 or less

### Requirement: Rust quality gates
The CI job SHALL run these commands in order after checking out the repository and installing Rust:
1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
3. `cargo test --workspace --all-features --locked`

#### Scenario: Formatting check fails
- **WHEN** source code is not formatted according to `rustfmt`
- **THEN** the `cargo fmt --all -- --check` step fails
- **AND** the workflow exits unsuccessfully

#### Scenario: Clippy warnings fail the build
- **WHEN** `cargo clippy` reports warnings
- **THEN** the clippy step fails because warnings are denied
- **AND** the workflow exits unsuccessfully

#### Scenario: Tests must pass
- **WHEN** any workspace test fails
- **THEN** the `cargo test` step fails
- **AND** the workflow exits unsuccessfully

### Requirement: Rust toolchain configuration
The CI workflow SHALL install and activate Rust using the repository `rust-toolchain.toml` rather than hard-coding a different toolchain specification in the workflow.

#### Scenario: Toolchain file governs Rust setup
- **WHEN** CI installs Rust for the job
- **THEN** rustup reads `rust-toolchain.toml` at the repository root to select channel, profile, and components
- **AND** the workflow does not pin a conflicting channel or component set in separate install inputs

### Requirement: Dependabot dependency updates
The repository SHALL configure Dependabot to open weekly update pull requests for GitHub Actions and Cargo with at most three simultaneous open pull requests per ecosystem.

#### Scenario: Weekly Actions updates
- **WHEN** Dependabot evaluates scheduled updates for GitHub Actions
- **THEN** it uses a weekly schedule
- **AND** limits open pull requests for that ecosystem

#### Scenario: Weekly Cargo updates
- **WHEN** Dependabot evaluates scheduled updates for Cargo
- **THEN** it uses a weekly schedule
- **AND** limits open pull requests for that ecosystem

### Requirement: CI scope exclusions
The CI and Dependabot setup MUST NOT add operating-system matrix builds, artifact uploads, coverage reporting, or release automation.

#### Scenario: Workflow stays minimal
- **WHEN** the CI workflow file is inspected
- **THEN** it does not upload artifacts
- **AND** it does not run coverage tools
- **AND** it does not publish releases

