use crate::cli::Args;
use crate::parser::traits::CommandTrait;
use crate::parser::{EnvVar, TemplateFile, TemplateFileType, Variable};
use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::{env, fs, io};

mod cli;

mod parser;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config_yaml = fs::read_to_string(&args.config).expect("Failed to read config file");
    let config = parser::from_yaml(&config_yaml)?;

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

pub fn create_project_dir(project_dir: &PathBuf, dry_run: &bool) -> Result<(), io::Error> {
    if !project_dir.exists() {
        if !*dry_run {
            fs::create_dir_all(project_dir)?;
        }
        println!("Project directory created at: {:?}", project_dir);
    } else {
        println!("Project directory already exists: {:?}", project_dir);
    }
    Ok(())
}

pub fn create_template_files(files: &Vec<TemplateFile>, dry_run: &bool) -> io::Result<()> {
    println!("Template files: creating...");

    for file in files.iter() {
        match file.file_type {
            TemplateFileType::Folder => {
                if !*dry_run {
                    fs::create_dir_all(&file.destination)?;
                    copy_folder(&file.source, &file.destination)?;
                }
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
                if !*dry_run {
                    fs::copy(&file.source, &file.destination)?;
                }
            }
        }
        println!("Template files: created {}", file.destination);
    }
    println!("Template files have been created!");
    Ok(())
}

pub fn copy_folder<P: AsRef<Path>>(source: P, destination: P) -> io::Result<()> {
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

pub fn create_env_vars(env_vars: &[EnvVar]) -> HashMap<String, String> {
    env_vars
        .iter()
        .map(|var| {
            env::set_var(&var.name, &var.value);
            (var.name.clone(), var.value.clone())
        })
        .collect()
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
                println!("{}: '{}' is not applicable.", command_type, command.name());
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
            println!(
                "{}: '{}' not executed (dry run).\n{:?}",
                command_type,
                command.name(),
                cmd
            );
            continue;
        }

        match cmd.status() {
            Ok(status) if status.success() => {
                println!(
                    "{} '{}' executed successfully.",
                    command_type,
                    command.name()
                );
            }
            Ok(status) => {
                println!(
                    "{} '{}' failed with status: {:?}",
                    command_type,
                    command.name(),
                    status
                );
                return Err(format!("Execution stopped due to command failure: {:?}", cmd).into());
            }
            Err(e) => {
                println!(
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
    println!("-------------------------------------------");
    println!(
        "{}: Finished executing ({} commands).",
        command_type,
        commands.len()
    );
    Ok(())
}
