use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_yml::Value;
use std::fmt;

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableValue::String(s) => write!(f, "STRING({})", s),
            VariableValue::Boolean(b) => write!(f, "BOOL({})", b),
            VariableValue::Select(o) => write!(
                f,
                "SELECT ({})",
                o.iter().fold(String::new(), |acc, x| acc + &x + ", ")
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
    #[serde(rename = "type")]
    pub var_type: String,
    pub options: Option<Vec<String>>,
    pub value: Option<VariableValue>,
}

// Implement Display for Variable
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Variable:\n  Name: {}\n  Description: {}\n  Default: {:?}\n  Type: {}\n  Options: {:?}",
            self.name, self.description, self.default, self.var_type, self.options
        )
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum VariableValue {
    String(String),
    Boolean(bool),
    Select(Vec<String>),
}

pub fn custom_deserialize_condition_value<'de, D>(
    deserializer: D,
) -> Result<VariableValue, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) => Ok(VariableValue::String(s)),
        Value::Bool(b) => Ok(VariableValue::Boolean(b)),
        Value::Sequence(arr) => {
            let strings: Result<Vec<String>, D::Error> = arr
                .into_iter()
                .map(|v| {
                    v.as_str()
                        .map(String::from)
                        .ok_or_else(|| D::Error::custom("Expected a string in array"))
                })
                .collect();
            strings.map(VariableValue::Select)
        }
        _ => Err(D::Error::custom("Expected a string, boolean, or array")),
    }
}
