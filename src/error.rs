use crate::error::CliError::{CreateDir, CreateFile};
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum CliError {
    CreateDir { dir: String, description: String },
    CreateFile { file: String, description: String },
    MissingFile { file: String },
    ProjectName,
}

impl CliError {
    pub(crate) fn create_dir_err(dir: String, description: String) -> Self {
        CreateDir { dir, description }
    }

    pub(crate) fn create_file_err(file: String, description: String) -> Self {
        CreateFile { file, description }
    }

    pub(crate) fn missing_file_err(file: String) -> Self {
        CliError::MissingFile { file }
    }

    pub(crate) fn project_name_err() -> Self {
        CliError::ProjectName
    }
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CreateDir { dir, description } => {
                write!(f, "Unable to create directory '{}': {}!", dir, description)
            }
            CreateFile { file, description } => {
                write!(f, "Unable to write file '{}': {}!", file, description)
            }
            CliError::MissingFile { file } => {
                write!(
                    f,
                    "Unable to write file '{}': Source file is missing!",
                    file
                )
            }
            CliError::ProjectName => {
                write!(f, "The project name must start with a letter and contain only alphanumeric characters and underscores!")
            }
        }
    }
}
