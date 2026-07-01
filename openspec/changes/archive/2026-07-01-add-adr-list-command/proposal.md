## Why

adrman currently cannot show a concise index of existing ADR files, which makes it slower for humans and agents to discover decision history in large repositories. Adding a dedicated list command as the first end-user feature establishes a predictable read workflow and validates the core ADR parsing surface early.

## What Changes

- Add a new ADR listing command with aliases: `adr list` and `adr ls`.
- Define command behavior to read ADR files from `docs/adr/` only.
- Define file inclusion rules using `^[0-9]+[-_ ].*\.md$`.
- Define extracted fields per ADR row:
  - ID from filename numeric prefix.
  - Status from `## Status` section content.
  - Title from the first `# ` heading.
  - File as the base filename.
- Define fallback behavior for missing metadata:
  - Missing title -> `Unknown`.
  - Missing status -> `Unknown`.
- Define directory-missing behavior:
  - If `docs/adr/` does not exist, print a warning and exit successfully.
- Define output contract:
  - Header: `ADRs (docs/adr/)`
  - Column header row: `ID    Status    Title    File`
  - Rows sorted by numeric ID, then filename.
- Rename the CLI binary from `adrman` to `adr`.
- Non-goals for this change:
  - No filtering, paging, or JSON output.
  - No recursive directory traversal outside `docs/adr/`.
  - No edits to ADR files or status normalization rules.

## Capabilities

### New Capabilities
- `adr-list-command`: List ADR entries from `docs/adr/` with deterministic parsing and tabular output.

### Modified Capabilities
- None.

## Impact

- Affected code:
  - `adrman-cli` command parsing and command dispatch.
  - ADR file discovery/parsing logic (likely in `adrman-core`).
  - CLI output formatting for tabular list output and warning behavior.
- Tooling/workflow:
  - Cargo package/bin naming and user-facing CLI invocation updates from `adrman` to `adr`.
- Documentation:
  - README usage examples and any command references must reflect `adr`.
