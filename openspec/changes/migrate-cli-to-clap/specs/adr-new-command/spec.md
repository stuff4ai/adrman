## ADDED Requirements

### Requirement: ADR new title argument shape
The `adr new` command SHALL accept exactly one title token.

Title tokens that begin with `-` MUST be accepted as titles rather than being interpreted as flags.

Additional title tokens MUST be rejected.

#### Scenario: Title token beginning with a hyphen is accepted
- **WHEN** a user runs `adr new --help` and the ADR workspace prerequisites are satisfied
- **THEN** the CLI creates a new ADR using `--help` as the title
- **AND** exits with a success status code

#### Scenario: Extra title tokens are rejected
- **WHEN** a user runs `adr new Use SQLite`
- **THEN** the CLI prints `Error: unexpected extra arguments` to standard error
- **AND** exits with status code `1`
- **AND** does not create a new ADR file
