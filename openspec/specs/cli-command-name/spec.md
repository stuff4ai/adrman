# cli-command-name Specification

## Purpose
Define the binary command name used to invoke adrman.
## Requirements
### Requirement: CLI binary naming
The distributed CLI binary MUST be named `adr`.

#### Scenario: User runs the CLI binary
- **WHEN** the CLI is built from the Cargo workspace
- **THEN** the binary command name is `adr`
