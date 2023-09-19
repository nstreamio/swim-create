use crate::error::CliError;
use crate::{Args, PROJECT_TEMPLATE_FOLDER};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use tera::Tera;
use walkdir::DirEntry;

pub(crate) fn validate_name(name: &str) -> Result<String, CliError> {
    let name = name.trim().to_lowercase();

    if Regex::new(r"^[a-zA-Z_]\w*$")
        .expect("Invalid regex!")
        .is_match(&name)
    {
        Ok(name)
    } else {
        Err(CliError::ProjectName)
    }
}

pub(crate) fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub(crate) fn create_dir(input_dir: PathBuf, args: &Args) -> Result<(), CliError> {
    let output_dir =
        Path::new(&args.name).join(input_dir.strip_prefix(PROJECT_TEMPLATE_FOLDER).map_err(
            |err| CliError::CreateDir {
                dir: input_dir.display().to_string(),
                description: err.to_string(),
            },
        )?);
    fs::create_dir(output_dir.display().to_string()).map_err(|err| CliError::CreateDir {
        dir: output_dir.display().to_string(),
        description: err.kind().to_string(),
    })?;
    Ok(())
}

pub(crate) fn create_file(input_file: PathBuf, args: &Args) -> Result<(), CliError> {
    let output_file =
        Path::new(&args.name).join(input_file.strip_prefix(PROJECT_TEMPLATE_FOLDER).map_err(
            |err| CliError::CreateFile {
                file: input_file.display().to_string(),
                description: err.to_string(),
            },
        )?);
    let input_text = fs::read_to_string(input_file).map_err(|err| CliError::CreateFile {
        file: output_file.display().to_string(),
        description: err.to_string(),
    })?;

    let mut tera = Tera::default();
    tera.add_raw_template("file", &input_text)
        .map_err(|err| CliError::CreateFile {
            file: output_file.display().to_string(),
            description: err.to_string(),
        })?;
    let context = tera::Context::from_serialize(args).map_err(|err| CliError::CreateFile {
        file: output_file.display().to_string(),
        description: err.to_string(),
    })?;
    let output_text = tera
        .render("file", &context)
        .map_err(|err| CliError::CreateFile {
            file: output_file.display().to_string(),
            description: err.to_string(),
        })?;

    fs::write(&output_file, output_text).map_err(|err| CliError::CreateFile {
        file: output_file.display().to_string(),
        description: err.to_string(),
    })?;

    Ok(())
}
