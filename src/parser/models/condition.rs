use crate::parser::{ConditionOperator, VariableValue};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_yml::Value;
use std::fmt;

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
                        .ok_or_else(|| Error::custom("Expected a string in array"))
                })
                .collect();

            // TODO: add support for multiselect
            strings?
                .first()
                .map(|s| VariableValue::String(s.clone()))
                .ok_or_else(|| Error::custom("Expected at least one string in the array"))
        }
        _ => Err(Error::custom("Expected a string, boolean, or array")),
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
