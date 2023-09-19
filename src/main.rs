mod error;
mod utils;

use crate::utils::*;
use clap::Parser;
use serde::Serialize;
use std::fmt::Debug;
use walkdir::WalkDir;

const PROJECT_TEMPLATE_FOLDER: &str = "templates/swim-template";

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
    #[arg(short, long, default_value = "4.0.1")]
    swim_version: String,
}

fn main() {
    let args = Args::parse();

    for path in WalkDir::new(PROJECT_TEMPLATE_FOLDER)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .map(|entry| entry.into_path())
    {
        // Creates the directories and files recursively
        if path.is_dir() {
            if let Err(err) = create_dir(path, &args) {
                println!("{}", err);
                std::process::exit(1)
            }
        } else if path.is_file() {
            if let Err(err) = create_file(path, &args) {
                println!("{}", err);
                std::process::exit(1)
            }
        }
    }

    println!("---Swim project created---");
    println!("Name: {}", args.name);
    println!("Port: {}", args.port);
    println!("Swim version: {}", args.swim_version);
}
