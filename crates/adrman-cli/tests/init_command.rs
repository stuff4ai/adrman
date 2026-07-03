mod support;

use adrman_core::DEFAULT_ADR_TEMPLATE;
use predicates::prelude::predicate;
use std::fs;
use support::CliTestWorkspace;

#[test]
fn init_creates_adr_template() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["init"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains(
        "Created docs/adr/.adr-template.md",
    ));

    let template_path = workspace.path().join("docs/adr/.adr-template.md");
    assert!(template_path.is_file());
    let content = fs::read_to_string(template_path).expect("template should be readable");
    assert_eq!(content, DEFAULT_ADR_TEMPLATE);
}

#[test]
fn init_creates_missing_parent_directories() {
    let workspace = CliTestWorkspace::new();
    let nested_root = workspace.path().join("nested/project");
    fs::create_dir_all(&nested_root).expect("nested directory should be created");

    let output = workspace.run_in_dir(&nested_root, &["init"]);
    output.assert_success();

    assert!(nested_root.join("docs/adr").is_dir());
    assert!(nested_root.join("docs/adr/.adr-template.md").is_file());
}

#[test]
fn init_does_not_overwrite_existing_template() {
    let workspace = CliTestWorkspace::new();
    workspace.write_adr_template("custom template content");

    let output = workspace.run(&["init"]);
    output.assert_success();
    output.assert_stdout(predicate::str::contains(
        "docs/adr/.adr-template.md already exists",
    ));

    let content = fs::read_to_string(workspace.path().join("docs/adr/.adr-template.md"))
        .expect("template should remain readable");
    assert_eq!(content, "custom template content");
}

#[test]
fn init_generated_template_matches_expected_content_exactly() {
    let workspace = CliTestWorkspace::new();

    let output = workspace.run(&["init"]);
    output.assert_success();

    let content = fs::read_to_string(workspace.path().join("docs/adr/.adr-template.md"))
        .expect("template should be readable");
    assert_eq!(content, DEFAULT_ADR_TEMPLATE);
}
