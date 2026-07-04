mod cli;

use adrman_core::{
    CheckOutputFormat, IndexCheckResult, IndexGenerateResult, InitAdrResult, ListAdrsResult,
    check_adr_index, check_adrs, check_has_failures, create_new_adr, format_adrs_table,
    format_check_result, generate_adr_index, init_adr_workspace, list_adrs,
};
use cli::{CheckArgs, Commands, IndexArgs};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = match cli::parse() {
        Ok(cli) => cli,
        Err(code) => return code,
    };

    match cli.command.expect("parser guarantees a subcommand") {
        Commands::Init => run_init(),
        Commands::List => run_list(),
        Commands::New { title_parts } => run_new(title_parts),
        Commands::Check(args) => run_check(args),
        Commands::Index(args) => run_index(args),
    }
}

fn run_init() -> ExitCode {
    match init_adr_workspace(Path::new(".")) {
        Ok(InitAdrResult::Created(_)) => {
            println!("Created docs/adr/.adr-template.md.");
            ExitCode::SUCCESS
        }
        Ok(InitAdrResult::AlreadyExists(_)) => {
            println!("docs/adr/.adr-template.md already exists.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: failed to initialize ADR workspace: {error}");
            ExitCode::from(1)
        }
    }
}

fn run_new(title_parts: Vec<String>) -> ExitCode {
    if title_parts.len() > 1 {
        eprintln!("Error: unexpected extra arguments");
        return ExitCode::from(1);
    }

    let Some(title) = title_parts.into_iter().next() else {
        eprintln!("Error: title is required");
        return ExitCode::from(1);
    };

    if title.is_empty() {
        eprintln!("Error: title is required");
        return ExitCode::from(1);
    }

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

fn run_check(args: CheckArgs) -> ExitCode {
    let format = match args.format {
        None => CheckOutputFormat::Human,
        Some(value) if value == "json" => CheckOutputFormat::Json,
        Some(value) => {
            eprintln!("Error: unsupported format '{value}'");
            return ExitCode::from(2);
        }
    };

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

fn run_index(args: IndexArgs) -> ExitCode {
    if args.check {
        match check_adr_index(Path::new(".")) {
            Ok(IndexCheckResult::UpToDate) => {
                println!("docs/adr/README.md is up to date.");
                ExitCode::SUCCESS
            }
            Ok(IndexCheckResult::Stale) => {
                eprintln!("Error: docs/adr/README.md is stale. Run `adr index` to update it.");
                ExitCode::from(1)
            }
            Ok(IndexCheckResult::MissingIndex) => {
                eprintln!("Error: docs/adr/README.md is missing. Run `adr index` to create it.");
                ExitCode::from(1)
            }
            Ok(IndexCheckResult::MissingDirectory(path)) => {
                eprintln!("Error: ADR directory '{}' does not exist.", path.display());
                ExitCode::from(1)
            }
            Err(error) => {
                eprintln!("Error: failed to check ADR index: {error}");
                ExitCode::from(1)
            }
        }
    } else {
        match generate_adr_index(Path::new(".")) {
            Ok(IndexGenerateResult::Written(path)) => {
                println!("{}", path.display());
                ExitCode::SUCCESS
            }
            Ok(IndexGenerateResult::MissingDirectory(path)) => {
                eprintln!("Error: ADR directory '{}' does not exist.", path.display());
                ExitCode::from(1)
            }
            Err(error) => {
                eprintln!("Error: failed to generate ADR index: {error}");
                ExitCode::from(1)
            }
        }
    }
}
