//! CLI process exit codes.
//!
//! - `0` (`Success`): command invoked correctly and completed successfully
//! - `1` (`CommandFailure`): valid invocation, but the operation failed or check did not pass
//! - `2` (`UsageError`): command was not invoked according to supported CLI syntax

use std::process::ExitCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliExitCode {
    Success = 0,
    CommandFailure = 1,
    UsageError = 2,
}

impl From<CliExitCode> for ExitCode {
    fn from(code: CliExitCode) -> ExitCode {
        ExitCode::from(code as u8)
    }
}

pub const SUCCESS: CliExitCode = CliExitCode::Success;
pub const COMMAND_FAILURE: CliExitCode = CliExitCode::CommandFailure;
pub const USAGE_ERROR: CliExitCode = CliExitCode::UsageError;
