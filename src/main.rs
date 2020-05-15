extern crate clap;
extern crate dirs;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod error;

use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

use error::CptbError;

mod project_builder;

#[derive(Deserialize)]
struct CMakeSettings {
    path: String,
    generator: Option<String>,
}

#[derive(Deserialize)]
struct Kit {
    toolchain: String,
    cmake: CMakeSettings,
}

#[derive(Deserialize)]
struct CptbSettings {
    default_kit: String,
}

fn get_kits<P: AsRef<Path>>(settings_dir: P) -> Result<HashMap<String, Kit>, CptbError> {
    let kits_file_path = settings_dir.as_ref().join("kits.json");
    let file = File::open(kits_file_path)?;
    let reader = BufReader::new(file);
    let kits: HashMap<String, Kit> = serde_json::from_reader(reader)?;
    Ok(kits)
}

fn get_settings<P: AsRef<Path>>(settings_dir: P) -> Result<CptbSettings, CptbError> {
    let settings_file_path = settings_dir.as_ref().join("settings.json");
    let file = File::open(settings_file_path)?;
    let reader = BufReader::new(file);
    let settings: CptbSettings = serde_json::from_reader(reader)?;
    Ok(settings)
}

fn main() -> Result<(), CptbError> {
    let home_dir =
        dirs::home_dir().expect("cptb is not supported on platforms without Home directories");
    let cptb_config_dir = format!("{}/{}", home_dir.to_str().expect(""), ".cptb");

    let kits = get_kits(&cptb_config_dir)?;
    let settings = get_settings(&cptb_config_dir)?;
    let kit = kits.get(&settings.default_kit).expect("");

    let cmake_dir = &kit.cmake.path;
    let toolchain_dir = &kit.toolchain;

    let matches = App::new("cptb")
        .version("0.1")
        .author("Silvan Wegmann")
        .about("C++ helper tool")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create new C++ projects")
                .arg(
                    Arg::with_name("bin")
                        .long("bin")
                        .help("Create an executable project"),
                )
                .arg(
                    Arg::with_name("non-static")
                        .long("non-static")
                        .help("Create a project with dynamically linked libc and libc++"),
                )
                .arg(Arg::with_name("object_name").required(true).index(1)),
        )
        .subcommand(SubCommand::with_name("build").about("Build the current project"))
        .get_matches();

    if let Some(new_matches) = matches.subcommand_matches("new") {
        let object_name = new_matches.value_of("object_name").unwrap();
        let static_build = !new_matches.is_present("non-static");

        project_builder::cptb_new_command(object_name, static_build);
    } else if let Some(_) = matches.subcommand_matches("build") {
        let current_path_var = match env::var("PATH") {
            Ok(val) => val,
            Err(_) => String::from(""),
        };

        let new_path_var = format!("{};{};{}", cmake_dir, toolchain_dir, current_path_var);
        let mut cmake_parameters = vec!["-S", ".", "-B", "build"];
        if let Some(generator) = &kit.cmake.generator {
            cmake_parameters.push("-G");
            cmake_parameters.push(generator);
        }

        let cmake_status = Command::new("cmake")
            .args(&["-S", ".", "-B", "build", "-G", "MinGW Makefiles"])
            .env("PATH", &new_path_var)
            .status()
            .expect("Couldn't call the CMake executable");
        println!("Exit status of CMake: {}", cmake_status);

        let build_status = Command::new("cmake")
            .args(&["--build", "build"])
            .env("PATH", &new_path_var)
            .status()
            .expect("Couldn't call the CMake executable");
        println!("Exit status of CMake/Build: {}", build_status);
    }

    Ok(())
}
