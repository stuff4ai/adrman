## Why

adrman currently parses CLI arguments manually in `adrman-cli`. That works for the current command set, but makes aliases, flags, usage errors, and future command growth harder to keep consistent. Migrating to `clap` establishes typed parser infrastructure aligned with the Command Line Interface Guidelines.

## What Changes

- Replace manual `std::env::args()` dispatch with derive-based `clap` parsing in `adrman-cli`.
- Preserve existing commands, aliases, flags, and command outcomes.
- Preserve the current minimal usage message for missing or unknown top-level commands.
- Report unexpected arguments on known commands with explicit `Error: unexpected argument '...'` messages instead of falling back to the top-level usage message.
- Keep comprehensive help and version output out of scope for this change.

## Capabilities

### New Capabilities
- `cli-parser`: Typed CLI argument parsing, command dispatch, aliases, flags, and parser error handling for the `adr` binary.

### Modified Capabilities
- None. Command behavior remains specified in existing per-command specs (`adr-list-command`, `adr-check-command`, etc.).

## Impact

- Affected code:
  - `adrman-cli` parser module, command dispatch, and parser integration tests.
  - `Cargo.toml` dependency on `clap`.
- Documentation:
  - New OpenSpec capability for CLI parser behavior.
- Non-goals:
  - Comprehensive `--help` output (#17).
  - `--version` output (#18).
  - Full exit-code policy consolidation (#15).
