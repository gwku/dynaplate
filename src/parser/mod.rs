mod error;
pub mod models;
pub mod traits;

pub use models::Command;
pub use models::Condition;
pub use models::ConditionOperator;
pub use models::Configuration;
pub use models::Dependency;
pub use models::EnvVar;
pub use models::TemplateFile;
pub use models::TemplateFileType;
pub use models::Variable;
pub use models::VariableValue;

pub use error::ParserError;

use crate::parser::error::ParseResult;

pub fn from_yaml(input: &str) -> ParseResult<Configuration> {
    serde_yml::from_str(input).map_err(ParserError::YamlParseError)
}

pub fn from_json(input: &str) -> ParseResult<Configuration> {
    serde_json::from_str(input).map_err(ParserError::JsonParseError)
}
