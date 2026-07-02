## 1. Core ADR creation logic

- [x] 1.1 Add next-ID discovery using existing ADR filename rules in `adrman-core`.
- [x] 1.2 Add title slug generation with empty-slug failure.
- [x] 1.3 Add template population and `create_new_adr` with missing-template and overwrite protection.
- [x] 1.4 Add unit tests for ID discovery, slug generation, template population, and error cases.

## 2. CLI command surface

- [x] 2.1 Add `adr new "<title>"` parsing and dispatch in `adrman-cli`.
- [x] 2.2 Wire success output and error messages for missing title, missing template, empty slug, and existing file.

## 3. CLI integration tests

- [x] 3.1 Add integration tests for success output, missing title, missing workspace/template, overwrite protection, and template population.
- [x] 3.2 Reuse shared CLI test helpers for ADR workspace setup.

## 4. Documentation

- [x] 4.1 Update README with `adr new` command behavior and examples.
