use std::fmt;
use serde::Deserialize;

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