## Context

adrman exposes a small set of subcommands (`init`, `list`, `new`, `check`, `index`) with a few aliases and flags. Manual parsing in `main.rs` matched commands by string and iterated remaining args for flags. The clap migration keeps user-visible outcomes stable while making parser structure explicit and extensible.

## Goals / Non-Goals

**Goals:**
- Parse CLI input with `clap` derive types (`Parser`, `Subcommand`, `Args`).
- Preserve command surface: `init`, `list`/`ls`, `new`, `check`/`validate`, `index`.
- Preserve `adr check --format json` and `adr index --check`.
- Preserve existing parser error messages and exit codes where tests already define them.
- Reject unexpected arguments on known commands with explicit error messages.
- Accept `adr new` title tokens that begin with `-` as titles, matching manual parsing behavior.
- Disable clap built-in help and version flags until dedicated issues land.

**Non-Goals:**
- Comprehensive help text (#17).
- Version output (#18).
- Centralized exit-code policy beyond current parser behavior (#15).

## Decisions

1. Add a dedicated `cli` module in `adrman-cli` for parser types and error mapping.
   - Rationale: keeps `main.rs` focused on command execution.
   - Alternative considered: inline all parser types in `main.rs`. Rejected because error mapping and subcommand definitions add noise.

2. Use clap derive parsing with custom error handling for legacy messages.
   - Rationale: typed parsing with preserved stderr contracts for existing integration tests.
   - Alternative considered: accept clap default error text. Rejected because several commands already have explicit error-message tests.

3. Treat unexpected arguments on known commands as parser errors, not top-level usage fallback.
   - Rationale: clap resolves the subcommand first; explicit per-command errors are clearer than re-printing global usage.
   - Behavioral change: `adr list --foo` and `adr init foo` previously printed the top-level usage message because manual parsing required zero trailing args.

4. Keep `new` title validation split between parser and command handler.
   - Rationale: missing title and extra title tokens retain existing exit codes (`1`) and messages.
   - Alternative considered: rely entirely on clap positional validation. Rejected because `adr new` without a title must keep exit code `1`, not parser exit code `2`.

5. Enable `allow_hyphen_values` for `adr new` title tokens.
   - Rationale: manual parsing treated tokens such as `--help` and `-foo` as titles; clap otherwise interprets them as flags.
   - Alternative considered: require `--` before hyphen-prefixed titles. Rejected because it changes existing user-visible behavior.

6. Store unsupported `--format` values in command dispatch, not clap `ValueEnum`.
   - Rationale: preserves the existing `Error: unsupported format '...'` message and exit code `2`.

7. Document product behavior in `cli-command-surface` rather than parser implementation details.
   - Rationale: long-term OpenSpec requirements should describe user-visible CLI behavior, not the parsing library choice.
   - Alternative considered: a `cli-parser` capability requiring `clap`. Rejected because implementation choices belong in design notes, not durable specs.

## Risks / Trade-offs

- [Risk] Future clap upgrades change default error text. -> Mitigation: parser integration tests lock in required messages and exit codes.
- [Risk] Help/version behavior deferred may confuse users expecting `-h`. -> Mitigation: tracked separately in #17 and #18; clap help flags remain disabled until then.
