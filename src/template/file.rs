use log::info;
use serde::Deserialize;
use std::path::Path;
use std::{fmt, fs, io};

#[derive(Debug, Deserialize)]
pub struct TemplateFile {
    pub source: String,
    pub destination: String,
    pub file_type: TemplateFileType,
}

impl fmt::Display for TemplateFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TemplateFile:\nSource: {}\nDestination: {}\n",
            self.source, self.destination
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateFileType {
    Folder,
    File,
}

impl fmt::Display for TemplateFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateFileType::Folder => write!(f, "Folder"),
            TemplateFileType::File => write!(f, "File"),
        }
    }
}

pub fn create_template_files(files: &Vec<TemplateFile>, dry_run: &bool) -> io::Result<()> {
    info!("Template files: creating...");

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
        info!("Template files: created {}", file.destination);
    }
    info!("Template files have been created!");
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
