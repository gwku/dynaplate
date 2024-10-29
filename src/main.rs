mod cli;
mod template;

use crate::cli::Args;
use crate::template::{
    Configuration, Dependency, EnvVar, TemplateFile, TemplateFileType, Variable,
};
use clap::Parser;
use log::{debug, error, info};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::format;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();
    let config_content = fs::read_to_string(&args.config).expect("Failed to read config file");
    let config = load_config(&config_content)?;

    create_project_dir(&args.project_dir)?;

    // execute_pre_commands(&config.pre_commands);

    create_template_files(&config.template_files).expect("Failed to create template files.");

    let environment_variables = create_env_vars(&config.environment);

    execute_dependency_commands(
        &config.dependencies,
        &args.project_dir,
        &config.variables,
        environment_variables,
        &args.dry_run,
        &args.verbose,
    )?;

    // execute_post_commands(&config.pre_commands);

    Ok(())
}

fn create_template_files(files: &Vec<TemplateFile>) -> io::Result<()> {
    info!("Template files: creating...");

    for file in files.iter() {
        match file.file_type {
            TemplateFileType::Folder => {
                fs::create_dir_all(&file.destination)?;
                copy_folder(&file.source, &file.destination)?;
            }
            TemplateFileType::File => {
                if fs::metadata(&file.destination)
                    .map(|meta| meta.is_dir())
                    .unwrap_or(false)
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!(
                            "Destination {} is a directory (while file_type is File)",
                            file.destination
                        ),
                    ));
                }
                fs::copy(&file.source, &file.destination)?;
            }
        }
        info!("Template files: created {}", file.destination);
    }
    info!("Template files have been created!");
    Ok(())
}

fn copy_folder<P: AsRef<Path>>(source: P, destination: P) -> io::Result<()> {
    let source_path = source.as_ref();
    let destination_path = destination.as_ref();

    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get file name"))?;

        let dest_path = destination_path.join(file_name);

        if entry_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_folder(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}

fn execute_pre_commands(commands: &Vec<template::Command>) {
    todo!()
}

fn execute_post_commands(commands: &Vec<template::Command>) {
    todo!()
}

fn load_config(config_content: &str) -> Result<Configuration, Box<dyn Error>> {
    serde_yml::from_str(config_content)
        .map_err(|e| format!("Failed to parse config file: {}", e).into())
}

fn create_project_dir(project_dir: &PathBuf) -> Result<(), io::Error> {
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
        info!("Project directory created at: {:?}", project_dir);
    } else {
        info!("Project directory already exists: {:?}", project_dir);
    }
    Ok(())
}

fn create_env_vars(env_vars: &[EnvVar]) -> HashMap<String, String> {
    env_vars
        .iter()
        .map(|var| {
            env::set_var(&var.name, &var.value);
            (var.name.clone(), var.value.clone())
        })
        .collect()
}

fn execute_dependency_commands(
    dependencies: &[Dependency],
    project_dir: &PathBuf,
    variables: &[Variable],
    envs: HashMap<String, String>,
    dry_run: &bool,
    verbose: &bool,
) -> Result<(), String> {
    for dep in dependencies {
        debug!("Command: handling: {:?}.", dep);
        if dep.is_applicable(variables).unwrap_or(false) {
            let args: Vec<&str> = dep.command.split_whitespace().collect();
            if args.is_empty() {
                return Err(format!(
                    "Command: dependency {} has empty command.",
                    dep.name
                ));
            }

            let mut cmd = Command::new(args[0]);
            cmd.current_dir(project_dir).envs(&envs);

            if !*verbose {
                cmd.stdout(Stdio::null()).stderr(Stdio::null());
            };

            if args.len() > 1 {
                cmd.args(&args[1..]);
            }

            if *dry_run {
                info!("Command: {} not executed (dry run).\n{:?}", dep.name, cmd);
                continue;
            }

            match cmd.status() {
                Ok(status) if status.success() => {
                    info!("Command {} executed successfully.", dep.name);
                }
                Ok(status) => {
                    error!("Command '{}' failed with status: {:?}", dep.name, status);
                    return Err(
                        format!("Execution stopped due to command failure: {:?}", cmd).into(),
                    );
                }
                Err(e) => {
                    error!("Command: failed to execute {}: {}", dep.name, e);
                    return Err(format!("Failed to execute command {}: {}", dep.name, e));
                }
            }
        } else {
            debug!("Command: {} is not applicable.", dep.name);
        }
    }
    info!("-------------------------------------------");
    info!(
        "Command: Finished executing ({} commands).",
        dependencies.len()
    );
    Ok(())
}
