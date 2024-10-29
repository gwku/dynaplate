use serde::Deserialize;
use std::fmt;

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
