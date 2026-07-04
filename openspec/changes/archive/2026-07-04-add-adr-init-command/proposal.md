## Why

adrman can create ADRs with `adr new`, but only when `docs/adr/` and `docs/adr/.adr-template.md` already exist. Adding `adr init` gives humans and agents a single command to bootstrap a new repository with the standard ADR directory and template.

## What Changes

- Add a new CLI command: `adr init`.
- Create `docs/adr/` when it does not exist.
- Create `docs/adr/.adr-template.md` when it does not exist with:
  - YAML front metadata containing `filename: "XXXX-{slug}.md"`.
  - The standard Nygard ADR section headings (`Title`, `Status`, `Context`, `Decision`, `Consequences`).
- Do not overwrite an existing `docs/adr/.adr-template.md`.
- Print a clear success message when the template is created.
- Print a clear message when the template already exists.
- Non-goals for this change:
  - No config support.
  - No JSON output.
  - No changes to `adr new` template population behavior.
  - No index generation.

## Capabilities

### New Capabilities
- `adr-init-command`: Bootstrap `docs/adr/` and the default `.adr-template.md` for new ADR workspaces.

### Modified Capabilities
- None.

## Impact

- Affected code:
  - `adrman-core` workspace initialization logic and template content constant.
  - `adrman-cli` command parsing and dispatch for the new `init` subcommand.
  - CLI integration tests for init behavior.
- Documentation:
  - README command reference for `adr init`.
