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

### Exit codes

adrman uses a small exit-code policy so scripts, CI, and agents can interpret results reliably:

| Code | Meaning | Examples |
|------|---------|----------|
| `0` | Success — command invoked correctly and completed successfully | `adr init`, `adr list` when `docs/adr/` is missing, `adr check` with valid ADRs, `adr index --check` with a fresh index |
| `1` | Command outcome failure — valid invocation, but the operation failed or check did not pass | invalid ADRs, stale or missing index, missing template, duplicate ADR IDs, filesystem errors |
| `2` | CLI usage error — command was not invoked according to supported syntax | no subcommand, unknown command, invalid flag, missing required argument, unsupported flag value, unexpected extra argument |

### `adr init`

- Bootstraps an ADR workspace in the current repository.
- Creates `docs/adr/` when it does not exist.
- Creates `docs/adr/.adr-template.md` when it does not exist.
- Does not overwrite an existing `docs/adr/.adr-template.md`.
- Prints a success message when the template is created.
- Prints an informative message when the template already exists.
- Exits with status code `0` for both created and already-existing outcomes.

Example:

```bash
adr init
```

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

### `adr check` / `adr validate`

- Validates ADR files in `docs/adr/` only.
- Reports markdown files in `docs/adr/` that do not match `^[0-9]+[-_ ].*\.md$`, except `docs/adr/.adr-template.md`.
- Detects duplicate numeric ADR IDs across included files.
- Requires Nygard sections with exact `##` headings:
  - `Status`
  - `Context`
  - `Decision`
  - `Consequences`
- Reports missing or empty required sections.
- Validates `Status` values against:
  - `Proposed`
  - `Accepted`
  - `Rejected`
  - `Deprecated`
  - `Superseded`
- Prints human-readable output by default.
- Supports `--format json` for agents and CI.
- Exits with status code `0` when all ADRs are valid.
- Exits with a non-zero status code when validation fails or `docs/adr/` is missing.

Examples:

```bash
adr check
adr validate --format json
```

### `adr index`

- Generates or updates `docs/adr/README.md`.
- Reuses the same ADR discovery and metadata extraction rules as `adr list`.
- Writes a Markdown table with columns `ID`, `Status`, `Title`, and `ADR`.
- Each `ADR` cell links to the ADR filename with a relative Markdown link.
- Sorts rows by numeric `ID`, then by `File`.
- Prints the written index path on success.
- Supports `--check` for CI:
  - Exits with status code `0` when `docs/adr/README.md` exists and matches the generated index.
  - Exits with a non-zero status code when the index is missing, stale, or `docs/adr/` is missing.

Examples:

```bash
adr index
adr index --check
```
