use std::fmt;
use serde::Deserialize;
use crate::parser::traits::CommandTrait;
use crate::parser::Variable;

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub command: String,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Command:\n  Name: {}\n  Command: {}\n",
            self.name, self.command
        )
    }
}

impl CommandTrait for Command {
    fn command(&self) -> &str {
        &self.command
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_applicable(&self, _variables: &[Variable]) -> Result<bool, String> {
        Ok(true)
    }
}