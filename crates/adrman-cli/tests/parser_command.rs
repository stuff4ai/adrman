mod support;

use predicates::prelude::predicate;
use support::CliTestWorkspace;

#[test]
fn ls_alias_matches_list_command() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");

    let list_output = workspace.run(&["list"]);
    let ls_output = workspace.run(&["ls"]);

    list_output.assert_success();
    ls_output.assert_success();
    assert_eq!(list_output.stdout_str(), ls_output.stdout_str());
}

#[test]
fn validate_alias_matches_check_command() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr(
        "0001-valid.md",
        "# Valid ADR\n\n## Status\n\nAccepted\n\n## Context\n\nContext.\n\n## Decision\n\nDecision.\n\n## Consequences\n\nConsequences.\n",
    );

    let check_output = workspace.run(&["check"]);
    let validate_output = workspace.run(&["validate"]);

    check_output.assert_success();
    validate_output.assert_success();
    assert_eq!(check_output.stdout_str(), validate_output.stdout_str());
}

#[test]
fn check_format_json_flag_is_accepted() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr(
        "0001-valid.md",
        "# Valid ADR\n\n## Status\n\nAccepted\n\n## Context\n\nContext.\n\n## Decision\n\nDecision.\n\n## Consequences\n\nConsequences.\n",
    );

    let output = workspace.run(&["check", "--format", "json"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("\"valid\":true"));
}

#[test]
fn index_check_flag_is_accepted() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");

    let generate_output = workspace.run(&["index"]);
    generate_output.assert_success();

    let check_output = workspace.run(&["index", "--check"]);
    check_output.assert_success();
    check_output.assert_stdout(predicate::str::contains("up to date"));
}

#[test]
fn unknown_command_exits_with_usage() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["unknown"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("Usage: adr <COMMAND>"));
    output.assert_stderr(predicate::str::contains("init"));
    output.assert_stderr(predicate::str::contains("list, ls"));
}

#[test]
fn invalid_check_flag_exits_with_error() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["check", "--foo"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("unexpected argument '--foo'"));
}

#[test]
fn invalid_index_flag_exits_with_error() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["index", "--foo"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("unexpected argument '--foo'"));
}

#[test]
fn list_rejects_unexpected_flags() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["list", "--foo"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("unexpected argument '--foo'"));
}

#[test]
fn init_rejects_unexpected_positional_arguments() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["init", "foo"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("unexpected argument 'foo'"));
}

#[test]
fn missing_subcommand_exits_with_usage() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&[]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("Usage: adr <COMMAND>"));
}
