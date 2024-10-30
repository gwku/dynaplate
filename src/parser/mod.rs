pub(crate) mod error;
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
use crate::parser::models::VariableType;

pub fn from_yaml(input: &str) -> ParseResult<Configuration> {
    let configuration = serde_yml::from_str(input).map_err(ParserError::YamlParseError)?;

    match validate_configuration(&configuration) {
        Ok(_) => Ok(configuration),
        Err(e) => Err(e),
    }
}

pub fn from_json(input: &str) -> ParseResult<Configuration> {
    let configuration = serde_json::from_str(input).map_err(ParserError::JsonParseError)?;

    match validate_configuration(&configuration) {
        Ok(_) => Ok(configuration),
        Err(e) => Err(e),
    }
}

fn validate_configuration(configuration: &Configuration) -> ParseResult<()> {
    for variable in &configuration.variables {
        if let Some(default_value) = &variable.default {
            match variable.var_type {
                VariableType::String => {
                    // No validation required for String type.
                }
                VariableType::Boolean => {
                    if default_value != "true" && default_value != "false" {
                        return Err(ParserError::InvalidDefaultValue(format!(
                            "Invalid default value '{}' for Boolean variable '{}'. Expected 'true' or 'false'.",
                            default_value, variable.name
                        )));
                    }
                }
                VariableType::Select => {
                    if let Some(options) = &variable.options {
                        if !options.contains(default_value) {
                            return Err(ParserError::InvalidDefaultValue(format!(
                                "Invalid default value '{}' for Select variable '{}'. Expected one of: {:?}.",
                                default_value, variable.name, options
                            )));
                        }
                    } else {
                        return Err(ParserError::InvalidDefaultValue(format!(
                            "Select variable '{}' has no options defined.",
                            variable.name
                        )));
                    }
                }
            }
        }
    }
    Ok(())
}
