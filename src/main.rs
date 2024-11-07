use crate::error::AppError::InvalidWorkspaceDir;
use crate::parser::models::VariableType;
use crate::parser::VariableValue;
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
use std::path::PathBuf;
use std::str::FromStr;

mod cli;
mod error;
mod parser;
mod utils;

fn main() -> Result<(), AppError> {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();

    let config_string = fs::read_to_string(&args.config).map_err(AppError::ConfigFileRead)?;

    let config = match &args.config_type {
        ConfigType::Json => parser::from_json(&config_string)?,
        ConfigType::Yaml => parser::from_yaml(&config_string)?,
    };

    println!("\x1b[1m{} ({})\x1b[0m", &config.name, &config.version);
    println!("\x1b[1m{}\x1b[0m", &config.description);

    let envs = &config.environment.to_env_map();

    let default_variables = add_default_variables(args.working_dir)?;
    let variables = gather_variables(&config.variables, default_variables)?;

    // unwrap allowed, since variable working_dir is added in code
    let working_dir_var = variables
        .iter()
        .find(|v| v.name == "working_dir")
        .unwrap()
        .value
        .clone();

    let working_dir = match working_dir_var {
        None => Err(InvalidWorkspaceDir),
        Some(value) => match value {
            VariableValue::String(string) => match PathBuf::from_str(&*string) {
                Ok(path) => Ok(path),
                Err(_) => Err(InvalidWorkspaceDir),
            },
            VariableValue::Boolean(_) | VariableValue::Select(_) => Err(InvalidWorkspaceDir),
        },
    }?;

    let project = Project {
        working_dir,
        envs: envs.to_owned(),
        variables: variables.clone(),
        clean: args.clean.unwrap(),
    };

    execute_commands(&config.pre_commands, &project);
    ensure_project_existence(&project.working_dir)?;
    copy_template_files(&config.template_files, &variables)?;
    execute_commands(&config.dependencies, &project);
    execute_commands(&config.post_commands, &project);
    Ok(())
}

fn add_default_variables(working_dir: Option<PathBuf>) -> Result<Vec<Variable>, AppError> {
    let working_dir_var = Variable {
        name: "working_dir".to_string(),
        description: "The working directory in which commands will run by default".to_string(),
        default: Some("./".to_string()),
        var_type: VariableType::String,
        options: None,
        value: match working_dir {
            None => None,
            Some(value) => Some(VariableValue::String(value.display().to_string())),
        },
    };

    Ok(vec![working_dir_var])
}

fn gather_variables(
    variables: &[Variable],
    default_variables: Vec<Variable>,
) -> Result<Vec<Variable>, AppError> {
    default_variables
        .iter()
        .chain(variables.iter())
        .map(|variable| {
            let user_input = match &variable.value {
                Some(value) => Some(value.clone()),
                None => prompt_for_variable(variable)?,
            };

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
