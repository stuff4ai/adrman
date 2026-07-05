## Why

adrman can list and create ADRs but cannot validate decision records against project conventions. Adding `adr check` gives humans, CI pipelines, and agents a strict validation workflow that catches structural problems before they merge.

## What Changes

- Add `adr check` with `adr validate` as an equivalent alias.
- Validate ADR files in `docs/adr/` using the existing filename discovery rules.
- Report invalid filenames, duplicate numeric ADR IDs, missing required Nygard sections, empty required sections, and unsupported status values.
- Print human-readable output by default and support `--format json` for automation.
- Exit with status code `0` when all ADRs are valid and a non-zero status code when validation fails.
- Non-goals for this change:
  - No config support for custom ADR directories, templates, or status lists.
  - No relationship or lifecycle validation.
  - No auto-fix or lint suggestions.

## Capabilities

### New Capabilities
- `cli-check`: Validate ADR files in `docs/adr/` for filename, ID, section, and status rules with human and JSON output.

### Modified Capabilities
- None.

## Impact

- Affected code:
  - `adrman-core` validation logic, section parsing, and output formatting.
  - `adrman-cli` command dispatch, `--format` flag handling, and exit codes.
  - CLI integration tests and core unit tests for validation rules.
- Documentation:
  - README command section for `adr check` / `adr validate`.
