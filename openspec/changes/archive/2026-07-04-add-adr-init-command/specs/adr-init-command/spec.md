## ADDED Requirements

### Requirement: ADR init command
The CLI SHALL provide `adr init` to bootstrap an ADR workspace in the current repository.

#### Scenario: User initializes a new ADR workspace
- **WHEN** a user runs `adr init` and `docs/adr/.adr-template.md` does not exist
- **THEN** the CLI creates `docs/adr/` if needed
- **AND** creates `docs/adr/.adr-template.md` with the default template content
- **AND** prints a clear success message
- **AND** exits with status code `0`

### Requirement: Default ADR template content
The generated `docs/adr/.adr-template.md` MUST contain YAML front metadata with `filename: "XXXX-{slug}.md"` followed by the standard Nygard section headings.

The file content MUST be exactly:

```markdown
---
filename: "XXXX-{slug}.md"
---

# Title

## Status

## Context

## Decision

## Consequences
```

#### Scenario: Generated template matches expected content
- **WHEN** a user runs `adr init` in a repository without a template
- **THEN** the created `docs/adr/.adr-template.md` content matches the expected template exactly

### Requirement: Missing parent directory handling
If `docs/adr/` does not exist, `adr init` MUST create the directory before creating the template.

#### Scenario: Init creates missing parent directories
- **WHEN** a user runs `adr init` and `docs/adr/` does not exist
- **THEN** the CLI creates `docs/adr/`
- **AND** creates `docs/adr/.adr-template.md`

### Requirement: Existing template protection
If `docs/adr/.adr-template.md` already exists, `adr init` MUST NOT overwrite it.

#### Scenario: Init does not overwrite an existing template
- **WHEN** a user runs `adr init` and `docs/adr/.adr-template.md` already exists
- **THEN** the CLI leaves the existing file unchanged
- **AND** prints a clear message that the template already exists
- **AND** exits with status code `0`
