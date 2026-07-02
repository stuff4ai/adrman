# cli-command-name Specification

## Purpose
Define the command name used to invoke adrman.
## Requirements
### Requirement: CLI command name
The distributed CLI command MUST be named `adr`.

#### Scenario: User runs the CLI command
- **WHEN** the CLI is built from the Cargo workspace
- **THEN** the command name is `adr`
