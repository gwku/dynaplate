use crate::parser::{Variable, VariableValue};
use crate::utils::error::UtilsResult;
use crate::utils::UtilsError;
use shell_escape::escape;
use std::collections::HashMap;

fn prepare_command_string(value: &str, clean: &bool) -> String {
    match clean {
        true => escape(value.into()).to_string(),
        false => value.to_string(),
    }
}

pub fn replace_variables(
    input: &str,
    variables: &[Variable],
    clean: &bool,
) -> UtilsResult<String> {
    let variables_map: HashMap<_, _> = variables
        .iter()
        .filter_map(|variable| variable.value.as_ref().map(|v| (variable.name.as_str(), v)))
        .collect();

    let mut result = String::with_capacity(input.len() + 50);
    let mut last_idx = 0;
    let bytes = input.as_bytes();

    let mut idx = 0;
    while idx < bytes.len() - 1 {
        if bytes[idx..].starts_with(b"{{") {
            if let Some(end) = input[idx + 2..].find("}}") {
                let var_name = &input[idx + 2..idx + 2 + end];
                result.push_str(&input[last_idx..idx]);

                match variables_map.get(var_name) {
                    Some(VariableValue::String(s)) => {
                        result.push_str(&prepare_command_string(s, clean))
                    }
                    Some(VariableValue::Boolean(b)) => {
                        result.push_str(&prepare_command_string(&b.to_string(), clean))
                    }
                    Some(VariableValue::Select(options)) => {
                        if let Some(selected) = options.first() {
                            result.push_str(&prepare_command_string(selected, clean));
                        } else {
                            return Err(UtilsError::VariableNotSet(format!(
                                "No value for {{{}}}",
                                var_name
                            )));
                        }
                    }
                    None => {
                        return Err(UtilsError::VariableNotSet(format!(
                            "Variable {} not found",
                            var_name
                        )))
                    }
                }

                last_idx = idx + 2 + end + 2;
                idx = last_idx;
                continue;
            }
        }
        idx += 1;
    }

    result.push_str(&input[last_idx..]);
    Ok(result)
}
