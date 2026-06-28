# Use GitHub Flow

## Status

Accepted

## Context

The project needs a simple collaboration workflow for small, reviewable changes.

The workflow should work well for humans and AI agents
without requiring long-lived branches or complex release processes.

## Decision

We will use [GitHub Flow](https://docs.github.com/en/get-started/using-github/github-flow/).

Changes will be made on short-lived branches created from `main`, opened as pull requests, and reviewed before merging.

## Consequences

Changes stay focused, visible, and reviewable.

The `main` branch remains protected from direct changes.

Small changes may take longer because they require a branch, pull request, and review before merge.
