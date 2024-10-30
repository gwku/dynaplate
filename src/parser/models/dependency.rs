use crate::parser::traits::condition::ConditionTrait;
use crate::parser::traits::CommandTrait;
use crate::parser::Condition;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub command: String,
    pub conditions: Option<Vec<Condition>>,
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dependency:\n  Name: {}\n  Command: {}\n  Conditions: {:?}",
            self.name, self.command, self.conditions
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
}

impl ConditionTrait for Dependency {
    fn get_conditions(&self) -> Option<&[Condition]> {
        self.conditions.as_deref()
    }
}
