use log::info;
use std::path::PathBuf;
use std::{fs, io};

pub fn create_project_dir(project_dir: &PathBuf, dry_run: &bool) -> Result<(), io::Error> {
    if !project_dir.exists() {
        if !*dry_run {
            fs::create_dir_all(project_dir)?;
        }
        info!("Project directory created at: {:?}", project_dir);
    } else {
        info!("Project directory already exists: {:?}", project_dir);
    }
    Ok(())
}
