# Use Conventional Commits

## Status

Accepted

## Context

The project needs a clear and consistent commit history.

Consistent commit messages make changes easier to review
and can support future automation such as changelog generation and releases.

## Decision

We will use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages.

Commit messages should use the form `type(scope): description` when a scope is useful,
or `type: description` when no scope is needed.

## Consequences

The project history will be easier to scan and understand.

Future automation can rely on structured commit messages.

Contributors and agents need to follow the commit message format,
which adds a small amount of discipline to each change.
