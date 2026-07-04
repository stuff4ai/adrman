## Why

adrman currently parses CLI arguments manually in `adrman-cli`. That works for the current command set, but makes aliases, flags, usage errors, and future command growth harder to keep consistent. Migrating to `clap` establishes typed parser infrastructure aligned with the Command Line Interface Guidelines.

## What Changes

- Replace manual `std::env::args()` dispatch with derive-based `clap` parsing in `adrman-cli`.
- Preserve existing commands, aliases, flags, and command outcomes.
- Preserve the current minimal usage message for missing or unknown top-level commands.
- Report unexpected arguments on known commands with explicit `Error: unexpected argument '...'` messages instead of falling back to the top-level usage message.
- Preserve `adr new` title-token compatibility, including titles that begin with `-`.
- Keep comprehensive help and version output out of scope for this change.

## Capabilities

### New Capabilities
- `cli-command-surface`: Top-level `adr` subcommands, aliases, shared usage output, and argument rejection behavior.

### Modified Capabilities
- `adr-new-command`: Document single-token title shape and hyphen-prefixed title acceptance.
- `adr-check-command`: Document `--format` flag validation errors.

## Impact

- Affected code:
  - `adrman-cli` parser module, command dispatch, and parser integration tests.
  - `Cargo.toml` dependency on `clap`.
- Documentation:
  - New OpenSpec capability for CLI command surface behavior.
  - Updated per-command specs for `adr new` title tokens and `adr check --format` validation.
- Non-goals:
  - Comprehensive `--help` output (#17).
  - `--version` output (#18).
  - Full exit-code policy consolidation (#15).
