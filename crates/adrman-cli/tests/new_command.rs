mod support;

use predicates::prelude::predicate;
use std::fs;
use support::CliTestWorkspace;

#[test]
fn new_command_requires_title() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("title is required"));
}

#[test]
fn new_command_creates_adr_and_prints_path() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();
    workspace.write_adr(
        "0004-use-openspec.md",
        "# Use OpenSpec\n\n## Status\n\nAccepted\n",
    );

    let output = workspace.run(&["new", "Use SQLite for local cache"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains(
        "docs/adr/0005-use-sqlite-for-local-cache.md",
    ));

    let created = workspace
        .path()
        .join("docs/adr/0005-use-sqlite-for-local-cache.md");
    assert!(created.is_file());
    let content = fs::read_to_string(created).expect("created adr should be readable");
    assert!(content.starts_with("# Use SQLite for local cache\n\n## Status\n\nProposed\n"));
    assert!(content.contains("## Context\n"));
    assert!(content.contains("## Decision\n"));
}

#[test]
fn new_command_fails_when_workspace_is_missing() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["new", "Use SQLite for local cache"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stderr(predicate::str::contains(
        "docs/adr/.adr-template.md is missing",
    ));
    assert!(!workspace.path().join("docs/adr").exists());
}

#[test]
fn new_command_fails_when_template_is_missing() {
    let workspace = CliTestWorkspace::new();
    workspace.write_file("docs/adr/.keep", "");

    let output = workspace.run(&["new", "Use SQLite for local cache"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stderr(predicate::str::contains(
        "docs/adr/.adr-template.md is missing",
    ));
}

#[test]
fn new_command_fails_when_slug_is_empty() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new", "!!!"]);
    assert_eq!(output.status_code(), Some(1));
    output.assert_stderr(predicate::str::contains(
        "title cannot be converted to a slug",
    ));

    let markdown_files: Vec<_> = fs::read_dir(workspace.path().join("docs/adr"))
        .expect("adr directory should exist")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md") && name != ".adr-template.md")
        .collect();
    assert!(markdown_files.is_empty());
}

#[test]
fn new_command_rejects_extra_arguments() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new", "Use", "SQLite"]);
    assert_eq!(output.status_code(), Some(2));
    output.assert_stderr(predicate::str::contains("unexpected extra arguments"));

    let created = workspace.path().join("docs/adr/0001-use.md");
    assert!(!created.exists());
}

#[test]
fn new_command_assigns_first_id_without_existing_adrs() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new", "First ADR"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("docs/adr/0001-first-adr.md"));
}

#[test]
fn new_command_normalizes_slugged_filename() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();
    workspace.write_adr(
        "0004-use-openspec.md",
        "# Use OpenSpec\n\n## Status\n\nAccepted\n",
    );

    let output = workspace.run(&["new", "API Design (v2)!!"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("docs/adr/0005-api-design-v2.md"));
}

#[test]
fn new_command_accepts_title_token_beginning_with_hyphen() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new", "--help"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("docs/adr/0001-help.md"));

    let created = workspace.path().join("docs/adr/0001-help.md");
    let content = fs::read_to_string(created).expect("created adr should be readable");
    assert!(content.starts_with("# --help\n\n## Status\n\nProposed\n"));
}

#[test]
fn new_command_accepts_title_token_beginning_with_single_hyphen_prefix() {
    let workspace = CliTestWorkspace::new();
    workspace.write_default_adr_template();

    let output = workspace.run(&["new", "-foo"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains("docs/adr/0001-foo.md"));
}
