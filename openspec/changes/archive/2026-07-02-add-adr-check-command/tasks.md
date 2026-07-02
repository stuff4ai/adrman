## 1. Core ADR validation logic

- [x] 1.1 Add ADR validation types, issue codes, and `check_adrs` in `adrman-core`.
- [x] 1.2 Add section extraction, duplicate ID detection, invalid filename detection, and status validation.
- [x] 1.3 Add human-readable and JSON formatters for validation results.
- [x] 1.4 Add unit tests for valid ADRs, invalid filenames, duplicate IDs, missing sections, empty sections, and invalid statuses.

## 2. CLI command surface

- [x] 2.1 Add `adr check` and `adr validate` parsing and dispatch in `adrman-cli`.
- [x] 2.2 Add `--format json` flag handling and exit code mapping.
- [x] 2.3 Update usage text for the new commands.

## 3. CLI integration tests

- [x] 3.1 Add integration tests for successful validation, failing validation, and JSON output.
- [x] 3.2 Reuse shared CLI test helpers for ADR workspace setup.

## 4. Documentation

- [x] 4.1 Update README with `adr check` / `adr validate` behavior and examples.
