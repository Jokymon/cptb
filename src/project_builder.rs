use handlebars::Handlebars;
use rust_embed::RustEmbed;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Asset;

#[derive(Serialize)]
struct TemplateParameters {
    projectname: String,
    static_build: bool,
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

pub fn cptb_new_command(project_name: &str, static_build: bool) {
    let template_parameters = TemplateParameters {
        projectname: project_name.to_string(),
        static_build: static_build,
    };

    fs::create_dir(project_name).expect("Couldn't create the directory");
    let reg = Handlebars::new();

    copy_template_file(&reg, "CMakeLists.txt", &template_parameters);

    let src_dir_path = format!("{}/{}", project_name, "src");
    std::fs::create_dir(src_dir_path).expect("Couldn't create project subdirectory 'src'");
    copy_template_file(&reg, "src/main.cpp", &template_parameters);
}
