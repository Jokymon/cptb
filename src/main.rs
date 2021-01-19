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

use clap::{App, Arg, SubCommand};

use cmake::CmakeBuilder;
use error::CptbError;

fn main() -> Result<(), CptbError> {
    let settings = settings::Settings::from_home()?;
    let cmake_builder = CmakeBuilder::from_settings(&settings);

    let matches = App::new("cptb")
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
        .subcommand(SubCommand::with_name("build").about("Build the current project"))
        .get_matches();

    if let Some(new_matches) = matches.subcommand_matches("new") {
        let object_name = new_matches.value_of("object_name").unwrap();
        let with_tests = new_matches.is_present("with-tests");
        let static_build = !new_matches.is_present("non-static");

        project_builder::cptb_new_command(object_name, with_tests, static_build);
    } else if let Some(_) = matches.subcommand_matches("build") {
        cmake_builder.generate(".", "build");
        cmake_builder.build("build");
    }

    Ok(())
}
