use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Command {
    name: String,
    command: String,
    args: Option<Vec<String>>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Command:\n  Name: {}\n  Command: {}\n  Args: {:?}",
            self.name, self.command, self.args
        )
    }
}
