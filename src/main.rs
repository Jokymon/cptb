extern crate clap;
extern crate dirs;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod error;
mod project_builder;
mod settings;

use clap::{App, Arg, SubCommand};
use std::env;
use std::process::Command;

use error::CptbError;

fn main() -> Result<(), CptbError> {
    let home_dir =
        dirs::home_dir().expect("cptb is not supported on platforms without Home directories");
    let cptb_config_dir = format!("{}/{}", home_dir.to_str().expect(""), ".cptb");
    let settings = settings::Settings::from_path(&cptb_config_dir)?;

    let cmake_dir = settings.default_cmake_dir().expect("A cmake dir is required");
    let cmake_generator = settings.default_cmake_generator();
    let toolchain_dir = settings.default_toolchain_dir().expect("A toolchain dir is required");

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
        if let Some(generator) = cmake_generator {
            cmake_parameters.push("-G");
            cmake_parameters.push(&generator);
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
