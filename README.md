# adrman ⚖️

**adrman** is a Git-first, agent-friendly CLI tool
for managing [Architectural (Any) Decision Records](https://adr.github.io/).

adrman helps humans and AI agents work with decision records
as plain files in a Git repository.

## Goals

- Keep decision records easy to create, review, and evolve through Git.
- Support configurable paths, templates, statuses, relationships, and indexes.
- Help agents discover existing decisions before proposing changes.
- Provide structured output for automation, CI, and AI-assisted workflows.

## Development

adrman is a [Rust](https://www.rust-lang.org/) [Cargo](https://doc.rust-lang.org/cargo/) workspace.

### Check workspace

```bash
cargo check --workspace
```

### Format code

```bash
cargo fmt
```

### Check formatting

```bash
cargo fmt --check
```

### Run lints

```bash
cargo clippy --workspace --all-targets
```

### Run tests

```bash
cargo test --workspace
```

### Run CLI

```bash
cargo run -p adrman-cli
```
