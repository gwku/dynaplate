use crate::parser::traits::CommandTrait;
use crate::parser::Variable;
use crate::utils::error::UtilsResult;
use crate::utils::UtilsError;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Project {
    pub path: PathBuf,
    pub envs: HashMap<String, String>,
    pub variables: Vec<Variable>,
}

pub fn execute_commands<T>(commands: &[T], project: &Project)
where
    T: CommandTrait,
{
    for command in commands {
        match execute_command(command, project) {
            Ok(_) => {
                println!("Successfully executed command: {}", command.name());
            }
            Err(e) => match e {
                UtilsError::CommandNotApplicable { .. } => {
                    println!("Skipped command: {} (false condition)", command.name());
                }
                _ => {
                    eprintln!("Failed executing command: {}", command.name());
                }
            },
        };
    }
}

pub fn execute_command<T: CommandTrait>(command: &T, project: &Project) -> UtilsResult<()> {
    match command.is_applicable(&project.variables) {
        Ok(condition) => {
            if !condition {
                return Err(UtilsError::CommandNotApplicable {
                    name: command.name().to_string(),
                });
            }
        }
        Err(_) => {
            todo!()
        }
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
