use clap::{Parser, Subcommand};
use std::process::ExitCode;

pub const USAGE: &str = "Usage: adr <COMMAND>\n\nCommands:\n  init             Bootstrap docs/adr/ and the ADR template\n  list, ls         List ADRs from docs/adr/\n  new              Create a new ADR from a title\n  check, validate  Validate ADRs in docs/adr/\n  index            Generate or verify docs/adr/README.md";

#[derive(Parser)]
#[command(
    name = "adr",
    about = None,
    long_about = None,
    disable_help_flag = true,
    disable_version_flag = true,
    disable_help_subcommand = true,
    arg_required_else_help = false,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    #[command(visible_alias = "ls")]
    List,
    New {
        #[arg(required = true, num_args = 1.., allow_hyphen_values = true)]
        title_parts: Vec<String>,
    },
    #[command(visible_alias = "validate")]
    Check(CheckArgs),
    Index(IndexArgs),
}

#[derive(clap::Args, Default)]
pub struct CheckArgs {
    #[arg(long)]
    pub format: Option<String>,
}

#[derive(clap::Args, Default)]
pub struct IndexArgs {
    #[arg(long)]
    pub check: bool,
}

pub fn parse() -> Result<Cli, ExitCode> {
    match Cli::try_parse() {
        Ok(cli) => {
            if cli.command.is_none() {
                eprintln!("{USAGE}");
                return Err(ExitCode::from(2));
            }
            Ok(cli)
        }
        Err(err) => Err(handle_parse_error(err)),
    }
}

fn handle_parse_error(err: clap::Error) -> ExitCode {
    use clap::error::ErrorKind;

    match err.kind() {
        ErrorKind::InvalidSubcommand | ErrorKind::MissingSubcommand | ErrorKind::DisplayHelp => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
        ErrorKind::TooManyValues => {
            eprintln!("Error: unexpected extra arguments");
            ExitCode::from(1)
        }
        ErrorKind::UnknownArgument => {
            let message = err.to_string();
            if let Some(argument) = extract_quoted_argument(&message) {
                eprintln!("Error: unexpected argument '{argument}'");
            } else {
                eprintln!("{message}");
            }
            ExitCode::from(2)
        }
        ErrorKind::MissingRequiredArgument => {
            let message = err.to_string();
            if message.contains("<TITLE") {
                eprintln!("Error: title is required");
                ExitCode::from(1)
            } else if message.contains("--format") {
                eprintln!("Error: --format requires a value");
                ExitCode::from(2)
            } else {
                eprintln!("{message}");
                ExitCode::from(2)
            }
        }
        _ => {
            let message = err.to_string();
            if message.contains("--format") && message.contains("a value is required") {
                eprintln!("Error: --format requires a value");
                ExitCode::from(2)
            } else {
                eprintln!("{message}");
                ExitCode::from(2)
            }
        }
    }
}

fn extract_quoted_argument(message: &str) -> Option<&str> {
    let start = message.find('\'')? + 1;
    let rest = message.get(start..)?;
    let end = rest.find('\'')?;
    rest.get(..end)
}
