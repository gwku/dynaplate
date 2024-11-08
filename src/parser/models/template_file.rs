use crate::parser::TemplateFileType;
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct TemplateFile {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub file_type: TemplateFileType,
}

impl fmt::Display for TemplateFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TemplateFile:\nSource: {}\nDestination: {}\n",
            self.source.to_string_lossy(),
            self.destination.to_string_lossy()
        )
    }
}
