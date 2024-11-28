use crate::parser::{TemplateFile, TemplateFileType, Variable};
use crate::utils::error::UtilsResult;
use crate::utils::variable::replace_variables;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn ensure_project_existence(project_dir: &PathBuf) -> UtilsResult<()> {
    if !project_dir.exists() {
        fs::create_dir_all(project_dir)?;
    }
    Ok(())
}

pub fn copy_template_files(
    files: &[TemplateFile],
    variables: &[Variable],
    use_filters: &bool,
) -> UtilsResult<()> {
    println!("Template files: processing...");

    for file in files.iter() {
        let file_source = PathBuf::from(replace_variables(
            &file.source.display().to_string(),
            variables,
            &false,
        )?);

        let file_destination = PathBuf::from(replace_variables(
            &file.destination.display().to_string(),
            variables,
            &false,
        )?);

        match file.file_type {
            TemplateFileType::Folder => {
                if let Err(e) = fs::create_dir_all(&file_destination) {
                    eprintln!(
                        "Template files: failed to create destination folder '{}': {}",
                        &file_destination.to_string_lossy(),
                        e
                    );
                }

                match copy_folder_contents_with_gitignore(
                    &file_source,
                    &file_destination,
                    use_filters,
                ) {
                    Ok(_) => {
                        println!(
                            "Template files: copied contents of folder '{}' to '{}'",
                            &file_source.to_string_lossy(),
                            &file_destination.to_string_lossy()
                        )
                    }
                    Err(e) => {
                        eprintln!(
                            "Template files: failed to copy contents of folder '{}' to '{}': {}",
                            &file_source.to_string_lossy(),
                            &file_destination.to_string_lossy(),
                            e
                        );
                    }
                }
            }
            TemplateFileType::File => {
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

pub fn copy_folder_contents_with_gitignore<P: AsRef<Path>>(
    source: P,
    destination: P,
    use_filters: &bool,
) -> io::Result<()> {
    let source_path = source.as_ref();
    let destination_path = destination.as_ref();

    print!(
        "Use filters ({}) for path: {}\n",
        *use_filters,
        &source_path.to_str().unwrap()
    );

    let walker = WalkBuilder::new(source_path)
        .follow_links(true)
        .hidden(false)
        .parents(true)
        .ignore(*use_filters)
        .git_ignore(*use_filters)
        .git_global(*use_filters)
        .git_exclude(*use_filters)
        .build();

    for entry in walker {
        match entry {
            Ok(result) => {
                let entry_path = result.path();
                match entry_path.strip_prefix(source_path) {
                    Ok(relative_path) => {
                        let dest_path = destination_path.join(relative_path);

                        if entry_path.is_dir() {
                            if let Err(e) = fs::create_dir_all(&dest_path) {
                                eprintln!(
                                    "Failed to create directory '{}': {}",
                                    dest_path.display(),
                                    e
                                );
                                continue;
                            }
                        } else {
                            if let Some(parent) = dest_path.parent() {
                                if let Err(e) = fs::create_dir_all(parent) {
                                    eprintln!(
                                        "Failed to create parent directory '{}': {}",
                                        parent.display(),
                                        e
                                    );
                                    continue;
                                }
                            }
                            if let Err(e) = fs::copy(&entry_path, &dest_path) {
                                eprintln!(
                                    "Failed to copy file '{}' to '{}': {}",
                                    entry_path.display(),
                                    dest_path.display(),
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to compute relative path for '{}': {}",
                            entry_path.display(),
                            e
                        );
                    }
                };
            }
            Err(e) => eprintln!("Error encountered during directory traversal: {}", e),
        }
    }
    Ok(())
}
