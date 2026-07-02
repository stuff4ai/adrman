use regex::Regex;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const ADR_DIR: &str = "docs/adr/";
const TEMPLATE_REL_PATH: &str = "docs/adr/.adr-template.md";
const TEMPLATE_FILE_NAME: &str = ".adr-template.md";
const TITLE_PLACEHOLDER_LINE: &str = "# Title";
const STATUS_HEADING: &str = "## Status";
const INITIAL_STATUS: &str = "Proposed";
const UNKNOWN: &str = "Unknown";
const REQUIRED_SECTIONS: &[&str] = &["Status", "Context", "Decision", "Consequences"];
const SUPPORTED_STATUSES: &[&str] = &[
    "Proposed",
    "Accepted",
    "Rejected",
    "Deprecated",
    "Superseded",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdrEntry {
    pub id: String,
    pub status: String,
    pub title: String,
    pub file: String,
    sort_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListAdrsResult {
    Entries(Vec<AdrEntry>),
    MissingDirectory(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub file: String,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckAdrsResult {
    Valid,
    Invalid(Vec<ValidationIssue>),
    MissingDirectory(PathBuf),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckOutputFormat {
    Human,
    Json,
}

#[derive(Debug)]
pub enum NewAdrError {
    MissingTemplate,
    EmptySlug,
    TargetExists(PathBuf),
    Io(io::Error),
}

impl std::fmt::Display for NewAdrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTemplate => write!(f, "{TEMPLATE_REL_PATH} is missing"),
            Self::EmptySlug => write!(f, "title cannot be converted to a slug"),
            Self::TargetExists(path) => {
                write!(f, "target file already exists: {}", path.display())
            }
            Self::Io(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for NewAdrError {}

pub fn create_new_adr(repo_root: &Path, title: &str) -> Result<PathBuf, NewAdrError> {
    let template = read_template(repo_root)?;
    let slug = slugify_title(title)?;
    let next_id = discover_next_adr_id(repo_root).map_err(NewAdrError::Io)?;
    let file_name = format!("{:04}-{slug}.md", next_id);
    let target_path = repo_root
        .join(ADR_DIR.trim_end_matches('/'))
        .join(&file_name);

    let content = populate_template(&template, title);
    write_new_adr_file(&target_path, &content)?;

    Ok(target_path)
}

fn write_new_adr_file(target_path: &Path, content: &str) -> Result<(), NewAdrError> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target_path)
    {
        Ok(mut file) => file
            .write_all(content.as_bytes())
            .map_err(NewAdrError::Io)
            .map(|_| ()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            Err(NewAdrError::TargetExists(target_path.to_path_buf()))
        }
        Err(error) => Err(NewAdrError::Io(error)),
    }
}

fn read_template(repo_root: &Path) -> Result<String, NewAdrError> {
    let template_path = repo_root.join(TEMPLATE_REL_PATH);
    match fs::read_to_string(&template_path) {
        Ok(content) => Ok(content),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Err(NewAdrError::MissingTemplate),
        Err(error) => Err(NewAdrError::Io(error)),
    }
}

fn discover_next_adr_id(repo_root: &Path) -> io::Result<u64> {
    let adr_path = repo_root.join(ADR_DIR.trim_end_matches('/'));
    let read_dir = fs::read_dir(&adr_path)?;
    let mut max_id = 0;

    for dir_entry in read_dir {
        let dir_entry = dir_entry?;
        if !dir_entry.file_type()?.is_file() {
            continue;
        }

        let file_name = dir_entry.file_name().to_string_lossy().into_owned();
        if !is_adr_filename(&file_name) {
            continue;
        }

        let Some((_, sort_id)) = extract_id(&file_name) else {
            continue;
        };

        max_id = max_id.max(sort_id);
    }

    Ok(max_id + 1)
}

fn slugify_title(title: &str) -> Result<String, NewAdrError> {
    static NON_ASCII_ALNUM_RE: OnceLock<Regex> = OnceLock::new();
    let pattern =
        NON_ASCII_ALNUM_RE.get_or_init(|| Regex::new(r"[^a-z0-9]+").expect("valid slug regex"));
    let lowercased = title.to_lowercase();
    let slug = pattern
        .replace_all(&lowercased, "-")
        .trim_matches('-')
        .to_string();

    if slug.is_empty() {
        return Err(NewAdrError::EmptySlug);
    }

    Ok(slug)
}

fn populate_template(template: &str, title: &str) -> String {
    let mut output = String::new();
    let mut in_status_section = false;
    let mut status_replaced = false;
    let mut title_replaced = false;

    for (index, line) in template.lines().enumerate() {
        if index > 0 {
            output.push('\n');
        }

        if !title_replaced && index == 0 && line == TITLE_PLACEHOLDER_LINE {
            output.push_str(&format!("# {title}"));
            title_replaced = true;
            continue;
        }

        if line.trim() == STATUS_HEADING {
            in_status_section = true;
            output.push_str(line);
            continue;
        }

        if in_status_section
            && !status_replaced
            && !line.trim().is_empty()
            && !line.trim().starts_with('#')
        {
            output.push_str(INITIAL_STATUS);
            status_replaced = true;
            continue;
        }

        if in_status_section && line.trim().starts_with('#') {
            in_status_section = false;
        }

        output.push_str(line);
    }

    if template.ends_with('\n') {
        output.push('\n');
    }

    output
}

pub fn list_adrs(repo_root: &Path) -> io::Result<ListAdrsResult> {
    let adr_path = repo_root.join(ADR_DIR.trim_end_matches('/'));
    let read_dir = match fs::read_dir(&adr_path) {
        Ok(read_dir) => read_dir,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(ListAdrsResult::MissingDirectory(adr_path));
        }
        Err(error) => return Err(error),
    };

    let mut entries = Vec::new();
    for dir_entry in read_dir {
        let dir_entry = dir_entry?;
        if !dir_entry.file_type()?.is_file() {
            continue;
        }

        let file_name = dir_entry.file_name().to_string_lossy().into_owned();
        if !is_adr_filename(&file_name) {
            continue;
        }

        let Some((id, sort_id)) = extract_id(&file_name) else {
            continue;
        };

        let content = fs::read_to_string(dir_entry.path())?;
        entries.push(AdrEntry {
            id,
            status: extract_status(&content).unwrap_or_else(|| UNKNOWN.to_string()),
            title: extract_title(&content).unwrap_or_else(|| UNKNOWN.to_string()),
            file: file_name,
            sort_id,
        });
    }

    entries.sort_by(|left, right| {
        left.sort_id
            .cmp(&right.sort_id)
            .then_with(|| left.file.cmp(&right.file))
    });
    Ok(ListAdrsResult::Entries(entries))
}

pub fn check_adrs(repo_root: &Path) -> io::Result<CheckAdrsResult> {
    let adr_path = repo_root.join(ADR_DIR.trim_end_matches('/'));
    let read_dir = match fs::read_dir(&adr_path) {
        Ok(read_dir) => read_dir,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(CheckAdrsResult::MissingDirectory(adr_path));
        }
        Err(error) => return Err(error),
    };

    let mut issues = Vec::new();
    let mut adr_files = Vec::new();

    for dir_entry in read_dir {
        let dir_entry = dir_entry?;
        if !dir_entry.file_type()?.is_file() {
            continue;
        }

        let file_name = dir_entry.file_name().to_string_lossy().into_owned();
        if file_name.ends_with(".md")
            && !is_adr_filename(&file_name)
            && file_name != TEMPLATE_FILE_NAME
        {
            issues.push(ValidationIssue {
                file: file_name.clone(),
                code: "invalid_filename",
                message: format!("{file_name} does not match ADR naming pattern"),
            });
            continue;
        }

        if !is_adr_filename(&file_name) {
            continue;
        }

        let Some((_, sort_id)) = extract_id(&file_name) else {
            continue;
        };

        let content = fs::read_to_string(dir_entry.path())?;
        adr_files.push((file_name, sort_id, content));
    }

    let mut id_to_files: std::collections::BTreeMap<u64, Vec<String>> =
        std::collections::BTreeMap::new();
    for (file_name, sort_id, _) in &adr_files {
        id_to_files
            .entry(*sort_id)
            .or_default()
            .push(file_name.clone());
    }

    for (sort_id, files) in id_to_files {
        if files.len() > 1 {
            let shared_by = files.join(", ");
            for file in files {
                issues.push(ValidationIssue {
                    file,
                    code: "duplicate_id",
                    message: format!("numeric ADR ID {sort_id} is shared by: {shared_by}"),
                });
            }
        }
    }

    for (file_name, _, content) in &adr_files {
        validate_adr_content(file_name, content, &mut issues);
    }

    issues.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.code.cmp(right.code))
            .then_with(|| left.message.cmp(&right.message))
    });

    if issues.is_empty() {
        Ok(CheckAdrsResult::Valid)
    } else {
        Ok(CheckAdrsResult::Invalid(issues))
    }
}

