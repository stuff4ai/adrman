# adr-new-command Specification

## Purpose
Provide a CLI workflow to create new ADR files in `docs/adr/` from a title, using the project template, deterministic ID assignment, and slugged filenames.
## Requirements
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

### Requirement: ADR workspace prerequisite
The ADR creation workflow SHALL write files to `docs/adr/` only when `docs/adr/` and `docs/adr/.adr-template.md` already exist.

The command MUST NOT create `docs/adr/` when it is missing and MUST NOT create or bootstrap `docs/adr/.adr-template.md`.

#### Scenario: ADR directory is missing
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and `docs/adr/` does not exist
- **THEN** the CLI reports that `docs/adr/.adr-template.md` is missing
- **AND** exits with a non-zero status code
- **AND** does not create `docs/adr/`
- **AND** does not create a new ADR file

### Requirement: ADR ID discovery scope
The command SHALL determine the next ADR ID only from regular files in `docs/adr/` whose base filename matches `^[0-9]+[-_ ].*\.md$`.

Files that do not match this pattern MUST be ignored for ID discovery, including `docs/adr/.adr-template.md`, `docs/adr/notes.md`, and other non-ADR markdown files.

#### Scenario: Template and non-ADR files are ignored
- **WHEN** `docs/adr/` contains `0004-use-openspec.md`, `.adr-template.md`, and `notes.md`
- **AND** a user runs `adr new "Use SQLite for local cache"`
- **THEN** ID discovery considers only `0004-use-openspec.md`
- **AND** the created file uses ID `0005`

#### Scenario: Unpadded existing IDs use numeric maximum
- **WHEN** `docs/adr/` contains `2-beta.md` and `0004-use-openspec.md`
- **AND** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file uses ID `0005`

### Requirement: ADR ID assignment
The command SHALL assign the next ADR ID as one greater than the highest numeric filename prefix discovered in `docs/adr/`, and MUST format new IDs with four-digit zero padding.

When no qualifying ADR files exist, the first assigned ID MUST be `0001`.

#### Scenario: Repository has existing ADR files
- **WHEN** `docs/adr/` contains ADR files with numeric prefixes up to `0004`
- **AND** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file uses ID `0005`

#### Scenario: Repository has no qualifying ADR files
- **WHEN** `docs/adr/` exists and contains no files matching `^[0-9]+[-_ ].*\.md$`
- **AND** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file uses ID `0001`

#### Scenario: Numeric gaps do not affect next ID
- **WHEN** `docs/adr/` contains only `0001-first.md` and `0004-fourth.md`
- **AND** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file uses ID `0005`

### Requirement: ADR title slug generation
The command SHALL generate a filename slug from the title using these rules:
- Convert the title to lowercase.
- Replace each sequence of characters that are not ASCII letters or digits with a single hyphen.
- Remove leading and trailing hyphens.
- If the resulting slug is empty, the command MUST fail with a non-zero exit code.

#### Scenario: Title is converted to a slugged filename
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and the next ID is `0005`
- **THEN** the created filename is `0005-use-sqlite-for-local-cache.md`

#### Scenario: Punctuation and repeated separators are normalized
- **WHEN** a user runs `adr new "API Design (v2)!!"` and the next ID is `0005`
- **THEN** the created filename is `0005-api-design-v2.md`

#### Scenario: Title cannot produce a slug
- **WHEN** a user runs `adr new "!!!"`
- **THEN** the CLI reports that the title cannot be converted to a slug
- **AND** exits with a non-zero status code
- **AND** does not create a file

### Requirement: ADR filename format
The command SHALL name new ADR files as `<id>-<slug>.md`, where `<id>` is the assigned four-digit ID and `<slug>` is generated from the title.

#### Scenario: New ADR filenames use a hyphen separator
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and the next ID is `0005`
- **THEN** the created path is `docs/adr/0005-use-sqlite-for-local-cache.md`

### Requirement: ADR template requirement
The command MUST read `docs/adr/.adr-template.md` before creating a new ADR file.

If the template file is missing, the command MUST exit with a non-zero status code, report that `docs/adr/.adr-template.md` is missing, and MUST NOT create an ADR file, create `docs/adr/`, or bootstrap template content.

#### Scenario: Template file is missing
- **WHEN** a user runs `adr new "Use SQLite for local cache"` and `docs/adr/` exists but `docs/adr/.adr-template.md` does not exist
- **THEN** the CLI reports that `docs/adr/.adr-template.md` is missing
- **AND** exits with a non-zero status code
- **AND** does not create a new ADR file

### Requirement: ADR template population
The command SHALL create new ADR content from `docs/adr/.adr-template.md` by:
- Replacing the first line that is exactly `# Title` with `# <provided title>`.
- Replacing the first non-empty line after `## Status` with `Proposed`.
- Leaving all other template content unchanged.

#### Scenario: Template placeholders are replaced
- **WHEN** a user runs `adr new "Use SQLite for local cache"`
- **THEN** the created file's first `# ` heading is `Use SQLite for local cache`
- **AND** the `## Status` section content is `Proposed`
- **AND** the remaining template sections are preserved

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

