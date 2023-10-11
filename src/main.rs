mod error;
mod utils;

use crate::utils::*;
use clap::Parser;
use rust_embed::RustEmbed;
use serde::Serialize;
use std::fmt::Debug;
use std::path::Path;

const PROJECT_TEMPLATE_FOLDER: &str = "swim-template";

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
    #[arg(short, long, default_value = "4.1.0.12")]
    swim_version: String,
}

#[derive(RustEmbed)]
#[folder = "templates"]
#[exclude = "swim-template/.git/*"]
struct Templates;

fn main() {
    let args = Args::parse();

    if let Err(err) = create_dir(args.name.clone()) {
        println!("{}", err);
        std::process::exit(1)
    }

    for path in Templates::iter() {
        if let Err(err) = create_file(Path::new(&path.to_string()), &args) {
            println!("{}", err);
            std::process::exit(1)
        }
    }

    println!("---Swim project created---");
    println!("Name: {}", args.name);
    println!("Port: {}", args.port);
    println!("Swim version: {}", args.swim_version);
}
