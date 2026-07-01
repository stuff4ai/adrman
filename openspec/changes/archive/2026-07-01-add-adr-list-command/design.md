## Context

This is the first user-facing adrman feature and it establishes the baseline UX for ADR discovery through the CLI. The command spans multiple concerns: command alias wiring in `adrman-cli`, ADR filesystem scanning/parsing in `adrman-core`, and output formatting contract stability for humans and agents. The existing repository already commits ADRs as plain Markdown files in `docs/adr/`, and the new behavior must remain deterministic, dependency-light, and Git-friendly.

## Goals / Non-Goals

**Goals:**
- Provide `adr list` with `adr ls` alias as equivalent entry points.
- Rename the CLI binary to `adr` and ensure command docs follow that naming.
- Parse ADR metadata from Markdown files in `docs/adr/` matching `^[0-9]+[-_ ].*\.md$`.
- Produce stable sorted output by numeric ID, then filename.
- Fail soft when `docs/adr/` is absent (warning + success exit).

**Non-Goals:**
- No recursive scan beyond `docs/adr/`.
- No output modes beyond the defined tabular text format.
- No metadata normalization beyond `Unknown` fallback for missing title/status.
- No ADR creation, editing, or linting behavior in this change.

## Decisions

1. Keep all listing input constrained to `docs/adr/` and filename regex filtering.
   - Rationale: aligns with existing ADR convention and prevents accidental ingestion of unrelated Markdown files.
   - Alternative considered: scan all Markdown files and infer ADR shape. Rejected as noisy and less predictable.

2. Extract metadata with simple, explicit parsing rules and fallback to `Unknown`.
   - Rationale: deterministic behavior for incomplete ADRs and straightforward test coverage.
   - Alternative considered: reject malformed ADR files. Rejected because listing should remain informative and non-blocking.

3. Sort entries using `(numeric_id, filename)` before rendering.
   - Rationale: preserves intuitive chronological ordering while keeping deterministic ties for same ID prefixes.
   - Alternative considered: lexicographic filename sort only. Rejected because `10-...` would sort before `2-...`.

4. Keep output as fixed plain-text headings and columns.
   - Rationale: stable human-readable output and easy parseability for agents without adding format flags yet.
   - Alternative considered: add table rendering library or JSON output immediately. Rejected to keep first feature focused and dependency-minimal.

5. Warn and return success if ADR directory is missing.
   - Rationale: allows bootstrapping repos and CI/agent workflows without hard failure when ADRs are not yet initialized.
   - Alternative considered: exit non-zero on missing directory. Rejected as overly strict for first-run and template repositories.

## Risks / Trade-offs

- [Risk] Markdown parsing edge cases (multiple status headings, malformed headings) could produce surprising values. -> Mitigation: document first-match rules and test representative malformed files.
- [Risk] Fixed-width columns may appear uneven with long titles/status text. -> Mitigation: keep contract simple now and revisit formatting improvements in a follow-up capability.
- [Risk] Binary rename to `adr` may affect existing scripts using old invocation. -> Mitigation: update docs and release notes; keep change scoped so follow-up compatibility decisions are explicit.
