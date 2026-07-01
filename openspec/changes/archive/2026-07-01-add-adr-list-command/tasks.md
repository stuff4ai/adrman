## 1. CLI command surface and binary rename

- [x] 1.1 Rename CLI binary configuration from `adrman` to `adr` and update command help/usage wiring.
- [x] 1.2 Add `list` subcommand with `ls` alias in CLI argument parsing and dispatch.
- [x] 1.3 Add tests that verify `adr list` and `adr ls` execute identical behavior paths.

## 2. ADR discovery and metadata extraction

- [x] 2.1 Implement ADR file discovery limited to `docs/adr/` with inclusion regex `^[0-9]+[-_ ].*\.md$`.
- [x] 2.2 Implement metadata extraction for ID, Status (`## Status`), Title (first `# ` heading), and File basename with `Unknown` fallback for missing title/status.
- [x] 2.3 Add parser tests for complete ADR metadata, missing title, missing status, and non-matching filenames.

## 3. Output rendering and ordering behavior

- [x] 3.1 Implement missing-directory behavior to print a warning and return success when `docs/adr/` is absent.
- [x] 3.2 Implement row sorting by numeric ID, then filename, and render output header/column lines exactly as specified.
- [x] 3.3 Add integration-style CLI tests for ordering (`2` before `10`), filename tiebreaks, and output format contract.

## 4. Documentation updates

- [x] 4.1 Update README and any user-facing command docs to use binary name `adr`.
- [x] 4.2 Document `adr list`/`adr ls` behavior, ADR filename expectations, and missing-directory warning behavior.

## 5. CLI integration test foundation

- [x] 5.1 Add reusable CLI integration test helpers under `crates/adrman-cli/tests/support/` for isolated workspace setup, file writing, ADR fixture writing, and compiled `adr` execution.
- [x] 5.2 Refactor `adr list` integration tests to use shared helpers and add snapshot coverage for stable normal output.
- [x] 5.3 Document local CLI-focused test and snapshot update commands for humans and agents.
