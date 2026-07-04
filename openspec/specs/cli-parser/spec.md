# cli-parser Specification

## Purpose
Define how the `adr` binary parses commands, aliases, flags, and parser-level errors before dispatching to per-command workflows.

## Requirements

### Requirement: CLI parser library
The `adr` binary SHALL parse command-line input with `clap` using derive-based parser types.

#### Scenario: Parser resolves a known subcommand
- **WHEN** a user runs `adr list`
- **THEN** the CLI dispatches to the list command workflow

### Requirement: CLI command surface
The parser SHALL support these top-level commands and aliases:
- `init`
- `list` with alias `ls`
- `new <title>`
- `check` with alias `validate`
- `index`

#### Scenario: List alias resolves to list command
- **WHEN** a user runs `adr ls`
- **THEN** the CLI executes the same listing workflow as `adr list`

#### Scenario: Validate alias resolves to check command
- **WHEN** a user runs `adr validate`
- **THEN** the CLI executes the same validation workflow as `adr check`

### Requirement: CLI flag parsing
The parser SHALL support:
- `adr check --format json`
- `adr index --check`

#### Scenario: Check JSON format flag is accepted
- **WHEN** a user runs `adr check --format json`
- **THEN** the CLI executes the check command with JSON output formatting

#### Scenario: Index check flag is accepted
- **WHEN** a user runs `adr index --check`
- **THEN** the CLI executes the index check workflow

### Requirement: Missing or unknown top-level command usage
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

### Requirement: Unexpected arguments on known commands
When a known subcommand receives an unexpected argument or flag, the CLI MUST print `Error: unexpected argument '<value>'` to standard error and exit with status code `2`.

#### Scenario: List command rejects unexpected flags
- **WHEN** a user runs `adr list --foo`
- **THEN** the CLI prints `Error: unexpected argument '--foo'` to standard error
- **AND** exits with status code `2`

#### Scenario: Init command rejects unexpected positional arguments
- **WHEN** a user runs `adr init foo`
- **THEN** the CLI prints `Error: unexpected argument 'foo'` to standard error
- **AND** exits with status code `2`

#### Scenario: Check command rejects unexpected flags
- **WHEN** a user runs `adr check --foo`
- **THEN** the CLI prints `Error: unexpected argument '--foo'` to standard error
- **AND** exits with status code `2`

#### Scenario: Index command rejects unexpected flags
- **WHEN** a user runs `adr index --foo`
- **THEN** the CLI prints `Error: unexpected argument '--foo'` to standard error
- **AND** exits with status code `2`

### Requirement: New command parser validation
The parser SHALL require exactly one title token for `adr new`.

#### Scenario: Missing title is rejected
- **WHEN** a user runs `adr new` without a title
- **THEN** the CLI prints `Error: title is required` to standard error
- **AND** exits with status code `1`

#### Scenario: Extra title tokens are rejected
- **WHEN** a user runs `adr new Use SQLite`
- **THEN** the CLI prints `Error: unexpected extra arguments` to standard error
- **AND** exits with status code `1`

### Requirement: Check format parser validation
The parser SHALL accept only `json` as the value for `adr check --format`.

#### Scenario: Missing format value is rejected
- **WHEN** a user runs `adr check --format` without a value
- **THEN** the CLI prints `Error: --format requires a value` to standard error
- **AND** exits with status code `2`

#### Scenario: Unsupported format value is rejected
- **WHEN** a user runs `adr check --format xml`
- **THEN** the CLI prints `Error: unsupported format 'xml'` to standard error
- **AND** exits with status code `2`

### Requirement: Deferred help and version output
The parser MUST NOT expose comprehensive `--help` or `--version` output in this change.

Comprehensive help and version behavior are tracked separately and MUST remain out of scope until their dedicated changes land.

#### Scenario: Built-in help flag is not enabled
- **WHEN** a user runs `adr --help`
- **THEN** the CLI does not print comprehensive command help output
