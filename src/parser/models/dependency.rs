use crate::parser::error::ParseResult;
use crate::parser::traits::CommandTrait;
use crate::parser::{Condition, ConditionOperator, ParserError, Variable};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub command: String,
    pub condition: Option<Vec<Condition>>,
}

impl Dependency {
    pub fn is_applicable(&self, variables: &[Variable]) -> ParseResult<bool> {
        if let Some(conditions) = &self.condition {
            for condition in conditions {
                let var = variables
                    .iter()
                    .find(|var| var.name == condition.variable)
                    .ok_or_else(|| ParserError::VariableDoesNotExist(condition.variable.clone()))?;

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
            self.name, self.command, self.condition
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

    fn is_applicable(&self, variables: &[Variable]) -> ParseResult<bool> {
        self.is_applicable(variables)
    }
}
