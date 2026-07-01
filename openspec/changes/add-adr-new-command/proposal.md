## Why

adrman can list existing ADRs but cannot create new decision records from the CLI. Adding `adr new` establishes the first write workflow for ADRs and lets humans and agents bootstrap decision history without hand-editing filenames, IDs, or template content.

## What Changes

- Add a new CLI command: `adr new "<title>"`.
- Require a title argument; reject invocations without one.
- Create new ADR files in `docs/adr/`, creating the directory when missing.
- Discover the next ADR ID from existing ADR filenames only:
  - Include only files in `docs/adr/` matching `^[0-9]+[-_ ].*\.md$`.
  - Ignore templates and other markdown files such as `.adr-template.md` and `notes.md`.
  - Use the highest numeric filename prefix plus one.
  - Format new IDs with four-digit zero padding (for example `0005`).
- Build filenames from the assigned ID and a slug generated from the title:
  - Lowercase the title.
  - Replace non-alphanumeric runs with single hyphens.
  - Trim leading and trailing hyphens.
  - Fail when the title cannot produce a slug.
  - Use `<id>-<slug>.md` (for example `0005-use-sqlite-for-local-cache.md`).
- Require `docs/adr/.adr-template.md`; fail without creating a file when it is missing.
- Populate new files from the template by replacing `# Title` with the provided title and setting initial status to `Proposed`.
- Refuse to overwrite an existing target file.
- Print the created file path on success.
- Non-goals for this change:
  - No config support.
  - No JSON output.
  - No search.
  - No validation/check command.
  - No index generation.
  - No lifecycle or status update commands.
  - No relationship handling.

## Capabilities

### New Capabilities
- `adr-new-command`: Create a new ADR file from the project template with deterministic ID assignment, slugged filename, and success output.

### Modified Capabilities
- None.

## Impact

- Affected code:
  - `adrman-cli` command parsing and dispatch for the new `new` subcommand.
  - ADR ID discovery, slug generation, template rendering, and file creation logic (likely in `adrman-core`).
  - CLI error handling for missing title, missing template, empty slug, existing target files, and filesystem failures.
- Documentation:
  - README and user-facing command docs for `adr new` behavior and examples.
