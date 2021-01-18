use git2::Repository;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use tempdir::TempDir;

use crate::cmake::CmakeBuilder;
use crate::error::CptbError;
use crate::settings::Settings;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Asset;

#[derive(Serialize)]
struct TemplateParameters {
    projectname: String,
    toolchain: String,
    target: String,
    cmake_version: String,
    static_build: bool,
}

#[derive(Deserialize)]
struct SystemDescription {
    toolchain: String,
    target: String,
    cmake_version: String,
}

fn copy_asset_to_target(source_template_path: &str, target_file_path: &str) {
    let template = Asset::get(source_template_path).unwrap();
    let mut target_file = File::create(target_file_path).unwrap();
    target_file.write_all(template.as_ref()).unwrap();
}

fn copy_template_file_to_target(
    reg: &handlebars::Handlebars,
    source_template_path: &str,
    target_file_path: &str,
    parameters: &TemplateParameters,
) {
    let template = Asset::get(source_template_path).unwrap();
    let mut template_file =
        File::create(format!("{}/{}", parameters.projectname, target_file_path)).unwrap();
    let file_content = reg
        .render_template(std::str::from_utf8(template.as_ref()).unwrap(), parameters)
        .unwrap();
    template_file.write_all(file_content.as_ref()).unwrap();
}

fn copy_template_file(
    reg: &handlebars::Handlebars,
    source_template_path: &str,
    parameters: &TemplateParameters,
) {
    copy_template_file_to_target(reg, source_template_path, source_template_path, parameters);
}

fn download_file(source_url: &str, target_file_path: &str) -> Result<(), CptbError> {
    let body: String = ureq::get(source_url).call()?.into_string()?;

    let mut file = File::create(target_file_path)?;
    file.write(body.as_bytes())?;

    Ok(())
}

fn get_system_description() -> SystemDescription {
    let tmp_dir = TempDir::new("system_description").unwrap();

    copy_asset_to_target(
        "detection/CMakeLists.txt",
        tmp_dir.path().join("CMakeLists.txt").to_str().unwrap(),
    );
    copy_asset_to_target(
        "detection/main.cpp",
        tmp_dir.path().join("main.cpp").to_str().unwrap(),
    );

    let cmake_builder = CmakeBuilder::from_settings(&Settings::from_home().unwrap());
    cmake_builder.generate(
        tmp_dir.path().to_str().unwrap(),
        tmp_dir.path().join("build").to_str().unwrap(),
    );

    let build_dir = tmp_dir.path().join("build");
    let status_file_path = build_dir.join("status.json");
    let file = File::open(status_file_path.to_str().unwrap()).unwrap();
    let reader = BufReader::new(file);
    let system_description: SystemDescription = serde_json::from_reader(reader).unwrap();

    system_description
}

pub fn cptb_new_command(project_name: &str, static_build: bool) {
    let system_description = get_system_description();

    let template_parameters = TemplateParameters {
        projectname: project_name.to_string(),
        static_build: static_build,
        toolchain: system_description.toolchain,
        target: system_description.target,
        cmake_version: system_description.cmake_version,
    };

    fs::create_dir(project_name).expect("Couldn't create the directory");
    let reg = Handlebars::new();

    copy_template_file(&reg, "CMakeLists.txt", &template_parameters);

    let src_dir_path = format!("{}/{}", project_name, "src");
    std::fs::create_dir(src_dir_path).expect("Couldn't create project subdirectory 'src'");
    copy_template_file(&reg, "src/main.cpp", &template_parameters);
    copy_template_file_to_target(&reg, "_gitignore", ".gitignore", &template_parameters);

    let cmake_dir_path = format!("{}/{}", project_name, "cmake");
    std::fs::create_dir(cmake_dir_path).expect("Couldn't create project subdirectory 'cmake'");
    let cpm_file_path = format!("{}/{}/{}", project_name, "cmake", "CPM.cmake");
    download_file(
        "https://github.com/TheLartians/CPM.cmake/releases/latest/download/CPM.cmake",
        &cpm_file_path,
    )
    .unwrap();

    let _repo = match Repository::init(project_name) {
        Ok(repo) => repo,
        Err(e) => panic!("Couldn't initialize repository in the new project: {}", e),
    };
}
