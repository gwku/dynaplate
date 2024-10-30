use std::fmt;
use serde::Deserialize;
use crate::parser::TemplateFileType;

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