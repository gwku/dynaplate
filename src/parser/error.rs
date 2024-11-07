use thiserror::Error;

pub type ParseResult<T> = Result<T, ParserError>;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("YAML parsing error: {0}")]
    YamlParseError(#[from] serde_yml::Error),

    #[error("JSON parsing error: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Variable '{0}' does not exist")]
    VariableDoesNotExist(String),

    #[error("Variable '{name}' has incorrect value: {val}")]
    VariableHasIncorrectValue { name: String, val: String },

    #[error("Specified default value is invalid: {0}")]
    InvalidDefaultValue(String),

    #[error("Command '{name}' is empty")]
    CommandIsEmpty { name: String },
}
