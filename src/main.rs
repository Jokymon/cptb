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

fn copy_template_file(reg: &handlebars::Handlebars, rel_src_path: &str, project_name: &str) {
    let template = Asset::get(rel_src_path).unwrap();
    let mut template_file = File::create(format!("{}/{}", project_name, rel_src_path)).unwrap();
    let file_content = reg
        .render_template(
            std::str::from_utf8(template.as_ref()).unwrap(),
            &json!({ "projectname": project_name })
        )
        .unwrap();
    template_file.write_all(file_content.as_ref()).unwrap();
}

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

        copy_template_file(&reg, "CMakeLists.txt", object_name);
        copy_template_file(&reg, "main.cpp", object_name);
    }
}
