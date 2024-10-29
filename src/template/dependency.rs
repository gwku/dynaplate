use crate::template::variable::custom_deserialize_condition_value;
use crate::template::variable::VariableValue;
use crate::template::Variable;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub command: String,
    pub when: Option<Vec<Condition>>,
}

impl Dependency {
    pub fn is_applicable(&self, variables: &[Variable]) -> Result<bool, String> {
        if let Some(conditions) = &self.when {
            for condition in conditions {
                let var = variables
                    .iter()
                    .find(|var| var.name == condition.variable)
                    .ok_or_else(|| {
                        format!(
                            "Invalid variable provided for condition: {}",
                            condition.variable
                        )
                    })?;

                let condition_applicable = match condition.operator {
                    ConditionOperator::Equals => var.value.as_ref() == Some(&condition.value),
                };

                if !condition_applicable {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dependency:\n  Name: {}\n  Command: {}\n  When: {:?}",
            self.name, self.command, self.when
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub variable: String,
    pub operator: ConditionOperator,
    #[serde(deserialize_with = "custom_deserialize_condition_value")]
    pub value: VariableValue,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConditionOperator {
    Equals,
}

impl fmt::Display for ConditionOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionOperator::Equals => write!(f, "EQUALS"),
        }
    }
}
