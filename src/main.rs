mod file_names;

use std::fs;
use clap::Parser;
use boilerplate::Boilerplate;
use crate::file_names::{FILE_WRITE_ERROR, FOLDER_CREATE_ERROR};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of the project
    name: String,
    /// The port of the Swim application
    #[arg(long, default_value_t = 9001)]
    port: u16,
    /// The version of the Swim server dependencies
    #[arg(long, default_value = "4.0.1")]
    swim_version: String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "settings.gradle")]
struct SettingsGradle<'a> {
    name: &'a String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "build.gradle")]
struct BuildGradle<'a, 'b> {
    name: &'a String,
    swim_version: &'b String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "MainPlane.java")]
struct MainPlaneJava<'a> {
    name: &'a String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "module-info.java")]
struct ModuleInfoJava<'a> {
    name: &'a String,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "server.recon")]
struct ServerRecon<'a> {
    name: &'a String,
    port: u16,
}

#[derive(Boilerplate)]
#[boilerplate(filename = "gradlew")]
struct Gradlew;

#[derive(Boilerplate)]
#[boilerplate(filename = "gradlew.bat")]
struct GradlewBat;

#[derive(Boilerplate)]
#[boilerplate(filename = ".gitignore")]
struct Gitignore;

fn main() {
    let args = Args::parse();

    println!("Name:  {}", args.name);
    println!("Port: {}", args.port);
    println!("Swim version: {}", args.swim_version);

    fs::create_dir(&args.name).unwrap();
    fs::create_dir_all(format!("{0}/src/main/java/{0}", args.name)).expect(FOLDER_CREATE_ERROR);
    fs::create_dir_all(format!("{0}/src/main/resources", args.name)).expect(FOLDER_CREATE_ERROR);

    fs::write(format!("{0}/settings.gradle", args.name), SettingsGradle { name: &args.name }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{0}/build.gradle", args.name), BuildGradle { name: &args.name, swim_version: &args.swim_version }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{0}/.gitignore", args.name), Gitignore.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{0}/gradlew", args.name), Gradlew.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{0}/gradlew.bat", args.name), GradlewBat.to_string()).expect(FILE_WRITE_ERROR);

    fs::write(format!("{0}/src/main/java/{0}/MainPlane.java", args.name), MainPlaneJava { name: &args.name }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{0}/src/main/java/module-info.java", args.name), ModuleInfoJava { name: &args.name }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{0}/src/main/resources/server.recon", args.name), ServerRecon { name: &args.name, port: args.port }.to_string()).expect(FILE_WRITE_ERROR);
}

