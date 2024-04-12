mod error;
mod utils;

use crate::utils::*;
use clap::{Parser, ValueEnum};
use rust_embed::RustEmbed;
use serde::Serialize;
use std::fmt::Debug;
use std::path::Path;

#[derive(Parser, Debug, Serialize)]
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
    #[arg(short, long, default_value = "4.2.14")]
    swim_version: String,
    /// The type of the project template.
    #[arg(short, long, value_enum, default_value_t = Template::Gradle)]
    template_type: Template,
}

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Template {
    /// Java template using Gradle.
    #[default]
    Gradle,
    /// Java template with a module using Gradle.
    GradleModule,
    /// Java template using Maven.
    Maven,
    /// Java template with a module using Maven.
    MavenModule,
    /// Rust template.
    Rust,
}

impl Template {
    pub fn get_folder(&self) -> String {
        match self {
            Template::Gradle => "swim-gradle-template".to_string(),
            Template::GradleModule => "swim-gradle-module-template".to_string(),
            Template::Maven => "swim-maven-template".to_string(),
            Template::MavenModule => "swim-maven-module-template".to_string(),
            Template::Rust => "swim-rust-template".to_string(),
        }
    }
}

#[derive(RustEmbed)]
#[folder = "templates"]
struct TemplatesDir;

fn main() {
    let args = Args::parse();

    if let Err(err) = create_dir(args.name.clone()) {
        println!("{}", err);
        std::process::exit(1)
    }

    let template_folder = args.template_type.get_folder();

    for path in TemplatesDir::iter().filter(|path| path.starts_with(&template_folder)) {
        if let Err(err) = create_file(Path::new(&path.to_string()), &args) {
            println!("{}", err);
            std::process::exit(1)
        }
    }

    println!("---Swim project created---");
    println!("Name: {}", args.name);
    println!("Port: {}", args.port);

    match args.template_type {
        Template::Rust => {
            println!("Swim version: pre-release");
        }
        _ => {
            println!("Swim version: {}", args.swim_version);
        }
    }
}
