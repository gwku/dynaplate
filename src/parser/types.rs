use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_yml::Value;
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

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EnvVar:\n  Key: {}\n  Value: {}", self.name, self.value)
    }
}

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

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableValue::String(s) => write!(f, "STRING({})", s),
            VariableValue::Boolean(b) => write!(f, "BOOL({})", b),
            VariableValue::Select(o) => write!(
                f,
                "SELECT ({})",
                o.iter().fold(String::new(), |acc, x| acc + &x + ", ")
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
    #[serde(rename = "type")]
    pub var_type: String,
    pub options: Option<Vec<String>>,
    pub value: Option<VariableValue>,
}

// Implement Display for Variable
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Variable:\n  Name: {}\n  Description: {}\n  Default: {:?}\n  Type: {}\n  Options: {:?}",
            self.name, self.description, self.default, self.var_type, self.options
        )
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum VariableValue {
    String(String),
    Boolean(bool),
    Select(Vec<String>),
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

#[derive(Debug, Deserialize)]
pub struct TemplateFile {
    pub source: String,
    pub destination: String,
    pub file_type: TemplateFileType,
}

impl fmt::Display for TemplateFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TemplateFile:\nSource: {}\nDestination: {}\n",
            self.source, self.destination
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateFileType {
    Folder,
    File,
}

impl fmt::Display for TemplateFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateFileType::Folder => write!(f, "Folder"),
            TemplateFileType::File => write!(f, "File"),
        }
    }
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

pub trait CommandTrait {
    fn command(&self) -> &str;
    fn name(&self) -> &str;
    fn is_applicable(&self, variables: &[Variable]) -> Result<bool, String>;
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
