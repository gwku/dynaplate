use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    String,
    Boolean,
    Select,
}

impl Display for VariableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self {
            VariableType::String => "String",
            VariableType::Boolean => "Boolean",
            VariableType::Select => "Select",
        };
        write!(f, "Variable type: {}", variant_name)
    }
}
