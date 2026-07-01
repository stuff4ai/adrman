## ADDED Requirements

### Requirement: ADR new command
The CLI SHALL provide an `adr new` command that creates a new ADR file from a required title argument.

#### Scenario: User creates an ADR with a title
- **WHEN** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the CLI creates a new ADR file in `docs/adr/`
- **AND** prints the created file path on success

#### Scenario: User omits the title argument
- **WHEN** a user runs `adr new` without a title argument
- **THEN** the CLI reports that the title is required
- **AND** exits with a non-zero status code

### Requirement: ADR directory creation
The ADR creation workflow SHALL write files to `docs/adr/` and MUST create `docs/adr/` when it does not exist.

#### Scenario: ADR directory is missing
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and `docs/adr/` does not exist
- **THEN** the CLI creates `docs/adr/`
- **AND** writes the new ADR file inside that directory

### Requirement: ADR ID assignment
The command SHALL assign the next numeric ADR ID after existing ADR files in `docs/adr/` and MUST format new IDs with four-digit zero padding.

#### Scenario: Repository has existing ADR files
- **WHEN** `docs/adr/` contains ADR files with numeric prefixes up to `0004`
- **AND** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file uses ID `0005`

#### Scenario: Repository has no ADR files
- **WHEN** `docs/adr/` exists and contains no ADR files with numeric prefixes
- **AND** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file uses ID `0001`

### Requirement: ADR filename format
The command SHALL name new ADR files using the assigned ID, a hyphen, and a slug generated from the title.

#### Scenario: Title is converted to a slugged filename
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and the next ID is `0005`
- **THEN** the created filename is `0005-use-sqlite-for-local-cache.md`

### Requirement: ADR template population
The command SHALL create new ADR content from `docs/adr/.adr-template.md`, replace `# Title` with the provided title, and set the initial status to `Proposed`.

#### Scenario: Template placeholders are replaced
- **WHEN** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file's first `# ` heading is `Use SQLite for local cache`
- **AND** the `## Status` section content is `Proposed`

### Requirement: ADR overwrite protection
The command MUST NOT overwrite an existing file.

#### Scenario: Target file already exists
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and the computed target path already exists
- **THEN** the CLI reports that the target file already exists
- **AND** exits with a non-zero status code
- **AND** does not modify the existing file

### Requirement: ADR creation success output
On successful creation, the command MUST print the created file path.

#### Scenario: ADR is created successfully
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and file creation succeeds
- **THEN** the CLI prints the path to the created ADR file
- **AND** exits with a success status code
