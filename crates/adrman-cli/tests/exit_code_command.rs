mod support;

use predicates::prelude::predicate;
use support::CliTestWorkspace;

const VALID_ADR: &str = "# Valid ADR\n\n## Status\n\nAccepted\n\n## Context\n\nContext text.\n\n## Decision\n\nDecision text.\n\n## Consequences\n\nConsequence text.\n";

// Exit code 0 — success

#[test]
fn init_exits_successfully() {
    let workspace = CliTestWorkspace::new();
    let output = workspace.run(&["init"]);
    assert_eq!(output.status_code(), Some(0));
}

#[test]
fn init_exits_successfully_when_template_already_exists() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["init"]);
    assert_eq!(output.status_code(), Some(0));
}

#[test]
fn list_exits_successfully_when_adr_directory_is_missing() {
    let workspace = CliTestWorkspace::new();
    let output = workspace.run(&["list"]);
    assert_eq!(output.status_code(), Some(0));
}

#[test]
fn check_exits_successfully_with_valid_adrs() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-valid.md", VALID_ADR);

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(0));
}

#[test]
fn index_check_exits_successfully_with_fresh_index() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");

    let generate_output = workspace.run(&["index"]);
    assert_eq!(generate_output.status_code(), Some(0));

    let check_output = workspace.run(&["index", "--check"]);
    assert_eq!(check_output.status_code(), Some(0));
}

// Exit code 1 — command outcome failure

#[test]
fn check_exits_with_failure_for_invalid_adrs() {
    let workspace = CliTestWorkspace::new();
    workspace.write_file("docs/adr/notes.md", "# Notes\n");

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(1));
}

#[test]
fn index_check_exits_with_failure_when_index_is_missing() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");

    let output = workspace.run(&["index", "--check"]);
    assert_eq!(output.status_code(), Some(1));
}

#[test]
fn index_check_exits_with_failure_when_index_is_stale() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");
    workspace.run(&["index"]);
    workspace.write_adr("0002-second.md", "# Second\n\n## Status\n\nAccepted\n");

    let output = workspace.run(&["index", "--check"]);
    assert_eq!(output.status_code(), Some(1));
}

#[test]
fn new_exits_with_failure_without_template() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["new", "Use SQLite for local cache"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stderr(predicate::str::contains(
        "docs/adr/.adr-template.md is missing",
    ));
}

#[test]
fn new_exits_with_failure_when_slug_is_empty() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new", "!!!"]);
    assert_eq!(output.status_code(), Some(1));
}

// Exit code 2 — CLI usage error

#[test]
fn no_command_exits_with_usage_error() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&[]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("Usage: adr <COMMAND>"));
}

#[test]
fn unknown_command_exits_with_usage_error() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["nope"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("Usage: adr <COMMAND>"));
}

#[test]
fn invalid_root_flag_exits_with_usage_error() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["--bad"]);
    assert_eq!(output.status_code(), Some(2));
}

#[test]
fn new_without_title_exits_with_usage_error() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("title is required"));
}

#[test]
fn new_with_extra_title_tokens_exits_with_usage_error() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new", "One", "Two"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("unexpected extra arguments"));
}

#[test]
fn check_unsupported_format_exits_with_usage_error() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["check", "--format", "xml"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("unsupported format 'xml'"));
}

#[test]
fn index_invalid_flag_exits_with_usage_error() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["index", "--bad"]);
    assert_eq!(output.status_code(), Some(2));
}
