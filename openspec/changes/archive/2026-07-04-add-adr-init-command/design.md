## Context

adrman already depends on `docs/adr/.adr-template.md` for `adr new`. The `adr init` command is the complementary bootstrap workflow for repositories that do not yet have an ADR workspace.

## Goals / Non-Goals

**Goals:**
- Add `adr init` with no arguments.
- Create `docs/adr/` when missing.
- Create `docs/adr/.adr-template.md` with deterministic content when missing.
- Refuse to overwrite an existing template file.
- Print clear stdout messages for created and already-existing outcomes.
- Exit with status code `0` for both success paths.

**Non-Goals:**
- No config, JSON output, or template customization flags.
- No changes to existing templates in repositories that already have one.
- No automatic index generation.

## Decisions

1. Keep initialization logic in `adrman-core` and CLI dispatch in `adrman-cli`.
   - Rationale: mirrors other commands and keeps filesystem behavior unit-testable.
   - Alternative considered: implement entirely in CLI. Rejected because template content and create-if-absent rules need focused tests.

2. Use `create_new` semantics for the template file.
   - Rationale: matches `adr new` overwrite protection and makes "already exists" explicit.
   - Alternative considered: read-then-compare content. Rejected as unnecessary for v1.

3. Store the default template content as a single constant in `adrman-core`.
   - Rationale: one source of truth for CLI output and tests.
   - Alternative considered: embed a workspace file. Rejected because init must work in empty repositories.

4. Treat "already exists" as success with an informative message.
   - Rationale: init is idempotent for repeated runs in CI and agent workflows.
   - Alternative considered: non-zero exit when template exists. Rejected because existing template is a valid end state.

## Risks / Trade-offs

- [Risk] Init template shape differs from long-form templates in mature repositories. -> Mitigation: do not overwrite existing templates; `adr new` continues using whatever template is present.
- [Risk] Front metadata is not yet consumed by `adr new`. -> Mitigation: document as bootstrap metadata for future tooling; out of scope for this change.
