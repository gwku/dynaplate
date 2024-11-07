use crate::parser::ParserError;
use std::io;
use std::io::Error;
use thiserror::Error;

pub type UtilsResult<T> = Result<T, UtilsError>;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("File I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Command not applicable: {name}")]
    CommandNotApplicable { name: String },

    #[error("Command {name} failed: {source}")]
    CommandFailed { name: String, source: Error },

    #[error("Command failed because of parse error: {0}")]
    CommandFailedDueToParseError(#[from] ParserError),

    #[error("Specified variable '{0}' is not set")]
    VariableNotSet(String),
}
