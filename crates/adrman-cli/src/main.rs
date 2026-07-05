mod cli;
mod exit_code;

use adrman_core::{
    CheckOutputFormat, IndexCheckResult, IndexGenerateResult, InitAdrResult, ListAdrsResult,
    check_adr_index, check_adrs, check_has_failures, create_new_adr, format_adrs_table,
    format_check_result, generate_adr_index, init_adr_workspace, list_adrs,
};
use cli::{CheckArgs, Commands, IndexArgs};
use exit_code::{COMMAND_FAILURE, CliExitCode, SUCCESS, USAGE_ERROR};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = match cli::parse() {
        Ok(cli) => cli,
        Err(code) => return code.into(),
    };

    match cli.command.expect("parser guarantees a subcommand") {
        Commands::Init => run_init(),
        Commands::List => run_list(),
        Commands::New { title_parts } => run_new(title_parts),
        Commands::Check(args) => run_check(args),
        Commands::Index(args) => run_index(args),
    }
    .into()
}

fn run_init() -> CliExitCode {
    match init_adr_workspace(Path::new(".")) {
        Ok(InitAdrResult::Created(_)) => {
            println!("Created docs/adr/.adr-template.md.");
            SUCCESS
        }
        Ok(InitAdrResult::AlreadyExists(_)) => {
            println!("docs/adr/.adr-template.md already exists.");
            SUCCESS
        }
        Err(error) => {
            eprintln!("Error: failed to initialize ADR workspace: {error}");
            COMMAND_FAILURE
        }
    }
}

fn run_new(title_parts: Vec<String>) -> CliExitCode {
    if title_parts.len() > 1 {
        eprintln!("Error: unexpected extra arguments");
        return USAGE_ERROR;
    }

    let Some(title) = title_parts.into_iter().next() else {
        eprintln!("Error: title is required");
        return USAGE_ERROR;
    };

    if title.is_empty() {
        eprintln!("Error: title is required");
        return COMMAND_FAILURE;
    }

    match create_new_adr(Path::new("."), &title) {
        Ok(path) => {
            println!("{}", path.display());
            SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {error}");
            COMMAND_FAILURE
        }
    }
}

fn run_list() -> CliExitCode {
    match list_adrs(Path::new(".")) {
        Ok(ListAdrsResult::Entries(entries)) => {
            print!("{}", format_adrs_table(&entries));
            SUCCESS
        }
        Ok(ListAdrsResult::MissingDirectory(path)) => {
            eprintln!(
                "Warning: ADR directory '{}' does not exist.",
                path.display()
            );
            SUCCESS
        }
        Err(error) => {
            eprintln!("Error: failed to list ADRs: {error}");
            COMMAND_FAILURE
        }
    }
}

fn run_check(args: CheckArgs) -> CliExitCode {
    let format = args.format.unwrap_or(CheckOutputFormat::Human);

    match check_adrs(Path::new(".")) {
        Ok(result) => {
            print!("{}", format_check_result(&result, format));
            if check_has_failures(&result) {
                COMMAND_FAILURE
            } else {
                SUCCESS
            }
        }
        Err(error) => {
            eprintln!("Error: failed to check ADRs: {error}");
            COMMAND_FAILURE
        }
    }
}

fn run_index(args: IndexArgs) -> CliExitCode {
    if args.check {
        match check_adr_index(Path::new(".")) {
            Ok(IndexCheckResult::UpToDate) => {
                println!("docs/adr/README.md is up to date.");
                SUCCESS
            }
            Ok(IndexCheckResult::Stale) => {
                eprintln!("Error: docs/adr/README.md is stale. Run `adr index` to update it.");
                COMMAND_FAILURE
            }
            Ok(IndexCheckResult::MissingIndex) => {
                eprintln!("Error: docs/adr/README.md is missing. Run `adr index` to create it.");
                COMMAND_FAILURE
            }
            Ok(IndexCheckResult::MissingDirectory(path)) => {
                eprintln!("Error: ADR directory '{}' does not exist.", path.display());
                COMMAND_FAILURE
            }
            Err(error) => {
                eprintln!("Error: failed to check ADR index: {error}");
                COMMAND_FAILURE
            }
        }
    } else {
        match generate_adr_index(Path::new(".")) {
            Ok(IndexGenerateResult::Written(path)) => {
                println!("{}", path.display());
                SUCCESS
            }
            Ok(IndexGenerateResult::MissingDirectory(path)) => {
                eprintln!("Error: ADR directory '{}' does not exist.", path.display());
                COMMAND_FAILURE
            }
            Err(error) => {
                eprintln!("Error: failed to generate ADR index: {error}");
                COMMAND_FAILURE
            }
        }
    }
}
