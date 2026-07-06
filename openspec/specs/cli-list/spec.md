# cli-list Specification

## Purpose
Provide a CLI workflow to list ADR files from `docs/adr/`, including command aliases, file discovery, metadata extraction, ordering, output format, and missing-directory handling.
## Requirements
### Requirement: ADR list commands
The CLI SHALL provide `adr list` and `adr ls` commands, and both commands MUST execute the same ADR listing behavior.

#### Scenario: User invokes the primary list command
- **WHEN** a user runs `adr list`
- **THEN** the CLI executes the ADR listing workflow and prints ADR table output to standard output

#### Scenario: User invokes the alias list command
- **WHEN** a user runs `adr ls`
- **THEN** the CLI executes the same ADR listing workflow and prints ADR table output with identical semantics to standard output

### Requirement: ADR file discovery scope
The ADR listing workflow SHALL read candidates only from the `docs/adr/` directory and MUST include only files whose base filename matches `^[0-9]+[-_ ].*\.md$`.

#### Scenario: Matching ADR filename is included
- **WHEN** `docs/adr/0001-use-openspec.md` exists
- **THEN** that file is included in ADR list processing

#### Scenario: Non-matching markdown filename is excluded
- **WHEN** `docs/adr/notes.md` exists
- **THEN** that file is excluded from ADR list processing

### Requirement: ADR metadata extraction
For each included ADR file, the list workflow SHALL extract:
- `ID` from the numeric filename prefix
- `Status` from the `## Status` section content
- `Title` from the first `# ` heading
- `File` as the base filename

Missing `Status` or missing `Title` MUST be rendered as `Unknown`.

#### Scenario: ADR with complete metadata
- **WHEN** an ADR file has numeric filename prefix, a `## Status` section value, and a first `# ` heading
- **THEN** output row fields use those extracted values plus the base filename

#### Scenario: ADR missing title
- **WHEN** an included ADR file has no `# ` heading
- **THEN** the output row `Title` is `Unknown`

#### Scenario: ADR missing status
- **WHEN** an included ADR file has no `## Status` section content
- **THEN** the output row `Status` is `Unknown`

### Requirement: Missing ADR directory handling
If `docs/adr/` does not exist, the command MUST print a warning to standard error and exit successfully.

#### Scenario: ADR directory absent
- **WHEN** a user runs `adr list` and `docs/adr/` is missing
- **THEN** the CLI prints a warning about the missing ADR directory to standard error
- **AND** exits with a success status code

### Requirement: ADR listing order and output format
The command SHALL sort output rows by numeric `ID` ascending, then by `File` ascending, and MUST print:
1) header line `ADRs (docs/adr/)`
2) blank line
3) column header line `ID    Status    Title    File`

#### Scenario: Numeric ordering takes precedence over lexical ordering
- **WHEN** included files produce IDs `2` and `10`
- **THEN** the row with `ID=2` appears before the row with `ID=10`

#### Scenario: Filename tiebreaker for equal IDs
- **WHEN** two included files share the same numeric `ID`
- **THEN** rows are ordered by ascending base filename
