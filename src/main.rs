extern crate clap;
extern crate dirs;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::Command;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Asset;

#[derive(Serialize)]
struct TemplateParameters {
    projectname: String,
    static_build: bool,
}

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

fn copy_template_file(
    reg: &handlebars::Handlebars,
    rel_src_path: &str,
    parameters: &TemplateParameters,
) {
    let template = Asset::get(rel_src_path).unwrap();
    let mut template_file =
        File::create(format!("{}/{}", parameters.projectname, rel_src_path)).unwrap();
    let file_content = reg
        .render_template(std::str::from_utf8(template.as_ref()).unwrap(), parameters)
        .unwrap();
    template_file.write_all(file_content.as_ref()).unwrap();
}

fn main() {
    let home_dir =
        dirs::home_dir().expect("cptb is not supported on platforms without Home directories");
    let cptb_config_dir = format!("{}/{}", home_dir.to_str().expect(""), ".cptb");

    let kits_file_path = format!("{}/{}", cptb_config_dir, "kits.json");
    let file = File::open(kits_file_path).expect("Couldn't find the kits.json file");
    let reader = BufReader::new(file);
    let kits: HashMap<String, Kit> = serde_json::from_reader(reader).expect("");
    let kit = kits.get("cmake-3-17_mingw-8-1").expect("");

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

        let template_parameters = TemplateParameters {
            projectname: object_name.to_string(),
            static_build: !new_matches.is_present("non-static"),
        };

        fs::create_dir(object_name).expect("Couldn't create the directory");
        let reg = Handlebars::new();

        copy_template_file(&reg, "CMakeLists.txt", &template_parameters);

        let src_dir_path = format!("{}/{}", object_name, "src");
        std::fs::create_dir(src_dir_path).expect("Couldn't create project subdirectory 'src'");
        copy_template_file(&reg, "src/main.cpp", &template_parameters);
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
}
