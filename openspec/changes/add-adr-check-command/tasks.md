## 1. Core ADR validation logic

- [ ] 1.1 Add ADR validation types, issue codes, and `check_adrs` in `adrman-core`.
- [ ] 1.2 Add section extraction, duplicate ID detection, invalid filename detection, and status validation.
- [ ] 1.3 Add human-readable and JSON formatters for validation results.
- [ ] 1.4 Add unit tests for valid ADRs, invalid filenames, duplicate IDs, missing sections, empty sections, and invalid statuses.

## 2. CLI command surface

- [ ] 2.1 Add `adr check` and `adr validate` parsing and dispatch in `adrman-cli`.
- [ ] 2.2 Add `--format json` flag handling and exit code mapping.
- [ ] 2.3 Update usage text for the new commands.

## 3. CLI integration tests

- [ ] 3.1 Add integration tests for successful validation, failing validation, and JSON output.
- [ ] 3.2 Reuse shared CLI test helpers for ADR workspace setup.

## 4. Documentation

- [ ] 4.1 Update README with `adr check` / `adr validate` behavior and examples.
