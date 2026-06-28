# AGENTS.md

**adrman** is a Git-first, agent-friendly CLI tool for managing [Architectural (Any) Decision Records](https://adr.github.io/).

## Project rules

- Keep changes small and reviewable.
- Prefer plain files and Git-friendly workflows.
- Do not introduce unrelated changes.
- Do not push directly to `main`.
- Do not merge pull requests.
- Do not publish releases.
- Update documentation when behavior or workflow changes.

## Change workflow

Use GitHub Flow:

1. Create a short-lived branch from `main`.
2. Make a focused change.
3. Open a pull request.
4. Wait for human review before merging.

Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages.
