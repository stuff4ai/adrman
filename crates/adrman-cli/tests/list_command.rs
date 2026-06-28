use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    path.push(format!("{prefix}_{}_{}", std::process::id(), nanos));
    fs::create_dir_all(&path).expect("temp directory should be created");
    path
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent should be created");
    }
    fs::write(path, content).expect("file should be written");
}

fn run_cli(workspace: &Path, command: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adr"))
        .arg(command)
        .current_dir(workspace)
        .output()
        .expect("CLI should execute")
}

#[test]
fn list_and_ls_commands_are_identical() {
    let workspace = unique_temp_dir("adr_cli_alias");
    write_file(
        &workspace.join("docs/adr/0001-first.md"),
        "# First\n\n## Status\n\nAccepted\n",
    );

    let list_output = run_cli(&workspace, "list");
    let ls_output = run_cli(&workspace, "ls");

    assert!(list_output.status.success());
    assert!(ls_output.status.success());
    assert_eq!(list_output.stdout, ls_output.stdout);
    assert_eq!(list_output.stderr, ls_output.stderr);
}

#[test]
fn missing_adr_directory_warns_and_exits_successfully() {
    let workspace = unique_temp_dir("adr_cli_missing_dir");

    let output = run_cli(&workspace, "list");
    assert!(output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("Warning: ADR directory"));
    assert!(stderr.contains("docs/adr"));
}

#[test]
fn rows_are_sorted_and_output_header_matches_contract() {
    let workspace = unique_temp_dir("adr_cli_output_contract");
    write_file(
        &workspace.join("docs/adr/10-zeta.md"),
        "# Zeta\n\n## Status\n\nAccepted\n",
    );
    write_file(
        &workspace.join("docs/adr/0002-alpha.md"),
        "# Alpha\n\n## Status\n\nAccepted\n",
    );
    write_file(
        &workspace.join("docs/adr/2-beta.md"),
        "# Beta\n\n## Status\n\nAccepted\n",
    );

    let output = run_cli(&workspace, "list");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
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
    let workspace = unique_temp_dir("adr_cli_unknown");
    write_file(
        &workspace.join("docs/adr/1-title-only.md"),
        "# Title Only\n\n## Context\n\nNo status section.\n",
    );
    write_file(
        &workspace.join("docs/adr/2-status-only.md"),
        "## Status\n\nAccepted\n\n## Context\n\nNo title.\n",
    );

    let output = run_cli(&workspace, "list");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("1    Unknown    Title Only    1-title-only.md"));
    assert!(stdout.contains("2    Accepted    Unknown    2-status-only.md"));
}
