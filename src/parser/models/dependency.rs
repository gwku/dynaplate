use crate::parser::traits::CommandTrait;
use crate::parser::{Condition, ConditionOperator, Variable};
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

impl CommandTrait for Dependency {
    fn command(&self) -> &str {
        &self.command
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_applicable(&self, variables: &[Variable]) -> Result<bool, String> {
        self.is_applicable(variables)
    }
}
