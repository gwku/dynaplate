use crate::parser::traits::condition::ConditionTrait;
use crate::parser::traits::CommandTrait;
use crate::parser::Variable;
use crate::utils::condition::has_applicable_conditions;
use crate::utils::error::UtilsResult;
use crate::utils::variable::replace_variables;
use crate::utils::UtilsError;
use crate::utils::UtilsError::{CommandFailedDueToParseError, CommandNotApplicable};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Project {
    pub working_dir: PathBuf,
    pub envs: HashMap<String, String>,
    pub variables: Vec<Variable>,
    pub clean: bool,
}

pub fn execute_commands<T: CommandTrait + ConditionTrait>(commands: &[T], project: &Project) {
    for command in commands {
        println!("Processing command '{}'", command.name());
        match execute_command(command, project) {
            Ok(_) => {
                println!("Successfully processed command: {}", command.name());
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

    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c").arg(cmd_with_variables_replaced);

    cmd.current_dir(&project.working_dir).envs(&project.envs);

    match cmd.output() {
        Ok(output) => match output.status.success() {
            true => Ok(()),
            false => {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(UtilsError::CommandFailed {
                    name: command.name().to_string(),
                    source: Error::new(ErrorKind::Other, error_message.to_string()),
                })
            }
        },
        Err(e) => Err(UtilsError::CommandFailed {
            name: command.name().to_string(),
            source: e,
        }),
    }
}
