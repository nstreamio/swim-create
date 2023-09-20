use crate::error::CliError;
use crate::{Args, PROJECT_TEMPLATE_FOLDER};
use regex::Regex;
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::Path;
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
        Err(CliError::project_name_err())
    }
}

pub(crate) fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub(crate) fn create_dir(input_dir: &Path, args: &Args) -> Result<(), CliError> {
    let output_dir = get_output_dir(input_dir, args).map_err(|err| {
        CliError::create_dir_err(input_dir.display().to_string(), err.to_string())
    })?;
    fs::create_dir(&output_dir)
        .map_err(|err| CliError::create_dir_err(output_dir, err.kind().to_string()))?;
    Ok(())
}

pub(crate) fn create_file(input_file: &Path, args: &Args) -> Result<(), CliError> {
    let output_file = get_output_dir(input_file, args).map_err(|err| {
        CliError::create_file_err(input_file.display().to_string(), err.to_string())
    })?;
    let input_text = fs::read_to_string(input_file)
        .map_err(|err| CliError::create_file_err(output_file.clone(), err.kind().to_string()))?;
    let output_text = replace_text(&input_text, args)
        .map_err(|err| CliError::create_file_err(output_file.clone(), err.to_string()))?;
    fs::write(&output_file, output_text)
        .map_err(|err| CliError::create_file_err(output_file, err.kind().to_string()))?;

    Ok(())
}

pub(crate) fn get_output_dir(input_dir: &Path, args: &Args) -> Result<String, Box<dyn Error>> {
    let output_dir = Path::new(&args.name).join(input_dir.strip_prefix(PROJECT_TEMPLATE_FOLDER)?);
    replace_text(&output_dir.display().to_string(), args)
}

pub(crate) fn replace_text(
    template_text: &str,
    values: impl Serialize,
) -> Result<String, Box<dyn Error>> {
    let mut tera = Tera::default();
    tera.add_raw_template("text", template_text)?;
    let context = tera::Context::from_serialize(values)?;
    Ok(tera.render("text", &context)?)
}
