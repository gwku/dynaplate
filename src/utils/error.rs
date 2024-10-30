use crate::parser::ParserError;
use std::io;
use thiserror::Error;

pub type UtilsResult<T> = Result<T, UtilsError>;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("File I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Command not applicable: {name}")]
    CommandNotApplicable { name: String },

    #[error("Command is empty {name}")]
    CommandIsEmpty { name: String },

    #[error("Command {name} failed")]
    CommandFailed { name: String },

    #[error("Command failed because of parse error: {0}")]
    CommandFailedDueToParseError(#[from] ParserError),
}
