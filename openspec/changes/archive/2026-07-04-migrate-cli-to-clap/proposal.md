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
- `cli-root`: Root `adr` binary name, invocation shape, top-level subcommands, aliases, shared usage output, and help/version deferral.

### Modified Capabilities
- `cli-command-name`: Merge the existing binary naming requirement into `cli-root`, because the `adr` binary name belongs to the root executable contract.
- `cli-new`: Document single-token title shape and hyphen-prefixed title acceptance.
- `cli-check`: Document `--format` flag validation errors.

## Impact

- Affected code:
  - `adrman-cli` parser module, command dispatch, and parser integration tests.
  - `Cargo.toml` dependency on `clap`.
- Documentation:
  - New OpenSpec capability for root-level CLI behavior, including the merged `cli-command-name` binary naming requirement.
  - Updated per-command specs for `adr new` title tokens and `adr check --format` validation.
- Non-goals:
  - Comprehensive `--help` output (#17).
  - `--version` output (#18).
  - Full exit-code policy consolidation (#15).
