pub mod application;
pub mod cli;
pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod shared;

use crate::cli::{build_help_message, parse_cli_args};
use crate::commands::execute_command;
use crate::infrastructure::register_signal_handlers;
use crate::shared::AppError;

pub fn main_entry<I>(args: I) -> Result<String, AppError>
where
    I: IntoIterator<Item = String>,
{
    register_signal_handlers();
    let options = parse_cli_args(args.into_iter().collect())?;
    execute_command(options)
}

pub fn render_error(error: &AppError) -> String {
    match error {
        AppError::Cli(message) => format!("Error: {}\n\n{}", message, build_help_message(),),
        _ => format!("Error: {}", error),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::shared::{CliError, FileSystemError};

    #[test]
    fn render_error_adds_help_for_cli_errors() {
        let rendered = render_error(&AppError::from(CliError::MissingInvoice));

        assert!(rendered.contains("Error: Debes indicar la factura"));
        assert!(rendered.contains("Uso:"));
    }

    #[test]
    fn exit_code_distinguishes_cli_errors() {
        let cli_error = AppError::from(CliError::MissingInvoice);
        let file_system_error = AppError::from(FileSystemError::NotAFile {
            path: PathBuf::from("/tmp"),
        });

        assert_eq!(cli_error.exit_code(), 2);
        assert_eq!(file_system_error.exit_code(), 1);
    }
}
