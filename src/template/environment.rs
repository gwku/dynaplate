use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct EnvVar {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EnvVar:\n  Key: {}\n  Value: {}", self.name, self.value)
    }
}
