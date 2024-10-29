use crate::template;
use crate::template::Variable;
use log::{debug, error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::process::Stdio;

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub command: String,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Command:\n  Name: {}\n  Command: {}\n",
            self.name, self.command
        )
    }
}

pub trait CommandTrait {
    fn command(&self) -> &str;
    fn name(&self) -> &str;
    fn is_applicable(&self, variables: &[Variable]) -> Result<bool, String>;
}

impl CommandTrait for Command {
    fn command(&self) -> &str {
        &self.command
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_applicable(&self, _variables: &[Variable]) -> Result<bool, String> {
        Ok(true)
    }
}

impl CommandTrait for template::Dependency {
    fn command(&self) -> &str {
        &self.command
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_applicable(&self, variables: &[Variable]) -> Result<bool, String> {
        self.is_applicable(variables)
    }
}

pub fn execute_commands<T: CommandTrait>(
    commands: &[T],
    project_dir: &PathBuf,
    envs: HashMap<String, String>,
    dry_run: &bool,
    verbose: &bool,
    command_type: &str,
    applicable_vars: Option<&[Variable]>,
) -> Result<(), String> {
    for command in commands {
        if let Some(vars) = applicable_vars {
            if !command.is_applicable(vars).unwrap_or(false) {
                debug!("{}: '{}' is not applicable.", command_type, command.name());
                continue;
            }
        }

        let args: Vec<&str> = command.command().split_whitespace().collect();
        if args.is_empty() {
            return Err(format!(
                "{}: command {} is empty.",
                command_type,
                command.name()
            ));
        }

        let mut cmd = std::process::Command::new(args[0]);
        cmd.current_dir(project_dir).envs(&envs);

        if !*verbose {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        if args.len() > 1 {
            cmd.args(&args[1..]);
        }

        if *dry_run {
            info!(
                "{}: '{}' not executed (dry run).\n{:?}",
                command_type,
                command.name(),
                cmd
            );
            continue;
        }

        match cmd.status() {
            Ok(status) if status.success() => {
                info!(
                    "{} '{}' executed successfully.",
                    command_type,
                    command.name()
                );
            }
            Ok(status) => {
                error!(
                    "{} '{}' failed with status: {:?}",
                    command_type,
                    command.name(),
                    status
                );
                return Err(format!("Execution stopped due to command failure: {:?}", cmd).into());
            }
            Err(e) => {
                error!(
                    "{}: failed to execute '{}': {}",
                    command_type,
                    command.name(),
                    e
                );
                return Err(format!(
                    "Failed to execute command '{}': {}",
                    command.name(),
                    e
                ));
            }
        }
    }
    info!("-------------------------------------------");
    info!(
        "{}: Finished executing ({} commands).",
        command_type,
        commands.len()
    );
    Ok(())
}
