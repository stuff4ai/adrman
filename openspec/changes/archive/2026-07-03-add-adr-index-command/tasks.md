## 1. Core ADR index logic

- [x] 1.1 Add index generation, comparison, and write helpers in `adrman-core`.
- [x] 1.2 Reuse `list_adrs` for discovery and metadata extraction.
- [x] 1.3 Add Markdown table formatting with deterministic ordering and pipe escaping.
- [x] 1.4 Add unit tests for generation, up-to-date check, missing index, and stale index.

## 2. CLI command surface

- [x] 2.1 Add `adr index` parsing and dispatch in `adrman-cli`.
- [x] 2.2 Add `--check` flag handling and exit code mapping.
- [x] 2.3 Update usage text for the new command.

## 3. CLI integration tests

- [x] 3.1 Add integration tests for index generation.
- [x] 3.2 Add integration tests for `--check` success, missing index, and stale index.
- [x] 3.3 Reuse shared CLI test helpers for ADR workspace setup.

## 4. Documentation and repository index

- [x] 4.1 Update README with `adr index` behavior and examples.
- [x] 4.2 Generate `docs/adr/README.md` for this repository.
