use serde::Deserialize;
use std::fmt;
use std::fmt::write;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
}

impl fmt::Display for ConditionOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionOperator::Equals => write!(f, "EQUALS"),
            ConditionOperator::NotEquals => write!(f, "NOT_EQUALS"),
        }
    }
}