pub fn check_has_failures(result: &CheckAdrsResult) -> bool {
    !matches!(result, CheckAdrsResult::Valid)
}

pub fn format_check_result(result: &CheckAdrsResult, format: CheckOutputFormat) -> String {
    match (result, format) {
        (CheckAdrsResult::Valid, CheckOutputFormat::Human) => {
            "All ADRs in docs/adr/ are valid.\n".to_string()
        }
        (CheckAdrsResult::Valid, CheckOutputFormat::Json) => {
            "{\"valid\":true,\"issues\":[]}\n".to_string()
        }
        (CheckAdrsResult::MissingDirectory(path), CheckOutputFormat::Human) => {
            format!("ADR directory '{}' does not exist.\n", path.display())
        }
        (CheckAdrsResult::MissingDirectory(path), CheckOutputFormat::Json) => {
            let message = format!("ADR directory '{}' does not exist.", path.display());
            format!(
                "{{\"valid\":false,\"issues\":[{{\"file\":\"\",\"code\":\"missing_directory\",\"message\":\"{}\"}}]}}\n",
                json_escape(&message)
            )
        }
        (CheckAdrsResult::Invalid(issues), CheckOutputFormat::Human) => format_human_issues(issues),
        (CheckAdrsResult::Invalid(issues), CheckOutputFormat::Json) => format_json_issues(issues),
    }
}

