mod support;

use predicates::prelude::predicate;
use std::fs;
use support::CliTestWorkspace;

#[test]
fn index_generates_readme_with_linked_table_rows() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");
    workspace.write_adr("0002-second.md", "# Second\n\n## Status\n\nProposed\n");

    let output = workspace.run(&["index"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("docs/adr/README.md"));

    let readme = fs::read_to_string(workspace.path().join("docs/adr/README.md"))
        .expect("README should be created");
    assert!(readme.starts_with("# Architectural Decision Records\n\n"));
    assert!(readme.contains("| 0001 | Accepted | First | [0001-first.md](0001-first.md) |"));
    assert!(readme.contains("| 0002 | Proposed | Second | [0002-second.md](0002-second.md) |"));
}

#[test]
fn index_rows_are_sorted_like_list_command() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("10-zeta.md", "# Zeta\n\n## Status\n\nAccepted\n");
    workspace.write_adr("0002-alpha.md", "# Alpha\n\n## Status\n\nAccepted\n");
    workspace.write_adr("2-beta.md", "# Beta\n\n## Status\n\nAccepted\n");

    let output = workspace.run(&["index"]);
    output.assert_success();

    let readme = fs::read_to_string(workspace.path().join("docs/adr/README.md"))
        .expect("README should be created");
    let alpha_pos = readme
        .find("0002-alpha.md")
        .expect("alpha row should exist");
    let beta_pos = readme.find("2-beta.md").expect("beta row should exist");
    let zeta_pos = readme.find("10-zeta.md").expect("zeta row should exist");
    assert!(alpha_pos < beta_pos);
    assert!(beta_pos < zeta_pos);
}

#[test]
fn check_succeeds_when_index_is_up_to_date() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");

    let generate_output = workspace.run(&["index"]);
    generate_output.assert_success();

    let check_output = workspace.run(&["index", "--check"]);
    check_output.assert_success();
    check_output.assert_stdout(predicate::str::contains("up to date"));
}

#[test]
fn check_fails_when_index_is_missing() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");

    let output = workspace.run(&["index", "--check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stderr(predicate::str::contains("missing"));
}

#[test]
fn check_fails_when_index_is_stale() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");
    workspace.write_file(
        "docs/adr/README.md",
        "# Architectural Decision Records\n\n| ID | Status | Title | ADR |\n| --- | --- | --- | --- |\n",
    );

    let output = workspace.run(&["index", "--check"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stderr(predicate::str::contains("stale"));
}

#[test]
fn missing_adr_directory_fails_for_generate_and_check() {
    let workspace = CliTestWorkspace::new();

    let generate_output = workspace.run(&["index"]);
    assert_eq!(generate_output.status_code(), Some(1));
    generate_output.assert_stderr(predicate::str::contains("does not exist"));

    let check_output = workspace.run(&["index", "--check"]);
    assert_eq!(check_output.status_code(), Some(1));
    check_output.assert_stderr(predicate::str::contains("does not exist"));
}
