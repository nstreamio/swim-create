use std::fmt::{Debug, Display};
use std::{fmt, fs};
use std::error::Error;
use clap::Parser;
use boilerplate::Boilerplate;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the project.
    /// Must start with a letter and contain only alphanumeric characters and underscores.
    #[clap(value_parser = validate_name)]
    name: String,
    /// The port of the Swim application.
    #[arg(short, long, default_value_t = 9001)]
    port: u16,
    /// The version of the Swim server dependencies.
    #[arg(short, long, default_value = "4.0.1")]
    swim_version: String,
}

fn validate_name(name: &str) -> Result<String, CliError> {
    let name = name.trim().to_lowercase();

    if Regex::new(r"^[a-zA-Z_]\w*$").unwrap().is_match(&name) {
        Ok(name)
    } else {
        Err(CliError::ProjectNameError)
    }
}


#[derive(Debug, Clone)]
enum CliError {
    FolderCreateError(String),
    FileWriteError(String),
    ProjectNameError,
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::FolderCreateError(description) => { write!(f, "Unable to create folder: {}!", description) }
            CliError::FileWriteError(description) => { write!(f, "Unable to write file: {}!", description) }
            CliError::ProjectNameError => { write!(f, "The project name must start with a letter and contain only alphanumeric characters and underscores!") }
        }
    }
}


#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/settings.gradle")]
struct SettingsGradle<'a> {
    name: &'a String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/build.gradle")]
struct BuildGradle<'a, 'b> {
    name: &'a String,
    swim_version: &'b String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/src/main/java/example/MainPlane.java")]
struct MainPlaneJava<'a> {
    name: &'a String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/src/main/java/module-info.java")]
struct ModuleInfoJava<'a> {
    name: &'a String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/src/main/resources/server.recon")]
struct ServerRecon<'a> {
    name: &'a String,
    port: u16,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/gradlew")]
struct Gradlew;

#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/gradlew.bat")]
struct GradlewBat;

#[derive(Boilerplate)]
#[boilerplate(filename = "swim-template/.gitignore")]
struct Gitignore;

fn create_dirs(name: &String) -> Result<(), CliError> {
    fs::create_dir(name).map_err(|err| CliError::FolderCreateError(err.to_string()))?;
    fs::create_dir_all(format!("{name}/src/main/java/{name}")).map_err(|err| CliError::FolderCreateError(err.to_string()))?;
    fs::create_dir_all(format!("{name}/src/main/resources")).map_err(|err| CliError::FolderCreateError(err.to_string()))?;
    Ok(())
}


fn write_files(name: &String, swim_version: &String, port: u16) -> Result<(), CliError> {
    fs::write(format!("{name}/setting.gradle"), SettingsGradle { name }.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;
    fs::write(format!("{name}/build.gradle"), BuildGradle { name, swim_version }.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;
    fs::write(format!("{name}/.gitignore"), Gitignore.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;
    fs::write(format!("{name}/gradlew"), Gradlew.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;
    fs::write(format!("{name}/gradlew.bat"), GradlewBat.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;

    fs::write(format!("{name}/src/main/java/{name}/MainPlane.java"), MainPlaneJava { name }.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;
    fs::write(format!("{name}/src/main/java/module-info.java"), ModuleInfoJava { name }.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;
    fs::write(format!("{name}/src/main/resources/server.recon"), ServerRecon { name, port }.to_string()).map_err(|err| CliError::FileWriteError(err.to_string()))?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let name = &args.name;
    let port = args.port;
    let swim_version = &args.swim_version;


    if let Err(err) = create_dirs(name) {
        println!("{}", err);
        std::process::exit(1)
    }

    if let Err(err) = write_files(name, swim_version, port) {
        println!("{}", err);
        std::process::exit(1)
    }

    println!("---Swim project created---");
    println!("Name: {}", name);
    println!("Port: {}", port);
    println!("Swim version: {}", swim_version);
}

