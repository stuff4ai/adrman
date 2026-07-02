mod support;

use predicates::prelude::predicate;
use support::CliTestWorkspace;

const VALID_ADR: &str = "# Valid ADR\n\n## Status\n\nAccepted\n\n## Context\n\nContext text.\n\n## Decision\n\nDecision text.\n\n## Consequences\n\nConsequence text.\n";

#[test]
fn check_and_validate_commands_are_identical() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-valid.md", VALID_ADR);

    let check_output = workspace.run(&["check"]);
    let validate_output = workspace.run(&["validate"]);

    check_output.assert_success();
    validate_output.assert_success();
    assert_eq!(check_output.stdout_str(), validate_output.stdout_str());
    assert_eq!(check_output.stderr_str(), validate_output.stderr_str());
}

#[test]
fn valid_adrs_exit_successfully_with_human_output() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-valid.md", VALID_ADR);

    let output = workspace.run(&["check"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("All ADRs in docs/adr/ are valid."));
}

#[test]
fn invalid_filename_exits_with_failure() {
    let workspace = CliTestWorkspace::new();
    workspace.write_file("docs/adr/notes.md", "# Notes\n");

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stdout(predicate::str::contains("invalid_filename"));
    output.assert_stdout(predicate::str::contains("notes.md"));
}

#[test]
fn duplicate_ids_exit_with_failure() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0002-alpha.md", VALID_ADR);
    workspace.write_adr("2-beta.md", VALID_ADR);

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stdout(predicate::str::contains("duplicate_id"));
}

#[test]
fn missing_section_exits_with_failure() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr(
        "0001-missing-context.md",
        "# Missing Context\n\n## Status\n\nAccepted\n\n## Decision\n\nDecision.\n\n## Consequences\n\nConsequences.\n",
    );

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stdout(predicate::str::contains("missing_section"));
    output.assert_stdout(predicate::str::contains("Context"));
}

#[test]
fn empty_section_exits_with_failure() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr(
        "0001-empty-decision.md",
        "# Empty Decision\n\n## Status\n\nAccepted\n\n## Context\n\nContext.\n\n## Decision\n\n## Consequences\n\nConsequences.\n",
    );

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stdout(predicate::str::contains("empty_section"));
    output.assert_stdout(predicate::str::contains("Decision"));
}

#[test]
fn invalid_status_exits_with_failure() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr(
        "0001-invalid-status.md",
        "# Invalid Status\n\n## Status\n\nDraft\n\n## Context\n\nContext.\n\n## Decision\n\nDecision.\n\n## Consequences\n\nConsequences.\n",
    );

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stdout(predicate::str::contains("invalid_status"));
}

#[test]
fn missing_adr_directory_exits_with_failure() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stdout(predicate::str::contains("does not exist"));
}

#[test]
fn json_output_reports_success() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-valid.md", VALID_ADR);

    let output = workspace.run(&["check", "--format", "json"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("\"valid\":true"));
    output.assert_stdout(predicate::str::contains("\"issues\":[]"));
}

#[test]
fn json_output_reports_failure() {
    let workspace = CliTestWorkspace::new();
    workspace.write_file("docs/adr/notes.md", "# Notes\n");

    let output = workspace.run(&["check", "--format", "json"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stdout(predicate::str::contains("\"valid\":false"));
    output.assert_stdout(predicate::str::contains("\"code\":\"invalid_filename\""));
}
