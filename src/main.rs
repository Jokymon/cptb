extern crate clap;
extern crate handlebars;
#[macro_use]
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

static CMAKE_PATH: &str = "C:\\Program Files\\CMake\\bin";
static MINGW_PATH: &str = "C:\\mingw-w64\\x86_64-8.1.0-win32-seh-rt_v6-rev0\\mingw64\\bin";

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
        .subcommand(
            SubCommand::with_name("build")
                .about("Build the current project")
        )
        .get_matches();

    if let Some(new_matches) = matches.subcommand_matches("new") {
        let object_name = new_matches.value_of("object_name").unwrap();

        fs::create_dir(object_name).expect("Couldn't create the directory");
        let reg = Handlebars::new();

        copy_template_file(&reg, "CMakeLists.txt", object_name);

        let src_dir_path = format!("{}/{}", object_name, "src");
        std::fs::create_dir(src_dir_path).expect("Couldn't create project subdirectory 'src'");
        copy_template_file(&reg, "src/main.cpp", object_name);
    }
    else if let Some(_) = matches.subcommand_matches("build") {
        let current_path_var = match env::var("PATH") {
            Ok(val) => val,
            Err(_) => String::from(""),
        };

        let new_path_var = format!("{};{};{}", CMAKE_PATH, MINGW_PATH, current_path_var);

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
}
