mod support;

use predicates::prelude::predicate;
use support::CliTestWorkspace;

#[test]
fn list_and_ls_commands_are_identical() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("0001-first.md", "# First\n\n## Status\n\nAccepted\n");

    let list_output = workspace.run(&["list"]);
    let ls_output = workspace.run(&["ls"]);

    list_output.assert_success();
    ls_output.assert_success();
    assert_eq!(list_output.stdout_str(), ls_output.stdout_str());
    assert_eq!(list_output.stderr_str(), ls_output.stderr_str());
}

#[test]
fn missing_adr_directory_warns_and_exits_successfully() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["list"]);
    output.assert_success();
    output.assert_stderr(predicate::str::contains("Warning: ADR directory"));
    output.assert_stderr(predicate::str::contains("docs/adr"));
}

#[test]
fn rows_are_sorted_and_output_header_matches_contract() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("10-zeta.md", "# Zeta\n\n## Status\n\nAccepted\n");
    workspace.write_adr("0002-alpha.md", "# Alpha\n\n## Status\n\nAccepted\n");
    workspace.write_adr("2-beta.md", "# Beta\n\n## Status\n\nAccepted\n");

    let output = workspace.run(&["list"]);
    output.assert_success();
    let stdout = output.stdout_str();
    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(lines[0], "ADRs (docs/adr/)");
    assert_eq!(lines[1], "");
    assert_eq!(lines[2], "ID    Status    Title    File");
    assert_eq!(lines[3], "0002    Accepted    Alpha    0002-alpha.md");
    assert_eq!(lines[4], "2    Accepted    Beta    2-beta.md");
    assert_eq!(lines[5], "10    Accepted    Zeta    10-zeta.md");
}

#[test]
fn missing_title_and_status_render_unknown() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr(
        "1-title-only.md",
        "# Title Only\n\n## Context\n\nNo status section.\n",
    );
    workspace.write_adr(
        "2-status-only.md",
        "## Status\n\nAccepted\n\n## Context\n\nNo title.\n",
    );

    let output = workspace.run(&["list"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains(
        "1    Unknown    Title Only    1-title-only.md",
    ));
    output.assert_stdout(predicate::str::contains(
        "2    Accepted    Unknown    2-status-only.md",
    ));
}

#[test]
fn filename_variants_are_discovered_with_text_ids_and_sorted() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr("1_foo.md", "# Foo\n\n## Status\n\nAccepted\n");
    workspace.write_adr("01-bar.md", "# Bar\n\n## Status\n\nAccepted\n");
    workspace.write_adr("001 gap.md", "# Gap\n\n## Status\n\nAccepted\n");

    let output = workspace.run(&["list"]);
    output.assert_success();
    let stdout = output.stdout_str();
    let lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(lines[3], "001    Accepted    Gap    001 gap.md");
    assert_eq!(lines[4], "01    Accepted    Bar    01-bar.md");
    assert_eq!(lines[5], "1    Accepted    Foo    1_foo.md");
}

#[test]
fn snapshots_normal_list_output_contract() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr(
        "0001-first.md",
        "# First\n\n## Status\n\nAccepted\n\n## Context\n\nFirst context.\n",
    );
    workspace.write_adr(
        "0002-second.md",
        "# Second\n\n## Status\n\nProposed\n\n## Context\n\nSecond context.\n",
    );
    workspace.write_adr(
        "0003-third.md",
        "# Third\n\n## Status\n\nSuperseded\n\n## Context\n\nThird context.\n",
    );

    let output = workspace.run(&["list"]);
    output.assert_success();

    insta::assert_snapshot!("adr_list_output_contract", output.stdout_str());
}
