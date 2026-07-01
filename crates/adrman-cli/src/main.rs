use adrman_core::{ListAdrsResult, format_adrs_table, list_adrs};
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let _binary = args.next();
    let command = args.next();
    let extra = args.next();

    match (command.as_deref(), extra) {
        (Some("list" | "ls"), None) => run_list(),
        _ => {
            eprintln!("Usage: adr <COMMAND>\n\nCommands:\n  list, ls    List ADRs from docs/adr/");
            ExitCode::from(2)
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
