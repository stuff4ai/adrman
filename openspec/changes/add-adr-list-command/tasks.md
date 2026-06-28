## 1. CLI command surface and binary rename

- [ ] 1.1 Rename CLI binary configuration from `adrman` to `adr` and update command help/usage wiring.
- [ ] 1.2 Add `list` subcommand with `ls` alias in CLI argument parsing and dispatch.
- [ ] 1.3 Add tests that verify `adr list` and `adr ls` execute identical behavior paths.

## 2. ADR discovery and metadata extraction

- [ ] 2.1 Implement ADR file discovery limited to `docs/adr/` with inclusion regex `^[0-9]+[-_ ].*\.md$`.
- [ ] 2.2 Implement metadata extraction for ID, Status (`## Status`), Title (first `# ` heading), and File basename with `Unknown` fallback for missing title/status.
- [ ] 2.3 Add parser tests for complete ADR metadata, missing title, missing status, and non-matching filenames.

## 3. Output rendering and ordering behavior

- [ ] 3.1 Implement missing-directory behavior to print a warning and return success when `docs/adr/` is absent.
- [ ] 3.2 Implement row sorting by numeric ID, then filename, and render output header/column lines exactly as specified.
- [ ] 3.3 Add integration-style CLI tests for ordering (`2` before `10`), filename tiebreaks, and output format contract.

## 4. Documentation updates

- [ ] 4.1 Update README and any user-facing command docs to use binary name `adr`.
- [ ] 4.2 Document `adr list`/`adr ls` behavior, ADR filename expectations, and missing-directory warning behavior.
