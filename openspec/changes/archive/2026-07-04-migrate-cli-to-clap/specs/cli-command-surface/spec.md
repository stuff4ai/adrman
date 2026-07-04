## ADDED Requirements

### Requirement: Top-level command surface
The `adr` binary SHALL expose these subcommands and aliases:
- `init`
- `list` with alias `ls`
- `new <title>`
- `check` with alias `validate`
- `index`

#### Scenario: List alias executes list workflow
- **WHEN** a user runs `adr ls`
- **THEN** the CLI executes the same listing workflow as `adr list`

#### Scenario: Validate alias executes check workflow
- **WHEN** a user runs `adr validate`
- **THEN** the CLI executes the same validation workflow as `adr check`

### Requirement: Supported command flags
The command surface SHALL accept:
- `adr check --format json`
- `adr index --check`

#### Scenario: Check JSON format flag is accepted
- **WHEN** a user runs `adr check --format json`
- **THEN** the CLI executes the check command with JSON output formatting

#### Scenario: Index check flag is accepted
- **WHEN** a user runs `adr index --check`
- **THEN** the CLI executes the index check workflow

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

### Requirement: Unexpected arguments on argument-free commands
Commands that take no arguments or flags MUST reject unexpected input with `Error: unexpected argument '<value>'` on standard error and exit with status code `2`.

#### Scenario: List command rejects unexpected flags
- **WHEN** a user runs `adr list --foo`
- **THEN** the CLI prints `Error: unexpected argument '--foo'` to standard error
- **AND** exits with status code `2`

#### Scenario: Init command rejects unexpected positional arguments
- **WHEN** a user runs `adr init foo`
- **THEN** the CLI prints `Error: unexpected argument 'foo'` to standard error
- **AND** exits with status code `2`

### Requirement: Unexpected flags on flag-bearing commands
Commands that accept flags MUST reject unknown flags with `Error: unexpected argument '<value>'` on standard error and exit with status code `2`.

#### Scenario: Check command rejects unexpected flags
- **WHEN** a user runs `adr check --foo`
- **THEN** the CLI prints `Error: unexpected argument '--foo'` to standard error
- **AND** exits with status code `2`

#### Scenario: Index command rejects unexpected flags
- **WHEN** a user runs `adr index --foo`
- **THEN** the CLI prints `Error: unexpected argument '--foo'` to standard error
- **AND** exits with status code `2`

### Requirement: Deferred help and version output
The CLI MUST NOT expose comprehensive `--help` or `--version` output in this change.

Comprehensive help and version behavior are tracked separately and MUST remain out of scope until their dedicated changes land.

#### Scenario: Built-in help flag is not enabled
- **WHEN** a user runs `adr --help`
- **THEN** the CLI does not print comprehensive command help output
