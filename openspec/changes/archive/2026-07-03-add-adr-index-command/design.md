## Context

adrman already discovers ADRs in `docs/adr/` and extracts ID, status, and title for `adr list`. There is no repository-local index file, so decision history is harder to browse in Git hosting UIs and documentation without manual README maintenance.

## Goals / Non-Goals

**Goals:**
- Add `adr index` to write `docs/adr/README.md`.
- Reuse `list_adrs` for discovery and metadata extraction.
- Emit a deterministic Markdown table with ID, Status, Title, and a relative file link.
- Add `adr index --check` for CI verification of index freshness.
- Return exit code `0` on successful generation or up-to-date check, and non-zero on failure.

**Non-Goals:**
- No configurable index path or custom templates.
- No JSON output mode.
- No ADR validation beyond list extraction rules.
- No preservation of manual edits inside generated sections.

## Decisions

1. Keep index generation in `adrman-core` and CLI dispatch in `adrman-cli`.
   - Rationale: mirrors `list_adrs` and `check_adrs` separation and enables focused unit tests.
   - Alternative considered: implement only in CLI. Rejected because formatting and comparison logic need reusable tests.

2. Reuse `list_adrs` as the single source of ADR rows.
   - Rationale: guarantees identical discovery, metadata extraction, and ordering with `adr list`.
   - Alternative considered: duplicate discovery logic. Rejected to avoid drift.

3. Write a fixed Markdown document shape:
   - Title line: `# Architectural Decision Records`
   - Blank line
   - Table header: `| ID | Status | Title | ADR |`
   - Separator: `| --- | --- | --- | --- |`
   - One row per ADR: `| {id} | {status} | {title} | [{file}]({file}) |`
   - Trailing newline at end of file
   - Rationale: stable, readable on Git hosting sites, and easy to compare in `--check`.
   - Alternative considered: HTML index. Rejected as less Git-friendly.

4. Compare full generated content byte-for-byte in `--check`.
   - Rationale: simplest deterministic stale detection for CI.
   - Alternative considered: compare only table body. Rejected as more complex with little benefit.

5. Fail when `docs/adr/` is missing for both generate and check modes.
   - Rationale: index generation requires an ADR workspace; CI should fail clearly when absent.
   - Alternative considered: warn and succeed like `adr list`. Rejected because index output would be misleading.

6. Print the written path on successful generation and a concise message on successful check.
   - Rationale: matches `adr new` path output and gives CI logs a clear success signal.
   - Alternative considered: silent success. Rejected as harder to debug in automation.

## Risks / Trade-offs

- [Risk] Manual edits to `docs/adr/README.md` are overwritten by `adr index`. → Mitigation: document that the file is generated and use `--check` in CI.
- [Risk] Titles containing `|` break Markdown tables. → Mitigation: escape pipe characters in table cells in a deterministic way.
- [Risk] Empty ADR directory produces an index with header only. → Mitigation: accept empty table as valid generated output and test it.
