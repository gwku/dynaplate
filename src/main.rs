mod cli;
mod template;

use crate::cli::Args;
use crate::template::command::execute_commands;
use crate::template::configuration::load_config;
use crate::template::file::create_template_files;
use crate::template::project::create_project_dir;
use crate::template::{Dependency, EnvVar, Variable};
use clap::Parser;
use environment::create_env_vars;
use std::{error::Error, fs};
use template::environment;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();
    let config_content = fs::read_to_string(&args.config).expect("Failed to read config file");
    let config = load_config(&config_content)?;

    create_project_dir(&args.project_dir, &args.dry_run)?;
    let environment_variables = create_env_vars(&config.environment);

    execute_commands(
        &config.pre_commands,
        &args.project_dir,
        environment_variables.clone(),
        &args.dry_run,
        &args.verbose,
        "Pre-command",
        None,
    )?;

    create_template_files(&config.template_files, &args.dry_run)
        .expect("Failed to create template files.");

    execute_commands(
        &config.dependencies,
        &args.project_dir,
        environment_variables.clone(),
        &args.dry_run,
        &args.verbose,
        "Dependency command",
        Some(&config.variables),
    )?;

    execute_commands(
        &config.post_commands,
        &args.project_dir,
        environment_variables.clone(),
        &args.dry_run,
        &args.verbose,
        "Post-command",
        Some(&config.variables),
    )?;

    Ok(())
}
