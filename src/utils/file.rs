use crate::parser::{TemplateFile, TemplateFileType, Variable};
use crate::utils::error::UtilsResult;
use crate::utils::variable::replace_variables;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn ensure_project_existence(project_dir: &PathBuf) -> UtilsResult<()> {
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
        Ok(())
    } else {
        Ok(())
    }
}

pub fn copy_template_files(files: &[TemplateFile], variables: &[Variable]) -> UtilsResult<()> {
    println!("Template files: processing...");

    for file in files.iter() {
        let file_source = PathBuf::from(replace_variables(
            &file.source.display().to_string(),
            &variables,
            &false,
        )?);

        let file_destination = PathBuf::from(replace_variables(
            &file.destination.display().to_string(),
            &variables,
            &false,
        )?);

        match file.file_type {
            TemplateFileType::Folder => {
                // Create destination folder if it does not exist
                if fs::exists(&file_destination).is_err()
                    && fs::create_dir_all(&file_destination).is_err()
                {
                    eprintln!(
                        "Template files: failed to create destination folder: '{}'",
                        &file_destination.to_string_lossy()
                    );
                }

                match copy_folder_contents(&file_source, &file_destination) {
                    Ok(_) => {
                        println!(
                            "Template files: copied contents of folder '{}' to '{}'",
                            &file_source.to_string_lossy(),
                            &file_destination.to_string_lossy()
                        )
                    }
                    Err(_) => {
                        eprintln!(
                            "Template files: failed to copy contents of folder '{}' to '{}'",
                            &file_source.to_string_lossy(),
                            &file_destination.to_string_lossy()
                        );
                    }
                }
            }
            TemplateFileType::File => {
                // Assert that the specified destination is not a folder
                if fs::metadata(&file_destination)
                    .map(|meta| meta.is_dir())
                    .unwrap_or(false)
                {
                    eprintln!(
                        "Destination {} is a directory (while specified file_type is '{}')",
                        &file_destination.to_string_lossy(),
                        &file.file_type
                    );
                }
                match fs::copy(&file_source, &file_destination) {
                    Ok(_) => {
                        println!(
                            "Template files: copied file '{}' to '{}'",
                            &file_source.to_string_lossy(),
                            &file_destination.to_string_lossy()
                        )
                    }
                    Err(e) => {
                        eprintln!(
                            "Template files: failed to copy file '{}' to '{}': {}",
                            &file_source.to_string_lossy(),
                            &file_destination.to_string_lossy(),
                            e
                        );
                    }
                };
            }
        }
    }
    println!("Template files have been processed!");
    
    Ok(())
}

pub fn copy_folder_contents<P: AsRef<Path>>(source: P, destination: P) -> io::Result<()> {
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
            copy_folder_contents(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}
