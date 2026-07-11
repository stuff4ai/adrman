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
3. `cargo llvm-cov nextest --workspace --all-features --locked --profile ci`

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
- **THEN** the `cargo llvm-cov nextest` step fails
- **AND** the workflow exits unsuccessfully

### Requirement: CI report job summary
The CI workflow SHALL write a Markdown report to `$GITHUB_STEP_SUMMARY` that summarizes overall CI status and the results of formatting, clippy, tests, and coverage.

#### Scenario: Job summary is generated
- **WHEN** the CI workflow finishes or fails
- **THEN** a Markdown report is appended to `$GITHUB_STEP_SUMMARY`
- **AND** the report includes overall CI status plus formatting, clippy, test, and coverage sections

### Requirement: Nextest test execution and JUnit output
The CI workflow SHALL run workspace tests with `cargo-nextest` through `cargo llvm-cov nextest` and produce JUnit output from a `ci` nextest profile.

#### Scenario: JUnit report is produced
- **WHEN** CI runs tests
- **THEN** nextest writes JUnit XML for the `ci` profile
- **AND** the workflow uploads that report as a CI artifact

### Requirement: Coverage collection and artifacts
The CI workflow SHALL generate coverage with `cargo-llvm-cov` and upload machine-readable and human-readable coverage artifacts without requiring external coverage services.

#### Scenario: Coverage artifacts are uploaded
- **WHEN** CI completes test execution
- **THEN** the workflow uploads an LCOV report, an HTML coverage report, and a text coverage summary
- **AND** coverage thresholds do not fail the workflow in the initial implementation

### Requirement: CI scope exclusions
The CI and Dependabot setup MUST NOT add operating-system matrix builds, paid third-party coverage services, GitHub Pages publishing, or release automation.

#### Scenario: Workflow stays focused
- **WHEN** the CI workflow file is inspected
- **THEN** it does not define an operating-system matrix
- **AND** it does not require Codecov or another paid coverage service
- **AND** it does not publish GitHub Pages or releases

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

