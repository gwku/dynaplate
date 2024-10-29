use crate::{Dependency, EnvVar, Variable};
use serde::Deserialize;
use std::fmt;
use crate::template::Command;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub description: String,
    pub version: String,
    pub pre_commands: Vec<Command>,
    pub environment: Vec<EnvVar>,
    pub dependencies: Vec<Dependency>,
    pub variables: Vec<Variable>,
    pub post_commands: Vec<Command>,
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TemplateConfig:\nName: {}\nDescription: {}\nVersion: {}\nPre-Commands: {:?}\nEnvironment: {:?}\nDependencies: {:?}\nVariables: {:?}\nPost-Commands: {:?}",
            self.name, self.description, self.version, self.pre_commands, self.environment, self.dependencies, self.variables, self.post_commands
        )
    }
}
