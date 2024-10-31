use crate::parser::traits::condition::ConditionTrait;
use crate::parser::traits::CommandTrait;
use crate::parser::Variable;
use crate::utils::condition::has_applicable_conditions;
use crate::utils::error::UtilsResult;
use crate::utils::variable::replace_variables;
use crate::utils::UtilsError;
use crate::utils::UtilsError::{
    CommandFailedDueToParseError, CommandIsEmpty, CommandNotApplicable,
};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Project {
    pub path: PathBuf,
    pub envs: HashMap<String, String>,
    pub variables: Vec<Variable>,
    pub clean: bool,
}

pub fn execute_commands<T: CommandTrait + ConditionTrait>(commands: &[T], project: &Project) {
    for command in commands {
        match execute_command(command, project) {
            Ok(_) => {
                println!("Successfully executed command: {}", command.name());
            }
            Err(e) => match e {
                CommandNotApplicable { .. } => {
                    println!("Skipped command: {} (false condition)", command.name());
                }
                e => {
                    eprintln!("Failed executing command '{}': {}", command.name(), e);
                }
            },
        };
    }
}

pub fn execute_command<T: CommandTrait + ConditionTrait>(
    command: &T,
    project: &Project,
) -> UtilsResult<()> {
    if command.get_conditions().is_some() {
        match has_applicable_conditions(command.get_conditions(), &project.variables) {
            Ok(condition) => {
                if !condition {
                    return Err(CommandNotApplicable {
                        name: command.name().to_string(),
                    });
                }
            }
            Err(e) => return Err(CommandFailedDueToParseError(e)),
        }
    }

    let cmd_with_variables_replaced =
        replace_variables(command.command(), &project.variables, &project.clean)?;

    let args = match extract_arguments(command.name().to_string(), cmd_with_variables_replaced) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let mut cmd = std::process::Command::new(&args[0]);
    cmd.current_dir(&project.path).envs(&project.envs);

    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    match cmd.status() {
        Ok(status) => match status.success() {
            true => Ok(()),
            false => Err(UtilsError::CommandFailed {
                name: command.name().to_string(),
            }),
        },
        Err(_) => Err(UtilsError::CommandFailed {
            name: command.name().to_string(),
        }),
    }
}

fn extract_arguments(
    command_name: String,
    cmd_with_variables_replaced: String,
) -> Result<Vec<String>, UtilsResult<()>> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;

    let chars: Vec<char> = cmd_with_variables_replaced.chars().collect();

    for c in chars.iter() {
        match *c {
            '"' | '\'' => {
                // Toggle the quoted state
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                // End of an argument, if not inside quotes
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => {
                // Collect characters into the current argument
                current_arg.push(*c);
            }
        }
    }

    // Push the last argument if any
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    if args.is_empty() {
        return Err(Err(CommandIsEmpty { name: command_name }));
    }
    Ok(args)
}
