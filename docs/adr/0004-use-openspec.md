# Use OpenSpec for spec-driven development

## Status

Accepted

## Context

The project needs a way to describe planned product behavior before implementation.

ADRs record durable project decisions,
but they are not a good place for detailed feature proposals, requirements, designs, and implementation tasks.

AI agents also need structured project context before changing code.

## Decision

We will use spec-driven development with OpenSpec for planned product changes.

OpenSpec will be used to describe proposals, requirements, designs, and implementation tasks before code changes.

ADRs remain the durable decision history for important project decisions.

OpenSpec changes should stay small, focused, and reviewable.

## Consequences

Feature work can be discussed and reviewed before implementation starts.

Agents can use OpenSpec artifacts as structured context for proposed behavior and implementation tasks.

The project gains another workflow layer, so OpenSpec should not be used for tiny documentation-only changes.
