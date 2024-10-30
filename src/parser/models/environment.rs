use serde::Deserialize;
use std::collections::HashMap;
use std::{env, fmt};

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

pub trait EnvVarSliceExt {
    fn to_env_map(&self) -> HashMap<String, String>;
}

impl EnvVarSliceExt for [EnvVar] {
    fn to_env_map(&self) -> HashMap<String, String> {
        self.iter()
            .map(|var| {
                env::set_var(&var.name, &var.value);
                (var.name.clone(), var.value.clone())
            })
            .collect()
    }
}
