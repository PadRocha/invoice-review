mod review_invoice;

use crate::cli::{build_help_message, build_version_message, CliOptions};
use crate::shared::AppError;

pub fn execute_command(options: CliOptions) -> Result<String, AppError> {
    match options {
        CliOptions::Help => Ok(build_help_message()),
        CliOptions::Version => Ok(build_version_message()),
        CliOptions::Review(options) => review_invoice::execute(options),
    }
}
