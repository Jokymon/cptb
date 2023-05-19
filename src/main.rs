#[macro_use]
extern crate clap;
extern crate dirs;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cmake;
mod error;
mod project_builder;
mod settings;
mod toolchains;

use clap::{App, Arg, SubCommand};

use cmake::CmakeBuilder;
use error::CptbError;

use std::env;
use std::process::Command;

fn main() -> Result<(), CptbError> {
    let settings: settings::Settings = settings::Settings::from_home()?;
    let cmake_builder = CmakeBuilder::from_settings(&settings);

    let app = App::new("cptb")
        .version(crate_version!())
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
                .arg(
                    Arg::with_name("with-tests")
                        .long("with-tests")
                        .help("Add basic structure for unit testing with Catch2"),
                )
                .arg(Arg::with_name("object_name").required(true).index(1)),
        )
        .subcommand(SubCommand::with_name("build")
            .about("Build the current project")
            .arg(
                Arg::with_name("debug")
                    .long("debug")
                    .help("Build the project in DEBUG configuration")
            )
        )
        .subcommand(
            SubCommand::with_name("buildenv")
                .about("Start a new shell with all environment variables set according to the build environment"),
        )
        .subcommand(
            SubCommand::with_name("toolchain")
            .about("Add, remove and query installed toolchains")
            .subcommand(
                SubCommand::with_name("list")
                .about("Show all installed toolchains")
            )
        );

    let matches = app.get_matches();

    if let Some(new_matches) = matches.subcommand_matches("new") {
        let object_name = new_matches.value_of("object_name").unwrap();
        let with_tests = new_matches.is_present("with-tests");
        let static_build = !new_matches.is_present("non-static");

        project_builder::cptb_new_command(object_name, with_tests, static_build);
    } else if let Some(new_matches) = matches.subcommand_matches("build") {
        let debug_build = new_matches.is_present("debug");
        let build_dir = if debug_build {
            "build-debug" 
        } else {
            "build"
        };

        cmake_builder.generate(".", build_dir, debug_build);
        cmake_builder.build(build_dir);
    } else if let Some(_) = matches.subcommand_matches("buildenv") {
        let cmake_dir = settings
            .default_cmake_dir()
            .expect("A cmake dir is required");
        let toolchain_dir = settings
            .default_toolchain_dir()
            .expect("A toolchain dir is required");

        let current_path_var = match env::var("PATH") {
            Ok(val) => val,
            Err(_) => String::from(""),
        };
        let new_path_var = format!("{};{};{}", cmake_dir, toolchain_dir, current_path_var);

        if cfg!(windows) {
            let current_prompt = match env::var("PROMPT") {
                Ok(val) => val,
                Err(_) => String::from("$P$G"),
            };
            let new_prompt = format!("(cptb build) {}", current_prompt);

            let _status = Command::new("cmd")
                .env("PATH", new_path_var)
                .env("PROMPT", new_prompt)
                .status()
                .expect("Couldn't run the shell executable");
        } else {
            let current_prompt = match env::var("PS1") {
                Ok(val) => val,
                Err(_) => String::from("> "),
            };
            let new_prompt = format!("(cptb build) {}", current_prompt);

            let shell = match env::var("SHELL") {
                Ok(val) => val,
                Err(_) => String::from("/bin/sh"),
            };

            let _status = Command::new(shell)
                .env("PATH", new_path_var)
                .env("PS1", new_prompt)
                .status()
                .expect("Couldn't run the shell executable");
        }
    } else if let Some(toolchain_command) = matches.subcommand_matches("toolchain") {
        if let Some(_) = toolchain_command.subcommand_matches("list") {
            toolchains::print_toolchain_list();
        } else {
            println!("No subcommand for 'toolchain' specified\n");
            println!("{}", toolchain_command.usage());
        }
    }

    Ok(())
}
