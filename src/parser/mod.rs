pub mod types;

use std::error::Error;
use types::Configuration;

pub fn from_yaml(input: &str) -> Result<Configuration, Box<dyn Error>> {
    serde_yml::from_str(input)
        .map_err(|e| format!("Failed to parse config file: {}", e).into())
}
