use std::fmt;
use serde::Deserialize;

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