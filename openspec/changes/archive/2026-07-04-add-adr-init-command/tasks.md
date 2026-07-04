## 1. Core ADR init logic

- [x] 1.1 Add default template content constant and `init_adr_workspace` in `adrman-core`.
- [x] 1.2 Create `docs/adr/` when missing and write template with create-if-absent semantics.
- [x] 1.3 Add unit tests for created, already-exists, and parent-directory creation paths.

## 2. CLI command surface

- [x] 2.1 Add `adr init` parsing and dispatch in `adrman-cli`.
- [x] 2.2 Wire success and already-exists stdout messages.
- [x] 2.3 Update usage text for the new command.

## 3. CLI integration tests

- [x] 3.1 Add integration test that init creates `docs/adr/.adr-template.md`.
- [x] 3.2 Add integration test that init creates missing parent directories.
- [x] 3.3 Add integration test that init does not overwrite an existing template.
- [x] 3.4 Add integration test that generated template content matches expected content exactly.

## 4. Documentation

- [x] 4.1 Update README with `adr init` behavior and examples.
