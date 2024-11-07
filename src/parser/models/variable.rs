use crate::parser::models::variable_type::VariableType;
use crate::parser::VariableValue;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Variable {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
    #[serde(rename = "type")]
    pub var_type: VariableType,
    pub options: Option<Vec<String>>,
    pub value: Option<VariableValue>,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Variable:\n  Name: {}\n  Description: {}\n  Default: {:?}\n  Type: {}\n  Options: {:?}",
            self.name, self.description, self.default, self.var_type, self.options
        )
    }
}
