use crate::parser::traits::condition::ConditionTrait;
use crate::parser::traits::CommandTrait;
use crate::parser::Variable;
use crate::utils::condition::has_applicable_conditions;
use crate::utils::error::UtilsResult;
use crate::utils::UtilsError;
use crate::utils::UtilsError::{CommandFailedDueToParseError, CommandNotApplicable};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Project {
    pub path: PathBuf,
    pub envs: HashMap<String, String>,
    pub variables: Vec<Variable>,
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

    let args: Vec<&str> = command.command().split_whitespace().collect();
    if args.is_empty() {
        return Err(UtilsError::CommandIsEmpty {
            name: command.name().to_string(),
        });
    }

    let mut cmd = std::process::Command::new(args[0]);
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
