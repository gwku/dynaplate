use std::collections::HashMap;
use serde::Deserialize;
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

pub fn create_env_vars(env_vars: &[EnvVar]) -> HashMap<String, String> {
    env_vars
        .iter()
        .map(|var| {
            env::set_var(&var.name, &var.value);
            (var.name.clone(), var.value.clone())
        })
        .collect()
}