use std::fmt;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use serde_yml::Value;
use crate::parser::{ConditionOperator, VariableValue};

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub variable: String,
    pub operator: ConditionOperator,
    #[serde(deserialize_with = "custom_deserialize_condition_value")]
    pub value: VariableValue,
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

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Condition:\n  Variable: {}\n  Operator: {:?}\n  Value: {}",
            self.variable, self.operator, self.value
        )
    }
}

