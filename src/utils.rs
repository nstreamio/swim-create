use crate::error::CliError;
use crate::{Args, Templates, PROJECT_TEMPLATE_FOLDER};
use regex::Regex;
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use tera::Tera;

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

pub(crate) fn create_dir(dir: String) -> Result<(), CliError> {
    fs::create_dir(&dir).map_err(|err| CliError::create_dir_err(dir, err.kind().to_string()))?;
    Ok(())
}

pub(crate) fn create_parent_dirs(file: &str) -> Result<(), CliError> {
    if let Some(parent_dirs) = Path::new(file).parent() {
        fs::create_dir_all(parent_dirs).map_err(|err| {
            CliError::create_dir_err(parent_dirs.display().to_string(), err.kind().to_string())
        })?;
    }
    Ok(())
}

pub(crate) fn create_file(input_file: &Path, args: &Args) -> Result<(), CliError> {
    let output_file = get_output_dir(input_file, args).map_err(|err| {
        CliError::create_file_err(input_file.display().to_string(), err.to_string())
    })?;

    create_parent_dirs(&output_file)?;

    let input_file = Templates::get(&input_file.display().to_string())
        .ok_or(CliError::missing_file_err(output_file.clone()))?;

    let input_text = String::from_utf8(input_file.data.to_vec())
        .map_err(|err| CliError::create_file_err(output_file.clone(), err.to_string()))?;

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
