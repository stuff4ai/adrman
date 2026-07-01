use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const ADR_DIR: &str = "docs/adr/";
const TEMPLATE_REL_PATH: &str = "docs/adr/.adr-template.md";
const TITLE_PLACEHOLDER_LINE: &str = "# Title";
const STATUS_HEADING: &str = "## Status";
const INITIAL_STATUS: &str = "Proposed";
const UNKNOWN: &str = "Unknown";

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

    if target_path.exists() {
        return Err(NewAdrError::TargetExists(target_path));
    }

    let content = populate_template(&template, title);
    fs::write(&target_path, content).map_err(NewAdrError::Io)?;

    Ok(target_path)
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
    let mut in_status_section = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if !in_status_section {
            if trimmed == "## Status" {
                in_status_section = true;
            }
            continue;
        }

        if trimmed.starts_with('#') {
            break;
        }
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    None
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
    fn create_new_adr_fails_when_target_exists() {
        let temp_dir = unique_temp_dir("adrman_core_target_exists");
        write_file(&temp_dir.join("docs/adr/.adr-template.md"), TEMPLATE);

        let target_path = temp_dir.join("docs/adr/0001-use-sqlite-for-local-cache.md");
        fs::create_dir_all(&target_path).expect("target path should be created");

        let error = create_new_adr(&temp_dir, "Use SQLite for local cache")
            .expect_err("existing target should fail");
        assert!(matches!(error, NewAdrError::TargetExists(_)));
    }
}
