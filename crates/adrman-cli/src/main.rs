use adrman_core::{ListAdrsResult, create_new_adr, format_adrs_table, list_adrs};
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let _binary = args.next();
    let command = args.next();

    match command.as_deref() {
        Some("list" | "ls") if args.next().is_none() => run_list(),
        Some("new") => run_new(args.next()),
        _ => {
            eprintln!(
                "Usage: adr <COMMAND>\n\nCommands:\n  list, ls    List ADRs from docs/adr/\n  new         Create a new ADR from a title"
            );
            ExitCode::from(2)
        }
    }
}

fn run_new(title: Option<String>) -> ExitCode {
    let Some(title) = title.filter(|title| !title.is_empty()) else {
        eprintln!("Error: title is required");
        return ExitCode::from(1);
    };

    match create_new_adr(Path::new("."), &title) {
        Ok(path) => {
            println!("{}", path.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run_list() -> ExitCode {
    match list_adrs(Path::new(".")) {
        Ok(ListAdrsResult::Entries(entries)) => {
            print!("{}", format_adrs_table(&entries));
            ExitCode::SUCCESS
        }
        Ok(ListAdrsResult::MissingDirectory(path)) => {
            eprintln!(
                "Warning: ADR directory '{}' does not exist.",
                path.display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: failed to list ADRs: {error}");
            ExitCode::from(1)
        }
    }
}
