use assert_cmd::Command;
use assert_fs::TempDir;
use assert_fs::fixture::{FileWriteStr, PathChild};
use predicates::Predicate;
use std::path::Path;
use std::process::Output;

pub struct CliTestWorkspace {
    temp_dir: TempDir,
}

impl CliTestWorkspace {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("temporary workspace should be created"),
        }
    }

    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn write_file(&self, relative_path: &str, contents: &str) {
        self.temp_dir
            .child(relative_path)
            .write_str(contents)
            .expect("test file should be written");
    }

    pub fn write_adr(&self, file_name: &str, contents: &str) {
        self.write_file(&format!("docs/adr/{file_name}"), contents);
    }

    #[allow(dead_code)]
    pub fn write_adr_template(&self, contents: &str) {
        self.write_file("docs/adr/.adr-template.md", contents);
    }

    #[allow(dead_code)]
    pub fn write_default_adr_template(&self) {
        self.write_adr_template(
            "# Title\n\n## Status\n\nWhat is the status, such as proposed, accepted, rejected, deprecated, superseded, etc.?\n\n## Context\n\nWhat is the issue that we're seeing that is motivating this decision or change?\n\n## Decision\n\nWhat is the change that we're proposing and/or doing?\n\n## Consequences\n\nWhat becomes easier or more difficult to do because of this change?\n",
        );
    }

    pub fn run(&self, args: &[&str]) -> CliRun {
        let mut command = Command::cargo_bin("adr").expect("compiled adr binary should exist");
        command.current_dir(self.path());
        command.args(args);
        let output = command.output().expect("CLI command should execute");
        CliRun::new(output)
    }
}

pub struct CliRun {
    output: Output,
}

impl CliRun {
    fn new(output: Output) -> Self {
        Self { output }
    }

    pub fn is_success(&self) -> bool {
        self.output.status.success()
    }

    pub fn status_code(&self) -> Option<i32> {
        self.output.status.code()
    }

    pub fn stdout_str(&self) -> String {
        String::from_utf8(self.output.stdout.clone()).expect("stdout should be valid UTF-8")
    }

    pub fn stderr_str(&self) -> String {
        String::from_utf8(self.output.stderr.clone()).expect("stderr should be valid UTF-8")
    }

    pub fn assert_success(&self) {
        assert!(
            self.is_success(),
            "expected successful exit status, got {:?}\nstdout:\n{}\nstderr:\n{}",
            self.status_code(),
            self.stdout_str(),
            self.stderr_str()
        );
    }

    pub fn assert_stdout<P>(&self, predicate: P)
    where
        P: Predicate<str>,
    {
        let stdout = self.stdout_str();
        assert!(
            predicate.eval(&stdout),
            "stdout assertion failed\nstdout:\n{}",
            stdout
        );
    }

    pub fn assert_stderr<P>(&self, predicate: P)
    where
        P: Predicate<str>,
    {
        let stderr = self.stderr_str();
        assert!(
            predicate.eval(&stderr),
            "stderr assertion failed\nstderr:\n{}",
            stderr
        );
    }
}
