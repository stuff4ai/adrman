use adrman_core::{
    CheckOutputFormat, ListAdrsResult, check_adrs, check_has_failures, create_new_adr,
    format_adrs_table, format_check_result, list_adrs,
};
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let _binary = args.next();
    let command = args.next();

    match command.as_deref() {
        Some("list" | "ls") if args.next().is_none() => run_list(),
        Some("new") => run_new(&mut args),
        Some("check" | "validate") => run_check(&mut args),
        _ => {
            eprintln!(
                "Usage: adr <COMMAND>\n\nCommands:\n  list, ls         List ADRs from docs/adr/\n  new              Create a new ADR from a title\n  check, validate  Validate ADRs in docs/adr/"
            );
            ExitCode::from(2)
        }
    }
}

fn run_new(args: &mut impl Iterator<Item = String>) -> ExitCode {
    let title = args.next();
    if args.next().is_some() {
        eprintln!("Error: unexpected extra arguments");
        return ExitCode::from(1);
    }

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

fn run_check(args: &mut impl Iterator<Item = String>) -> ExitCode {
    let mut format = CheckOutputFormat::Human;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--format" => {
                let Some(value) = args.next() else {
                    eprintln!("Error: --format requires a value");
                    return ExitCode::from(2);
                };
                match value.as_str() {
                    "json" => format = CheckOutputFormat::Json,
                    _ => {
                        eprintln!("Error: unsupported format '{value}'");
                        return ExitCode::from(2);
                    }
                }
            }
            _ => {
                eprintln!("Error: unexpected argument '{arg}'");
                return ExitCode::from(2);
            }
        }
    }

    match check_adrs(Path::new(".")) {
        Ok(result) => {
            print!("{}", format_check_result(&result, format));
            if check_has_failures(&result) {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(error) => {
            eprintln!("Error: failed to check ADRs: {error}");
            ExitCode::from(1)
        }
    }
}
