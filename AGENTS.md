# AGENTS.md

**adrman** is a Git-first, agent-friendly CLI tool
for managing [Architectural (Any) Decision Records](https://adr.github.io/).

## Project rules

- Keep changes small and reviewable.
- Prefer plain files and Git-friendly workflows.
- Do not introduce unrelated changes.
- Do not push directly to `main`.
- Do not merge pull requests.
- Do not publish releases.
- Update documentation when behavior or workflow changes.

## Decision records

Use ADRs in `docs/adr/` to understand and record important project decisions.

Before proposing a change, check existing ADRs for relevant decisions.

When a change introduces or changes an important decision, add a new ADR.

Accepted ADRs are immutable history. To replace one, create a new ADR that supersedes it.

Keep ADRs brief, plain, and reviewable. Use the template in `docs/adr/.adr-template.md`.

Do not renumber existing ADRs.

## Spec-driven development

Use OpenSpec in `openspec/` for spec-driven development of planned product changes.

Use ADRs for durable project decisions.

Use OpenSpec for proposals, requirements, designs, and implementation tasks before changing product behavior.

Do not use OpenSpec for tiny documentation-only changes.

Before implementing a feature, check active OpenSpec changes and relevant specs.

Keep OpenSpec changes small, focused, and reviewable.

## Change workflow

Use GitHub Flow:

1. Create a short-lived branch from `main`.
2. Make a focused change.
3. Open a pull request.
4. Wait for human review before merging.

Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages.

## Rust commands

Use [Cargo](https://doc.rust-lang.org/cargo/) commands for Rust development.

- Check workspace: `cargo check --workspace`
- Format code: `cargo fmt`
- Check formatting: `cargo fmt --check`
- Run lints: `cargo clippy --workspace --all-targets`
- Run tests: `cargo test --workspace`
- Run CLI: `cargo run -p adrman-cli --bin adr -- list`

## Cursor Cloud specific instructions

This is a self-contained Rust Cargo workspace (`adrman-cli` binary + `adrman-core` library). There are no servers, databases, network calls, or other background services to start — verification is entirely via the Cargo commands listed under "Rust commands" above.

The pinned toolchain lives in `rust-toolchain.toml` (stable, with `clippy`/`rustfmt`/`rust-src`); `rustup` auto-installs it on the first `cargo` invocation, so the initial command in a fresh VM may pause briefly while the toolchain syncs.