fn validate_adr_content(file_name: &str, content: &str, issues: &mut Vec<ValidationIssue>) {
    for section in REQUIRED_SECTIONS {
        match extract_section_content(content, section) {
            SectionContent::Missing => issues.push(ValidationIssue {
                file: file_name.to_string(),
                code: "missing_section",
                message: format!("missing required section '{section}'"),
            }),
            SectionContent::Empty => issues.push(ValidationIssue {
                file: file_name.to_string(),
                code: "empty_section",
                message: format!("required section '{section}' is empty"),
            }),
            SectionContent::Present(value) if *section == "Status" => {
                if !SUPPORTED_STATUSES.contains(&value.as_str()) {
                    issues.push(ValidationIssue {
                        file: file_name.to_string(),
                        code: "invalid_status",
                        message: format!("unsupported status '{value}'"),
                    });
                }
            }
            SectionContent::Present(_) => {}
        }
    }
}

enum SectionContent {
    Missing,
    Empty,
    Present(String),
}

fn extract_section_content(content: &str, section_name: &str) -> SectionContent {
    let heading = format!("## {section_name}");
    let mut in_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if !in_section {
            if trimmed == heading {
                in_section = true;
            }
            continue;
        }

        if trimmed.starts_with('#') {
            return SectionContent::Empty;
        }
        if !trimmed.is_empty() {
            return SectionContent::Present(trimmed.to_string());
        }
    }

    if in_section {
        SectionContent::Empty
    } else {
        SectionContent::Missing
    }
}

