# cli-index Specification

## Purpose
Provide a CLI workflow to generate and verify a Markdown ADR index at `docs/adr/README.md`, including discovery, metadata extraction, deterministic table output, and CI check mode.
## Requirements
### Requirement: ADR index command
The CLI SHALL provide `adr index` to generate or update the ADR index at `docs/adr/README.md`.

#### Scenario: User generates the ADR index
- **WHEN** a user runs `adr index`
- **THEN** the CLI writes `docs/adr/README.md` with the generated index content
- **AND** exits with status code `0`

### Requirement: ADR index discovery and metadata
The index workflow SHALL reuse the same ADR discovery and metadata extraction behavior as `adr list`.

#### Scenario: Index includes discovered ADR files
- **WHEN** `docs/adr/0001-use-openspec.md` exists and matches ADR filename rules
- **THEN** the generated index includes a row for that ADR with extracted ID, Status, and Title

#### Scenario: Index excludes non-matching markdown files
- **WHEN** `docs/adr/notes.md` exists
- **THEN** the generated index does not include a row for `notes.md`

### Requirement: ADR index table format
The generated index SHALL contain a stable Markdown table with columns `ID`, `Status`, `Title`, and `ADR`.

Each `ADR` cell MUST be a relative Markdown link to the ADR filename in the same directory.

Rows MUST be sorted by numeric ID ascending, then by filename ascending.

#### Scenario: Index renders a linked table row
- **WHEN** an ADR file `0001-use-openspec.md` has ID `0001`, status `Accepted`, and title `Use OpenSpec`
- **THEN** the generated index row contains `0001`, `Accepted`, `Use OpenSpec`, and `[0001-use-openspec.md](0001-use-openspec.md)`

#### Scenario: Index ordering matches list ordering
- **WHEN** included ADR files produce IDs `2` and `10`
- **THEN** the row with ID `2` appears before the row with ID `10` in the generated index

### Requirement: Deterministic index output
The index generation workflow MUST produce byte-identical output for the same ADR directory contents.

#### Scenario: Repeated generation is stable
- **WHEN** a user runs `adr index` twice without changing ADR files
- **THEN** the second generated index content is identical to the first

### Requirement: ADR index check mode
The CLI SHALL support `adr index --check` to verify that `docs/adr/README.md` is present and up to date.

#### Scenario: Check succeeds when index is up to date
- **WHEN** a user runs `adr index --check` and `docs/adr/README.md` matches the generated index
- **THEN** the CLI exits with status code `0`

#### Scenario: Check fails when index is missing
- **WHEN** a user runs `adr index --check` and `docs/adr/README.md` does not exist
- **THEN** the CLI exits with a non-zero status code

#### Scenario: Check fails when index is stale
- **WHEN** a user runs `adr index --check` and `docs/adr/README.md` differs from the generated index
- **THEN** the CLI exits with a non-zero status code

### Requirement: Missing ADR directory handling
If `docs/adr/` does not exist, `adr index` MUST report the missing directory and exit with a non-zero status code.

If `docs/adr/` does not exist, `adr index --check` MUST report the missing directory and exit with a non-zero status code.

#### Scenario: Generate fails when ADR directory is absent
- **WHEN** a user runs `adr index` and `docs/adr/` is missing
- **THEN** the CLI reports that the ADR directory is missing
- **AND** exits with a non-zero status code

#### Scenario: Check fails when ADR directory is absent
- **WHEN** a user runs `adr index --check` and `docs/adr/` is missing
- **THEN** the CLI reports that the ADR directory is missing
- **AND** exits with a non-zero status code
