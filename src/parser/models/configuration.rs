use crate::parser::{Command, Dependency, EnvVar, TemplateFile, Variable};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub description: String,
    pub version: String,
    #[serde(default)]
    pub pre_commands: Vec<Command>,
    #[serde(default)]
    pub environment: Vec<EnvVar>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub variables: Vec<Variable>,
    #[serde(default)]
    pub template_files: Vec<TemplateFile>,
    #[serde(default)]
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
