use crate::cli::{Args, ConfigType};
use crate::error::AppError;
use crate::parser::models::environment::EnvVarSliceExt;
use crate::utils::command::execute_commands;
use crate::utils::file::{copy_template_files, ensure_project_existence};
use crate::utils::Project;
use clap::Parser;
use std::fs;

mod cli;
mod error;
mod parser;
mod utils;

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let config_string = fs::read_to_string(&args.config).map_err(AppError::ConfigFileRead)?;

    let config = match &args.config_type {
        ConfigType::Json => parser::from_json(&config_string)?,
        ConfigType::Yaml => parser::from_yaml(&config_string)?,
    };

    ensure_project_existence(&args.project_dir)?;

    let envs = &config.environment.to_env_map();

    let project = Project {
        path: args.project_dir,
        envs: envs.to_owned(),
        variables: config.variables,
    };

    execute_commands(&config.pre_commands, &project);
    copy_template_files(&config.template_files);
    execute_commands(&config.dependencies, &project);
    execute_commands(&config.post_commands, &project);

    Ok(())
}
