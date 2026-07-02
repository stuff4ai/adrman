# adrman ⚖️

**adrman** is a Git-first, agent-friendly CLI tool
for managing [Architectural (Any) Decision Records](https://adr.github.io/).

adrman helps humans and AI agents work with decision records
as plain files in a Git repository.

The CLI binary name is `adr`.

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
cargo run -p adrman-cli --bin adr -- list
```

## Commands

### `adr list` / `adr ls`

- Reads ADR files from `docs/adr/` only.
- Includes only files matching `^[0-9]+[-_ ].*\.md$`.
- Extracts:
  - `ID` from filename numeric prefix
  - `Status` from `## Status`
  - `Title` from first `# ` heading
  - `File` as base filename
- Uses `Unknown` when title or status is missing.
- If `docs/adr/` is missing, prints a warning and exits successfully.
- Sorts rows by numeric `ID`, then by `File`.

Output format:

```text
ADRs (docs/adr/)

ID    Status    Title    File
```

### `adr new "<title>"`

- Requires a title argument.
- Requires an existing ADR workspace: `docs/adr/` and `docs/adr/.adr-template.md`.
- Does not create `docs/adr/` or bootstrap a template when either is missing.
- Discovers the next ADR ID from files in `docs/adr/` matching `^[0-9]+[-_ ].*\.md$`.
- Assigns the highest numeric filename prefix plus one, formatted as four digits (for example `0005`).
- Builds filenames as `<id>-<slug>.md`, where the slug is a lowercase ASCII form of the title.
- Populates new files from `docs/adr/.adr-template.md`:
  - Replaces `# Title` with the provided title.
  - Sets the initial `## Status` content to `Proposed`.
- Refuses to overwrite an existing target file.
- Prints the created file path on success.

Example:

```bash
adr new "Use SQLite for local cache"
```
