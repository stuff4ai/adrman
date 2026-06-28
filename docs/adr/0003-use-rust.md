# Use Rust

## Status

Accepted

## Context

The project needs to create, read, validate, update,
and list decision records in a way that works for humans and AI agents.

The project also needs a simple implementation structure
that separates command-line behavior from reusable decision-record logic.

## Decision

We will implement adrman in Rust.

We will use Cargo with the stable Rust toolchain.

We will start with a Cargo workspace containing:

- `crates/adrman-cli` for the command-line interface
- `crates/adrman-core` for reusable decision-record logic

## Consequences

adrman will be built as a native command-line tool.

The CLI and core logic can evolve separately while staying in one repository.

Rust adds some learning and setup cost, so the project should keep the initial code and commands simple.
