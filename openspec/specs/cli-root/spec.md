# cli-root Specification

## Purpose
Define the root executable contract for adrman: the `adr` binary name, root invocation shape, top-level command routing, aliases, shared usage text, and deferred help/version behavior.

## Requirements

### Requirement: CLI binary naming
The distributed CLI binary MUST be named `adr`.

#### Scenario: User runs the CLI binary
- **WHEN** the CLI is built from the Cargo workspace
- **THEN** the binary command name is `adr`

### Requirement: Root invocation shape
The CLI SHALL be invoked as `adr <command> [command arguments]`.

Command-specific arguments and flags are governed by the corresponding per-command specs.

#### Scenario: User invokes a root command
- **WHEN** a user runs `adr list`
- **THEN** the CLI treats `list` as the top-level command token
- **AND** routes execution to the list command workflow

### Requirement: Top-level command surface
The `adr` binary SHALL expose these top-level commands and aliases:
- `init`
- `list` with alias `ls`
- `new`
- `check` with alias `validate`
- `index`

#### Scenario: List alias routes to list workflow
- **WHEN** a user runs `adr ls`
- **THEN** the CLI routes execution to the same listing workflow as `adr list`

#### Scenario: Validate alias routes to check workflow
- **WHEN** a user runs `adr validate`
- **THEN** the CLI routes execution to the same validation workflow as `adr check`

### Requirement: Missing or unknown subcommand usage
When no subcommand is provided or the subcommand is unknown, the CLI MUST print the project usage message to standard error and exit with status code `2`.

The usage message MUST list the supported commands:
- `init`
- `list, ls`
- `new`
- `check, validate`
- `index`

#### Scenario: User runs adr without a subcommand
- **WHEN** a user runs `adr` with no subcommand
- **THEN** the CLI prints the usage message to standard error
- **AND** exits with status code `2`

#### Scenario: User runs an unknown subcommand
- **WHEN** a user runs `adr unknown`
- **THEN** the CLI prints the usage message to standard error
- **AND** exits with status code `2`

### Requirement: CLI exit-code policy
The CLI MUST use exit codes to distinguish successful execution, command outcome failures, and CLI usage errors:
- `0` for success, including idempotent no-op outcomes where the requested state is already true
- `1` for valid invocations where the requested operation failed or check did not pass
- `2` for invocations that do not match supported CLI syntax

Command handlers MUST NOT return exit code `1` for pure CLI usage errors.

#### Scenario: User invokes a command with invalid syntax
- **WHEN** a user runs `adr` with an unknown flag, missing required argument, or unsupported flag value
- **THEN** the CLI exits with status code `2`

#### Scenario: User invokes a valid command that fails its outcome check
- **WHEN** a user runs `adr check` and validation finds issues
- **THEN** the CLI exits with status code `1`

### Requirement: Deferred help and version output
The CLI MUST NOT expose comprehensive `--help` or `--version` output in this change.

Comprehensive help and version behavior are tracked separately and MUST remain out of scope until their dedicated changes land.

#### Scenario: Built-in help flag is not enabled
- **WHEN** a user runs `adr --help`
- **THEN** the CLI does not print comprehensive command help output

### Requirement: CLI output stream policy
The CLI MUST use standard output for command results and standard error for user-facing diagnostics.

Command results include requested data, generated file paths, human-readable reports, state-check reports, and machine-readable output such as JSON.

Diagnostics include usage errors, warnings, concise next-step guidance, operational errors that prevent producing the requested result, and future progress or status messages for slower operations.

Normal standard error MUST NOT be used as a default log stream and MUST NOT include log-level prefixes, timestamps, module paths, Rust debug structs, backtraces, or internal implementation context by default.

Machine-readable standard output MUST NOT be mixed with human diagnostics.

Future output modes such as `--quiet`, `--verbose`, `--debug`, `--plain`, or `--porcelain` are tracked separately and MUST remain out of scope until their dedicated changes land.

#### Scenario: Usage error is diagnostic output
- **WHEN** a user invokes `adr` with unsupported syntax
- **THEN** the CLI prints the usage or error message to standard error
- **AND** exits with status code `2`

#### Scenario: Command report is result output
- **WHEN** a command reports requested state, such as validation results
- **THEN** the CLI prints the report to standard output
- **AND** uses the exit code to signal pass or failure

#### Scenario: Generated file path is result output
- **WHEN** a command creates or updates a file and prints the resulting path
- **THEN** the CLI prints the path to standard output

#### Scenario: Operational error is diagnostic output
- **WHEN** a command cannot produce the requested result because of a missing prerequisite, such as a missing ADR directory before index generation
- **THEN** the CLI prints a concise user-facing error with next-step guidance to standard error
- **AND** exits with a non-zero status code
