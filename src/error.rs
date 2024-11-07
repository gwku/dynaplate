use crate::cli::CliError;
use crate::parser::ParserError;
use crate::utils::UtilsError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("CLI error: {0}")]
    Cli(#[from] CliError),

    #[error("Parser error: {0}")]
    Parse(#[from] ParserError),

    #[error("Utilities error: {0}")]
    Utils(#[from] UtilsError),

    #[error("Provided config can not be read: {0}")]
    ConfigFileRead(#[from] io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(#[from] inquire::InquireError),
    
    #[error("Invalid working directory")]
    InvalidWorkspaceDir,
}
