use adrman_core::CheckOutputFormat;
use clap::{Parser, Subcommand};

use crate::exit_code::USAGE_ERROR;

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
    #[arg(long, value_parser = parse_check_format)]
    pub format: Option<CheckOutputFormat>,
}

#[derive(clap::Args, Default)]
pub struct IndexArgs {
    #[arg(long)]
    pub check: bool,
}

fn parse_check_format(value: &str) -> Result<CheckOutputFormat, String> {
    match value {
        "json" => Ok(CheckOutputFormat::Json),
        other => Err(format!("unsupported format '{other}'")),
    }
}

pub fn parse() -> Result<Cli, crate::exit_code::CliExitCode> {
    match Cli::try_parse() {
        Ok(cli) => {
            if cli.command.is_none() {
                eprintln!("{USAGE}");
                return Err(USAGE_ERROR);
            }
            Ok(cli)
        }
        Err(err) => Err(handle_parse_error(err)),
    }
}

fn handle_parse_error(err: clap::Error) -> crate::exit_code::CliExitCode {
    use clap::error::ErrorKind;

    match err.kind() {
        ErrorKind::InvalidSubcommand | ErrorKind::MissingSubcommand | ErrorKind::DisplayHelp => {
            eprintln!("{USAGE}");
            USAGE_ERROR
        }
        ErrorKind::TooManyValues => {
            eprintln!("Error: unexpected extra arguments");
            USAGE_ERROR
        }
        ErrorKind::UnknownArgument => {
            let message = err.to_string();
            if let Some(argument) = extract_quoted_argument(&message) {
                eprintln!("Error: unexpected argument '{argument}'");
            } else {
                eprintln!("{message}");
            }
            USAGE_ERROR
        }
        ErrorKind::MissingRequiredArgument => {
            let message = err.to_string();
            if message.contains("<TITLE") {
                eprintln!("Error: title is required");
            } else if message.contains("--format") {
                eprintln!("Error: --format requires a value");
            } else {
                eprintln!("{message}");
            }
            USAGE_ERROR
        }
        ErrorKind::InvalidValue => {
            let message = err.to_string();
            if let Some(value) = extract_invalid_value(&message) {
                if message.contains("--format") {
                    eprintln!("Error: unsupported format '{value}'");
                } else {
                    eprintln!("Error: {value}");
                }
            } else {
                eprintln!("{message}");
            }
            USAGE_ERROR
        }
        _ => {
            let message = err.to_string();
            if message.contains("--format") && message.contains("a value is required") {
                eprintln!("Error: --format requires a value");
            } else {
                eprintln!("{message}");
            }
            USAGE_ERROR
        }
    }
}

fn extract_quoted_argument(message: &str) -> Option<&str> {
    let start = message.find('\'')? + 1;
    let rest = message.get(start..)?;
    let end = rest.find('\'')?;
    rest.get(..end)
}

fn extract_invalid_value(message: &str) -> Option<&str> {
    let marker = "invalid value '";
    let start = message.find(marker)? + marker.len();
    let rest = message.get(start..)?;
    let end = rest.find('\'')?;
    rest.get(..end)
}

#[cfg(test)]
mod tests {
    use crate::exit_code::{COMMAND_FAILURE, CliExitCode, SUCCESS, USAGE_ERROR};
    use std::process::ExitCode;

    #[test]
    fn exit_code_constants_match_policy() {
        assert_eq!(SUCCESS, CliExitCode::Success);
        assert_eq!(COMMAND_FAILURE, CliExitCode::CommandFailure);
        assert_eq!(USAGE_ERROR, CliExitCode::UsageError);
        assert_eq!(ExitCode::from(SUCCESS), ExitCode::SUCCESS);
        assert_eq!(ExitCode::from(COMMAND_FAILURE), ExitCode::from(1));
        assert_eq!(ExitCode::from(USAGE_ERROR), ExitCode::from(2));
    }
}
