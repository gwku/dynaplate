use crate::error::AppError;
use crate::parser::models::VariableType;
use crate::parser::{Variable, VariableValue};
use inquire::{Confirm, Select, Text};

pub fn prompt_for_variable(variable: &Variable) -> Result<Option<VariableValue>, AppError> {
    let prompt_description = match &variable.default {
        Some(default) => {
            let default_text = match variable.var_type {
                VariableType::Boolean => {
                    if default == "true" {
                        "yes"
                    } else {
                        "no"
                    }
                }
                _ => &format!("{:?}", default),
            };
            format!("{} (default: {})", variable.description, default_text)
        }
        None => variable.description.clone(),
    };

    match variable.var_type {
        VariableType::String => {
            let default_value = variable.default.as_deref().unwrap_or("");
            let answer = Text::new(&prompt_description)
                .with_default(default_value)
                .prompt()
                .map_err(AppError::from)?;

            Ok(Some(VariableValue::String(answer)))
        }
        VariableType::Boolean => {
            let default_value = variable
                .default
                .as_deref()
                .map(|d| d == "true")
                .unwrap_or(false);
            let answer = Confirm::new(&prompt_description)
                .with_default(default_value)
                .prompt()
                .map_err(AppError::from)?;

            Ok(Some(VariableValue::Boolean(answer)))
        }
        VariableType::Select => {
            let options = variable.options.clone().unwrap_or_default();
            let answer = Select::new(&prompt_description, options)
                .prompt()
                .map_err(AppError::from)?;

            Ok(Some(VariableValue::Select(vec![answer])))
        }
    }
}
