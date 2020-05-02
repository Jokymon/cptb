extern crate clap;
extern crate handlebars;
#[macro_use]
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Asset;

fn main() {
    let matches = App::new("cptb")
        .version("1.0")
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
                .arg(Arg::with_name("object_name").required(true).index(1)),
        )
        .get_matches();

    if let Some(new_matches) = matches.subcommand_matches("new") {
        let object_name = new_matches.value_of("object_name").unwrap();

        fs::create_dir(object_name).expect("Couldn't create the directory");
        let reg = Handlebars::new();

        let cmake_template = Asset::get("CMakeLists.txt").unwrap();
        let mut cmake_file = File::create(format!("{}/CMakeLists.txt", object_name)).unwrap();
        let cmake_content = reg
            .render_template(
                std::str::from_utf8(cmake_template.as_ref()).unwrap(),
                &json!({ "projectname": object_name }),
            )
            .unwrap();
        cmake_file.write_all(cmake_content.as_ref()).unwrap();
    }
}
