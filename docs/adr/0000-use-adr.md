# Use ADR

## Status

Accepted

## Context

The project needs a simple way to record important choices
so humans and AI agents can understand why the project works the way it does.

## Decision

We will use [Architectural (Any) Decision Records](https://adr.github.io/) to record important project decisions.

ADRs will be stored as Markdown files in `docs/adr/` and numbered with a four-digit prefix.

We will use the Nygard ADR template with these sections:

- Status
- Context
- Decision
- Consequences

## Consequences

Important project decisions will be easier to find, review, and change through Git.

AI agents will have a clearer source of project context before proposing changes.

Writing ADRs adds a small amount of process, so we should keep records brief and focused.
