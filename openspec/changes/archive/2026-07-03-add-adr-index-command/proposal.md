## Why

adrman can list ADRs in the terminal but does not maintain a browsable index in the repository. A generated `docs/adr/README.md` gives humans, agents, and documentation sites a stable, linkable overview of decision history without manual upkeep.

## What Changes

- Add `adr index` to generate or update `docs/adr/README.md`.
- Reuse the same ADR discovery and metadata extraction rules as `adr list`.
- Write a deterministic Markdown table with columns for ID, Status, Title, and a relative file link.
- Add `adr index --check` for CI:
  - Exit successfully when `docs/adr/README.md` exists and matches the generated index.
  - Exit with a non-zero status when the index is missing or stale.
- Non-goals for this change:
  - No custom output paths or index templates.
  - No JSON output mode.
  - No validation of ADR content beyond what `adr list` already extracts.

## Capabilities

### New Capabilities
- `adr-index-command`: Generate and verify a Markdown ADR index at `docs/adr/README.md`.

### Modified Capabilities
- None.

## Impact

- Affected code:
  - `adrman-core` index generation, comparison, and file writing logic.
  - `adrman-cli` command dispatch, `--check` flag handling, and exit codes.
  - CLI integration tests and core unit tests for generation and check behavior.
- Documentation:
  - README command section for `adr index`.
  - Generated `docs/adr/README.md` in this repository.
