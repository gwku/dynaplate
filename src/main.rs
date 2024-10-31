use crate::{
    cli::{prompt::prompt_for_variable, Args, ConfigType},
    error::AppError,
    parser::{models::environment::EnvVarSliceExt, Variable},
    utils::{
        command::execute_commands,
        file::{copy_template_files, ensure_project_existence},
        Project,
    },
};
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
    
    let envs = &config.environment.to_env_map();
    let variables = gather_variables(&config.variables)?;

    let project = Project {
        path: args.project_dir,
        envs: envs.to_owned(),
        variables,
        clean: args.clean.unwrap(),
    };

    execute_commands(&config.pre_commands, &project);
    ensure_project_existence(&project.path)?;
    copy_template_files(&config.template_files);
    execute_commands(&config.dependencies, &project);
    execute_commands(&config.post_commands, &project);

    Ok(())
}

fn gather_variables(variables: &[Variable]) -> Result<Vec<Variable>, AppError> {
    variables
        .iter()
        .map(|variable| {
            let user_input = prompt_for_variable(variable)?;
            Ok(Variable {
                name: variable.name.clone(),
                description: variable.description.to_string(),
                default: variable.default.clone(),
                var_type: variable.var_type.clone(),
                options: variable.options.clone(),
                value: user_input,
            })
        })
        .collect()
}
