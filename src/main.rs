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

fn main() {
    let args = Args::parse();

    let name = &args.name;
    let port = args.port;
    let swim_version = &args.swim_version;

    println!("Name:  {}", name);
    println!("Port: {}", port);
    println!("Swim version: {}", swim_version);

    fs::create_dir(name).expect(FOLDER_CREATE_ERROR);
    fs::create_dir_all(format!("{name}/src/main/java/{name}")).expect(FOLDER_CREATE_ERROR);
    fs::create_dir_all(format!("{name}/src/main/resources")).expect(FOLDER_CREATE_ERROR);

    fs::write(format!("{name}/setting.gradle"), SettingsGradle { name }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{name}/build.gradle"), BuildGradle { name, swim_version }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{name}/.gitignore"), Gitignore.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{name}/gradlew"), Gradlew.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{name}/gradlew.bat"), GradlewBat.to_string()).expect(FILE_WRITE_ERROR);

    fs::write(format!("{name}/src/main/java/{name}/MainPlane.java"), MainPlaneJava { name }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{name}/src/main/java/module-info.java"), ModuleInfoJava { name }.to_string()).expect(FILE_WRITE_ERROR);
    fs::write(format!("{name}/src/main/resources/server.recon"), ServerRecon { name, port }.to_string()).expect(FILE_WRITE_ERROR);
}
