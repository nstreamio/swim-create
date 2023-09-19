use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum CliError {
    CreateDir { dir: String, description: String },
    CreateFile { file: String, description: String },
    ProjectName,
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::CreateDir { dir, description } => {
                write!(f, "Unable to create directory '{}': {}!", dir, description)
            }
            CliError::CreateFile { file, description } => {
                write!(f, "Unable to write file '{}': {}!", file, description)
            }
            CliError::ProjectName => {
                write!(f, "The project name must start with a letter and contain only alphanumeric characters and underscores!")
            }
        }
    }
}
