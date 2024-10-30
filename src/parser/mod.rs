pub mod models;
pub  mod traits;

use std::error::Error;
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

pub fn from_yaml(input: &str) -> Result<Configuration, Box<dyn Error>> {
    serde_yml::from_str(input)
        .map_err(|e| format!("Failed to parse config file: {}", e).into())
}
