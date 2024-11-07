use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum VariableValue {
    String(String),
    Boolean(bool),
    Select(Vec<String>),
}

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableValue::String(s) => write!(f, "STRING({})", s),
            VariableValue::Boolean(b) => write!(f, "BOOL({})", b),
            VariableValue::Select(o) => write!(
                f,
                "SELECT ({})",
                o.iter().fold(String::new(), |acc, x| acc + x + ", ")
            ),
        }
    }
}
