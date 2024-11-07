use crate::parser::error::ParseResult;
use crate::parser::{Condition, ConditionOperator, ParserError, Variable, VariableValue};

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
                ConditionOperator::Equals => match (&var.value, &condition.value) {
                    (Some(VariableValue::String(v)), VariableValue::String(c)) => v == c,
                    (Some(VariableValue::Boolean(v)), VariableValue::Boolean(c)) => v == c,
                    (Some(VariableValue::Select(v)), VariableValue::String(c)) => v == c, // TODO: add support for MultipleSelect
                    _ => false, // Type mismatch or None value
                },
            };

            if condition_applicable {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
