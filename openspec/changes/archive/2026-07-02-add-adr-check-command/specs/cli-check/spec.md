## ADDED Requirements

### Requirement: ADR check commands
The CLI SHALL provide `adr check` and `adr validate` commands, and both commands MUST execute the same ADR validation behavior.

#### Scenario: User invokes the primary check command
- **WHEN** a user runs `adr check`
- **THEN** the CLI executes the ADR validation workflow

#### Scenario: User invokes the validate alias
- **WHEN** a user runs `adr validate`
- **THEN** the CLI executes the same ADR validation workflow with identical semantics

### Requirement: ADR validation scope
The validation workflow SHALL read candidates from `docs/adr/` only.

Files whose base filename matches `^[0-9]+[-_ ].*\.md$` MUST be validated as ADR content.

Regular files in `docs/adr/` ending in `.md` that do not match the ADR filename pattern MUST be reported as invalid filenames, except `docs/adr/.adr-template.md`.

#### Scenario: Matching ADR filename is validated
- **WHEN** `docs/adr/0001-use-openspec.md` exists with valid content
- **THEN** that file is included in ADR validation

#### Scenario: Non-matching markdown filename is reported
- **WHEN** `docs/adr/notes.md` exists
- **THEN** validation reports an invalid filename issue for `notes.md`

#### Scenario: Template file is excluded from invalid filename checks
- **WHEN** only `docs/adr/.adr-template.md` exists as a non-matching markdown file
- **THEN** validation does not report an invalid filename issue for `.adr-template.md`

### Requirement: Duplicate ADR ID detection
The validation workflow SHALL detect when two or more included ADR files share the same numeric filename prefix value.

#### Scenario: Duplicate numeric IDs are reported
- **WHEN** `docs/adr/0002-alpha.md` and `docs/adr/2-beta.md` both exist
- **THEN** validation reports duplicate ID issues for the involved files

#### Scenario: Distinct numeric IDs pass
- **WHEN** included ADR files have unique numeric filename prefixes
- **THEN** validation does not report duplicate ID issues

### Requirement: Required Nygard sections
For each included ADR file, validation SHALL require these sections with exact `##` headings:
- `Status`
- `Context`
- `Decision`
- `Consequences`

Missing required sections MUST be reported.

#### Scenario: ADR with all required sections passes section presence checks
- **WHEN** an included ADR file contains non-empty `## Status`, `## Context`, `## Decision`, and `## Consequences` sections
- **THEN** validation does not report missing-section issues for that file

#### Scenario: ADR missing a required section is reported
- **WHEN** an included ADR file has no `## Context` section
- **THEN** validation reports a missing-section issue for `Context`

### Requirement: Empty required sections
For each required section, validation SHALL require at least one non-empty line of content before the next heading.

Empty required sections MUST be reported.

#### Scenario: ADR with populated required sections passes
- **WHEN** each required section contains non-empty content before the next heading
- **THEN** validation does not report empty-section issues for that file

#### Scenario: ADR with an empty required section is reported
- **WHEN** an included ADR file contains `## Decision` followed immediately by another heading with no content in between
- **THEN** validation reports an empty-section issue for `Decision`

### Requirement: Supported ADR statuses
The `Status` section content MUST be exactly one of:
- `Proposed`
- `Accepted`
- `Rejected`
- `Deprecated`
- `Superseded`

Any other status value MUST be reported as invalid.

#### Scenario: Supported status passes
- **WHEN** an included ADR file's `## Status` section content is `Accepted`
- **THEN** validation does not report an invalid-status issue for that file

#### Scenario: Unsupported status is reported
- **WHEN** an included ADR file's `## Status` section content is `Draft`
- **THEN** validation reports an invalid-status issue for that file

### Requirement: Human-readable validation output
By default, the command SHALL print human-readable validation results to standard output.

#### Scenario: Successful validation prints confirmation
- **WHEN** a user runs `adr check` and all ADRs are valid
- **THEN** the CLI prints a human-readable success message
- **AND** exits with status code `0`

#### Scenario: Failed validation prints issues
- **WHEN** a user runs `adr check` and validation finds issues
- **THEN** the CLI prints human-readable issue details
- **AND** exits with a non-zero status code

### Requirement: JSON validation output
The command SHALL support `--format json` and MUST print structured JSON to standard output.

The JSON output MUST include:
- a boolean `valid` field
- an `issues` array of issue objects with `file`, `code`, and `message` fields

#### Scenario: JSON output for successful validation
- **WHEN** a user runs `adr check --format json` and all ADRs are valid
- **THEN** the CLI prints JSON with `"valid": true`
- **AND** `"issues": []`
- **AND** exits with status code `0`

#### Scenario: JSON output for failed validation
- **WHEN** a user runs `adr check --format json` and validation finds issues
- **THEN** the CLI prints JSON with `"valid": false`
- **AND** a non-empty `issues` array
- **AND** exits with a non-zero status code

### Requirement: Missing ADR directory handling
If `docs/adr/` does not exist, the command MUST report the missing directory and exit with a non-zero status code.

#### Scenario: ADR directory absent
- **WHEN** a user runs `adr check` and `docs/adr/` is missing
- **THEN** the CLI reports that the ADR directory is missing
- **AND** exits with a non-zero status code