fn format_human_issues(issues: &[ValidationIssue]) -> String {
    let issue_label = if issues.len() == 1 { "issue" } else { "issues" };
    let mut output = format!(
        "ADR validation failed ({} {issue_label}):\n\n",
        issues.len()
    );
    for issue in issues {
        output.push_str(&format!(
            "{}: {} ({})\n",
            issue.file, issue.message, issue.code
        ));
    }
    output
}

fn format_json_issues(issues: &[ValidationIssue]) -> String {
    let mut output = String::from("{\"valid\":false,\"issues\":[");
    for (index, issue) in issues.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str(&format!(
            "{{\"file\":\"{}\",\"code\":\"{}\",\"message\":\"{}\"}}",
            json_escape(&issue.file),
            json_escape(issue.code),
            json_escape(&issue.message)
        ));
    }
    output.push_str("]}\n");
    output
}

fn json_escape(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            ch if ch.is_control() => output.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => output.push(ch),
        }
    }
    output
}

pub fn format_adrs_table(entries: &[AdrEntry]) -> String {
    let mut output = String::from("ADRs (docs/adr/)\n\nID    Status    Title    File\n");
    for entry in entries {
        output.push_str(&format!(
            "{}    {}    {}    {}\n",
            entry.id, entry.status, entry.title, entry.file
        ));
    }
    output
}

fn is_adr_filename(file_name: &str) -> bool {
    static ADR_FILENAME_RE: OnceLock<Regex> = OnceLock::new();
    ADR_FILENAME_RE
        .get_or_init(|| Regex::new(r"^[0-9]+[-_ ].*\.md$").expect("valid ADR filename regex"))
        .is_match(file_name)
}

fn extract_id(file_name: &str) -> Option<(String, u64)> {
    let digits_end = file_name
        .bytes()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if digits_end == 0 {
        return None;
    }

    let id_text = file_name[..digits_end].to_string();
    let sort_id = id_text.parse().ok()?;
    Some((id_text, sort_id))
}

fn extract_title(content: &str) -> Option<String> {
    content
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .and_then(|title| (!title.is_empty()).then(|| title.to_string()))
}

