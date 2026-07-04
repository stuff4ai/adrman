## ADDED Requirements

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

### Requirement: Deferred help and version output
The CLI MUST NOT expose comprehensive `--help` or `--version` output in this change.

Comprehensive help and version behavior are tracked separately and MUST remain out of scope until their dedicated changes land.

#### Scenario: Built-in help flag is not enabled
- **WHEN** a user runs `adr --help`
- **THEN** the CLI does not print comprehensive command help output
