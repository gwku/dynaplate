use crate::parser::error::ParseResult;
use crate::parser::{Condition, ConditionOperator, ParserError, Variable};

pub fn has_applicable_conditions(
    conditions: Option<&[Condition]>,
    variables: &[Variable],
) -> ParseResult<bool> {
    if let Some(conditions) = conditions {
        for condition in conditions {
            let var = variables
                .iter()
                .find(|var| var.name.to_lowercase() == condition.variable.to_lowercase())
                .ok_or_else(|| ParserError::VariableDoesNotExist(condition.variable.clone()))?;

            let condition_applicable = match condition.operator {
                ConditionOperator::Equals => var.value.as_ref() == Some(&condition.value),
            };

            if condition_applicable {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