fn extract_status(content: &str) -> Option<String> {
    match extract_section_content(content, "Status") {
        SectionContent::Present(value) => Some(value),
        SectionContent::Missing | SectionContent::Empty => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn parses_complete_adr_metadata() {
        let temp_dir = unique_temp_dir("adrman_core_complete");
        write_file(
            &temp_dir.join("docs/adr/0001-first.md"),
            "# First ADR\n\n## Status\n\nAccepted\n",
        );

        let result = list_adrs(&temp_dir).expect("listing should succeed");
        let ListAdrsResult::Entries(entries) = result else {
            panic!("docs/adr exists, entries should be returned");
        };
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, "0001");
        assert_eq!(entries[0].status, "Accepted");
        assert_eq!(entries[0].title, "First ADR");
        assert_eq!(entries[0].file, "0001-first.md");
    }

    #[test]
    fn missing_title_or_status_defaults_to_unknown() {
        let temp_dir = unique_temp_dir("adrman_core_unknown");
        write_file(
            &temp_dir.join("docs/adr/0001-title-only.md"),
            "# Title Only\n\n## Context\n\nNo status section.\n",
        );
        write_file(
            &temp_dir.join("docs/adr/0002-status-only.md"),
            "## Status\n\nAccepted\n\n## Context\n\nNo title heading.\n",
        );

        let result = list_adrs(&temp_dir).expect("listing should succeed");
        let ListAdrsResult::Entries(entries) = result else {
            panic!("docs/adr exists, entries should be returned");
        };
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].title, "Title Only");
        assert_eq!(entries[0].status, "Unknown");
        assert_eq!(entries[1].title, "Unknown");
        assert_eq!(entries[1].status, "Accepted");
    }

    #[test]
    fn excludes_non_matching_files_and_sorts_by_id_then_filename() {
        let temp_dir = unique_temp_dir("adrman_core_sorting");
        write_file(
            &temp_dir.join("docs/adr/10-zeta.md"),
            "# Zeta\n\n## Status\n\nAccepted\n",
        );
        write_file(
            &temp_dir.join("docs/adr/2-beta.md"),
            "# Beta\n\n## Status\n\nAccepted\n",
        );
        write_file(
            &temp_dir.join("docs/adr/0002-alpha.md"),
            "# Alpha\n\n## Status\n\nAccepted\n",
        );
        write_file(
            &temp_dir.join("docs/adr/readme.md"),
            "# Not an ADR\n\n## Status\n\nAccepted\n",
        );

        let result = list_adrs(&temp_dir).expect("listing should succeed");
        let ListAdrsResult::Entries(entries) = result else {
            panic!("docs/adr exists, entries should be returned");
        };

        let files: Vec<&str> = entries.iter().map(|entry| entry.file.as_str()).collect();
        assert_eq!(files, vec!["0002-alpha.md", "2-beta.md", "10-zeta.md"]);
        let ids: Vec<&str> = entries.iter().map(|entry| entry.id.as_str()).collect();
        assert_eq!(ids, vec!["0002", "2", "10"]);
    }

    #[test]
    fn supports_all_agreed_filename_variants_with_text_ids() {
        let temp_dir = unique_temp_dir("adrman_core_variants");
        write_file(
            &temp_dir.join("docs/adr/1_foo.md"),
            "# Foo\n\n## Status\n\nAccepted\n",
        );
        write_file(
            &temp_dir.join("docs/adr/01-bar.md"),
            "# Bar\n\n## Status\n\nAccepted\n",
        );
        write_file(
            &temp_dir.join("docs/adr/001 gap.md"),
            "# Gap\n\n## Status\n\nAccepted\n",
        );

        let result = list_adrs(&temp_dir).expect("listing should succeed");
        let ListAdrsResult::Entries(entries) = result else {
            panic!("docs/adr exists, entries should be returned");
        };

        let files: Vec<&str> = entries.iter().map(|entry| entry.file.as_str()).collect();
        assert_eq!(files, vec!["001 gap.md", "01-bar.md", "1_foo.md"]);

        let ids: Vec<&str> = entries.iter().map(|entry| entry.id.as_str()).collect();
        assert_eq!(ids, vec!["001", "01", "1"]);
    }

    const TEMPLATE: &str = "# Title\n\n## Status\n\nWhat is the status, such as proposed, accepted, rejected, deprecated, superseded, etc.?\n\n## Context\n\nContext text.\n";
    const VALID_ADR: &str = "# Valid ADR\n\n## Status\n\nAccepted\n\n## Context\n\nContext text.\n\n## Decision\n\nDecision text.\n\n## Consequences\n\nConsequence text.\n";

    #[test]
    fn check_adrs_accepts_valid_adr_files() {
        let temp_dir = unique_temp_dir("adrman_core_check_valid");
        write_file(&temp_dir.join("docs/adr/0001-valid.md"), VALID_ADR);

        let result = check_adrs(&temp_dir).expect("check should succeed");
        assert_eq!(result, CheckAdrsResult::Valid);
    }

    #[test]
    fn check_adrs_reports_invalid_filename() {
        let temp_dir = unique_temp_dir("adrman_core_check_invalid_filename");
        write_file(&temp_dir.join("docs/adr/notes.md"), "# Notes\n");

        let result = check_adrs(&temp_dir).expect("check should succeed");
        let CheckAdrsResult::Invalid(issues) = result else {
            panic!("invalid filename should fail validation");
        };
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].file, "notes.md");
        assert_eq!(issues[0].code, "invalid_filename");
    }

    #[test]
    fn check_adrs_ignores_template_for_invalid_filename_checks() {
        let temp_dir = unique_temp_dir("adrman_core_check_template_ignored");
        write_file(&temp_dir.join("docs/adr/.adr-template.md"), TEMPLATE);

        let result = check_adrs(&temp_dir).expect("check should succeed");
        assert_eq!(result, CheckAdrsResult::Valid);
    }

    #[test]
    fn check_adrs_reports_duplicate_numeric_ids() {
        let temp_dir = unique_temp_dir("adrman_core_check_duplicate_ids");
        write_file(&temp_dir.join("docs/adr/0002-alpha.md"), VALID_ADR);
        write_file(&temp_dir.join("docs/adr/2-beta.md"), VALID_ADR);

        let result = check_adrs(&temp_dir).expect("check should succeed");
        let CheckAdrsResult::Invalid(issues) = result else {
            panic!("duplicate ids should fail validation");
        };
        assert_eq!(issues.len(), 2);
        assert!(issues.iter().all(|issue| issue.code == "duplicate_id"));
    }

    #[test]
    fn check_adrs_reports_missing_required_section() {
        let temp_dir = unique_temp_dir("adrman_core_check_missing_section");
        write_file(
            &temp_dir.join("docs/adr/0001-missing-context.md"),
            "# Missing Context\n\n## Status\n\nAccepted\n\n## Decision\n\nDecision.\n\n## Consequences\n\nConsequences.\n",
        );

        let result = check_adrs(&temp_dir).expect("check should succeed");
        let CheckAdrsResult::Invalid(issues) = result else {
            panic!("missing section should fail validation");
        };
        assert!(
            issues
                .iter()
                .any(|issue| issue.code == "missing_section" && issue.message.contains("Context"))
        );
    }

    #[test]
    fn check_adrs_reports_empty_required_section() {
        let temp_dir = unique_temp_dir("adrman_core_check_empty_section");
        write_file(
            &temp_dir.join("docs/adr/0001-empty-decision.md"),
            "# Empty Decision\n\n## Status\n\nAccepted\n\n## Context\n\nContext.\n\n## Decision\n\n## Consequences\n\nConsequences.\n",
        );

        let result = check_adrs(&temp_dir).expect("check should succeed");
        let CheckAdrsResult::Invalid(issues) = result else {
            panic!("empty section should fail validation");
        };
        assert!(
            issues
                .iter()
                .any(|issue| issue.code == "empty_section" && issue.message.contains("Decision"))
        );
    }

    #[test]
    fn check_adrs_reports_invalid_status() {
        let temp_dir = unique_temp_dir("adrman_core_check_invalid_status");
        write_file(
            &temp_dir.join("docs/adr/0001-invalid-status.md"),
            "# Invalid Status\n\n## Status\n\nDraft\n\n## Context\n\nContext.\n\n## Decision\n\nDecision.\n\n## Consequences\n\nConsequences.\n",
        );

        let result = check_adrs(&temp_dir).expect("check should succeed");
        let CheckAdrsResult::Invalid(issues) = result else {
            panic!("invalid status should fail validation");
        };
        assert!(issues.iter().any(|issue| issue.code == "invalid_status"));
    }

    #[test]
    fn check_adrs_reports_missing_directory() {
        let temp_dir = unique_temp_dir("adrman_core_check_missing_directory");

        let result = check_adrs(&temp_dir).expect("check should succeed");
        let CheckAdrsResult::MissingDirectory(path) = result else {
            panic!("missing directory should be reported");
        };
        assert_eq!(path, temp_dir.join("docs/adr"));
    }

    #[test]
    fn format_check_result_json_for_success_and_failure() {
        let success = format_check_result(&CheckAdrsResult::Valid, CheckOutputFormat::Json);
        assert_eq!(success, "{\"valid\":true,\"issues\":[]}\n");

        let failure = format_check_result(
            &CheckAdrsResult::Invalid(vec![ValidationIssue {
                file: "notes.md".to_string(),
                code: "invalid_filename",
                message: "notes.md does not match ADR naming pattern".to_string(),
            }]),
            CheckOutputFormat::Json,
        );
        assert!(failure.contains("\"valid\":false"));
        assert!(failure.contains("\"file\":\"notes.md\""));
        assert!(failure.contains("\"code\":\"invalid_filename\""));
    }

    #[test]
    fn slugify_title_normalizes_punctuation() {
        assert_eq!(
            slugify_title("API Design (v2)!!").expect("slug should be generated"),
            "api-design-v2"
        );
    }

    #[test]
    fn slugify_title_rejects_empty_slug() {
        assert!(matches!(slugify_title("!!!"), Err(NewAdrError::EmptySlug)));
    }

    #[test]
    fn discover_next_adr_id_ignores_template_and_non_adr_files() {
        let temp_dir = unique_temp_dir("adrman_core_next_id_scope");
        write_file(&temp_dir.join("docs/adr/.adr-template.md"), TEMPLATE);
        write_file(
            &temp_dir.join("docs/adr/0004-use-openspec.md"),
            "# Use OpenSpec\n\n## Status\n\nAccepted\n",
        );
        write_file(
            &temp_dir.join("docs/adr/notes.md"),
            "# Notes\n\n## Status\n\nAccepted\n",
        );

        assert_eq!(
            discover_next_adr_id(&temp_dir).expect("next id should be discovered"),
            5
        );
    }

    #[test]
    fn discover_next_adr_id_uses_numeric_maximum() {
        let temp_dir = unique_temp_dir("adrman_core_next_id_max");
        write_file(&temp_dir.join("docs/adr/.adr-template.md"), TEMPLATE);
        write_file(
            &temp_dir.join("docs/adr/2-beta.md"),
            "# Beta\n\n## Status\n\nAccepted\n",
        );
        write_file(
            &temp_dir.join("docs/adr/0004-use-openspec.md"),
            "# Use OpenSpec\n\n## Status\n\nAccepted\n",
        );

        assert_eq!(
            discover_next_adr_id(&temp_dir).expect("next id should be discovered"),
            5
        );
    }

    #[test]
    fn discover_next_adr_id_starts_at_one_without_adr_files() {
        let temp_dir = unique_temp_dir("adrman_core_next_id_empty");
        write_file(&temp_dir.join("docs/adr/.adr-template.md"), TEMPLATE);

        assert_eq!(
            discover_next_adr_id(&temp_dir).expect("next id should be discovered"),
            1
        );
    }

    #[test]
    fn populate_template_replaces_title_and_status() {
        let content = populate_template(TEMPLATE, "Use SQLite for local cache");
        assert!(content.starts_with("# Use SQLite for local cache\n\n## Status\n\nProposed\n"));
        assert!(content.contains("## Context\n\nContext text."));
    }

    #[test]
    fn create_new_adr_writes_expected_file() {
        let temp_dir = unique_temp_dir("adrman_core_create_new");
        write_file(&temp_dir.join("docs/adr/.adr-template.md"), TEMPLATE);
        write_file(
            &temp_dir.join("docs/adr/0004-use-openspec.md"),
            "# Use OpenSpec\n\n## Status\n\nAccepted\n",
        );

        let created =
            create_new_adr(&temp_dir, "Use SQLite for local cache").expect("adr should be created");
        assert_eq!(
            created,
            temp_dir.join("docs/adr/0005-use-sqlite-for-local-cache.md")
        );

        let content = fs::read_to_string(&created).expect("created file should be readable");
        assert!(content.starts_with("# Use SQLite for local cache\n\n## Status\n\nProposed\n"));
    }

    #[test]
    fn create_new_adr_fails_when_template_is_missing() {
        let temp_dir = unique_temp_dir("adrman_core_missing_template");
        fs::create_dir_all(temp_dir.join("docs/adr")).expect("adr directory should exist");

        let error = create_new_adr(&temp_dir, "Use SQLite for local cache")
            .expect_err("missing template should fail");
        assert!(matches!(error, NewAdrError::MissingTemplate));
    }

    #[test]
    fn write_new_adr_file_fails_when_target_already_exists() {
        let temp_dir = unique_temp_dir("adrman_core_target_exists");
        let target_path = temp_dir.join("docs/adr/0001-use-sqlite-for-local-cache.md");
        write_file(&target_path, "existing content");

        let error = write_new_adr_file(&target_path, "new content")
            .expect_err("existing target should fail");
        assert!(matches!(error, NewAdrError::TargetExists(_)));

        let content = fs::read_to_string(&target_path).expect("existing file should remain");
        assert_eq!(content, "existing content");
    }
}
