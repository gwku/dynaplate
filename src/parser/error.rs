use thiserror::Error;

pub type ParseResult<T> = Result<T, ParserError>;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("YAML parsing error: {0}")]
    YamlParseError(#[from] serde_yml::Error),

    #[error("JSON parsing error: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Variable does not exist: {0}")]
    VariableDoesNotExist(String),

    #[error("Specified default value is invalid: {0}")]
    InvalidDefaultValue(String),
}
