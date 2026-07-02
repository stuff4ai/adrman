## Context

adrman already discovers ADRs in `docs/adr/` and parses title and status for `adr list`. The `adr new` command creates files with required Nygard sections from the template. There is no validation path for existing ADRs, so malformed files can accumulate unnoticed until humans or agents read them.

## Goals / Non-Goals

**Goals:**
- Add `adr check` and `adr validate` with identical behavior.
- Reuse existing ADR filename regex `^[0-9]+[-_ ].*\.md$` for included ADR files.
- Flag markdown files in `docs/adr/` that are not the template and do not match the ADR filename pattern.
- Detect duplicate numeric IDs across included ADR files.
- Validate required sections (`Status`, `Context`, `Decision`, `Consequences`) for presence and non-empty content.
- Validate status values against the supported set.
- Provide human-readable output by default and `--format json` for agents and CI.
- Return exit code `0` on success and non-zero on validation failure.

**Non-Goals:**
- No config for custom directories, templates, statuses, or section names.
- No relationship, supersession link, or title/filename consistency checks.
- No auto-fix or formatting suggestions.

## Decisions

1. Keep validation logic in `adrman-core` and CLI dispatch in `adrman-cli`.
   - Rationale: mirrors `list_adrs` and `create_new_adr` separation and enables focused unit tests.
   - Alternative considered: implement only in CLI. Rejected because section and ID rules need reusable tests.

2. Reuse existing filename and ID parsing helpers.
   - Rationale: keeps discovery aligned with `adr list` and `adr new`.
   - Alternative considered: duplicate parsing rules in a new module. Rejected to avoid drift.

3. Treat non-matching markdown files in `docs/adr/` as invalid filenames, excluding `docs/adr/.adr-template.md`.
   - Rationale: surfaces stray markdown that will never appear in listings while ignoring the template file.
   - Alternative considered: silently ignore non-matching files like `adr list`. Rejected because validation should be strict.

4. Detect duplicate IDs by numeric `sort_id` from filename prefixes.
   - Rationale: matches how `adr list` orders records and catches `0002-foo.md` plus `2-bar.md`.
   - Alternative considered: compare raw ID text only. Rejected because unequal text can represent the same number.

5. Parse section content with exact `## <Section>` heading matches and first non-empty body line rules consistent with status extraction.
   - Rationale: matches current template and list parsing conventions without adding a Markdown parser.
   - Alternative considered: fuzzy heading matching. Rejected as ambiguous.

6. Require exact status strings: `Proposed`, `Accepted`, `Rejected`, `Deprecated`, `Superseded`.
   - Rationale: matches template guidance and `adr new` initial status.
   - Alternative considered: case-insensitive matching. Rejected to keep first version explicit.

7. Fail validation when `docs/adr/` is missing.
   - Rationale: CI and agents need a clear failure when the ADR workspace is absent.
   - Alternative considered: warn and succeed like `adr list`. Rejected for a strict validation command.

8. Emit structured JSON with a top-level `valid` boolean and an `issues` array of objects containing `file`, `code`, and `message`.
   - Rationale: simple machine-readable contract for agents without over-specifying formatting.
   - Alternative considered: mirror human text in JSON. Rejected as harder to consume programmatically.

## Risks / Trade-offs

- [Risk] Legacy ADRs with lowercase statuses fail validation. → Mitigation: document supported statuses; users can normalize in a follow-up change.
- [Risk] Section parsing misses non-standard heading levels. → Mitigation: align with current template and test against real `docs/adr/` files.
- [Risk] Duplicate-ID detection may flag intentional historical duplicates. → Mitigation: validation reports both files involved so users can rename or renumber.
