## Context

adrman already lists ADRs from `docs/adr/` with shared filename and metadata parsing rules in `adrman-core`. The `adr new` command adds the first write workflow and must reuse those discovery conventions so ID assignment stays consistent with `adr list`.

The command depends on an existing ADR workspace (`docs/adr/` and `docs/adr/.adr-template.md`). It does not bootstrap directories or templates.

## Goals / Non-Goals

**Goals:**
- Add `adr new "<title>"` with required title argument.
- Reuse ADR filename regex `^[0-9]+[-_ ].*\.md$` for next-ID discovery.
- Generate deterministic lowercase ASCII slugs from titles.
- Populate new files from `.adr-template.md` with `Proposed` status.
- Fail clearly on missing workspace/template, empty slug, or existing target file.
- Print created file path on success.

**Non-Goals:**
- No config, JSON output, search, validation, index generation, lifecycle updates, or relationship handling.
- No ADR directory or template bootstrapping.

## Decisions

1. Keep write logic in `adrman-core` and CLI dispatch in `adrman-cli`.
   - Rationale: mirrors `list_adrs` separation and keeps filesystem rules testable without subprocesses.
   - Alternative considered: implement entirely in CLI. Rejected because ID/slug/template rules need focused unit tests.

2. Reuse existing ADR filename matching for next-ID discovery.
   - Rationale: highest numeric prefix plus one matches the accepted PRD and stays aligned with `adr list`.
   - Alternative considered: scan title headings for IDs. Rejected as less predictable and more expensive.

3. Treat missing `docs/adr/` as missing template.
   - Rationale: template path lives inside the directory; one clear failure path avoids partial bootstrapping.
   - Alternative considered: create directory then fail on template. Rejected by PRD review.

4. Use simple line-based template substitution.
   - Rationale: matches current `.adr-template.md` shape and keeps behavior explicit.
   - Alternative considered: full Markdown parser. Rejected as unnecessary dependency/complexity.

5. Print repo-relative created path on success.
   - Rationale: stable output for humans, agents, and tests running from repository root.
   - Alternative considered: absolute paths only. Rejected because relative paths are easier to copy into Git workflows.

## Risks / Trade-offs

- [Risk] Slug collisions from different titles normalize to the same filename. -> Mitigation: overwrite protection fails safely; users can choose a different title.
- [Risk] Template edits that remove exact `# Title` line may leave placeholder text. -> Mitigation: document first-line replacement rule and test against current template.
- [Risk] Four-digit IDs overflow after `9999`. -> Mitigation: acceptable for current scope; revisit if repositories exceed that count.
