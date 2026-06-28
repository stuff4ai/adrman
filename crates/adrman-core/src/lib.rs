use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const ADR_DIR: &str = "docs/adr/";
const UNKNOWN: &str = "Unknown";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdrEntry {
    pub id: u64,
    pub status: String,
    pub title: String,
    pub file: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListAdrsResult {
    Entries(Vec<AdrEntry>),
    MissingDirectory(PathBuf),
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

        let Some(id) = extract_id(&file_name) else {
            continue;
        };

        let content = fs::read_to_string(dir_entry.path())?;
        entries.push(AdrEntry {
            id,
            status: extract_status(&content).unwrap_or_else(|| UNKNOWN.to_string()),
            title: extract_title(&content).unwrap_or_else(|| UNKNOWN.to_string()),
            file: file_name,
        });
    }

    entries.sort_by(|left, right| {
        left.id
            .cmp(&right.id)
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

fn extract_id(file_name: &str) -> Option<u64> {
    let digits_end = file_name
        .bytes()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if digits_end == 0 {
        return None;
    }

    file_name[..digits_end].parse().ok()
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
        assert_eq!(entries[0].id, 1);
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
            &temp_dir.join("docs/adr/2 alpha.md"),
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
        assert_eq!(files, vec!["2 alpha.md", "2-beta.md", "10-zeta.md"]);
    }
}
