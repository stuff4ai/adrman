## ADDED Requirements

### Requirement: Check format flag validation
The `adr check --format` flag SHALL accept only `json` as a value.

#### Scenario: Missing format value is rejected
- **WHEN** a user runs `adr check --format` without a value
- **THEN** the CLI prints `Error: --format requires a value` to standard error
- **AND** exits with status code `2`

#### Scenario: Unsupported format value is rejected
- **WHEN** a user runs `adr check --format xml`
- **THEN** the CLI prints `Error: unsupported format 'xml'` to standard error
- **AND** exits with status code `2`
