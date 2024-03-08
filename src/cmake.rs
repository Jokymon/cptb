use crate::settings::Settings;
use std::env;
use std::process::Command;

pub struct CmakeBuilder {
    generator: String,
    path_variable: String,
}

impl CmakeBuilder {
    pub fn from_settings(settings: &Settings) -> CmakeBuilder {
        CmakeBuilder::from_toolchain(settings, &settings.default_kit)
    }

    pub fn from_toolchain(settings: &Settings, toolchain_id: &str) -> CmakeBuilder {
        let cmake_dir = settings
            .cmake_dir(toolchain_id)
            .expect("A cmake dir is required");
        let toolchain_dir = settings
            .toolchain_dir(toolchain_id)
            .expect("A toolchain dir is required");
        let cmake_generator = settings
            .cmake_generator(toolchain_id)
            .expect("A generator is required");

        let current_path_var = match env::var("PATH") {
            Ok(val) => val,
            Err(_) => String::from(""),
        };
        let new_path_var = format!("{};{};{}", cmake_dir, toolchain_dir, current_path_var);
        CmakeBuilder {
            generator: cmake_generator,
            path_variable: new_path_var,
        }
    }

    pub fn generate(&self, source_dir: &str, build_dir: &str, debug_build: bool) {
        let mut build_arguments = vec!["-S", source_dir, "-B", build_dir, "-G", &self.generator];
        if debug_build {
            build_arguments.push("-D");
            build_arguments.push("CMAKE_BUILD_TYPE=Debug");
        }
        let cmake_status = Command::new("cmake")
            .args(build_arguments)
            .env("PATH", &self.path_variable)
            .status()
            .expect("Couldn't call the CMake executable");
        println!("Exit status of CMake generate: {}", cmake_status);
    }

    pub fn build(&self, build_dir: &str) {
        let build_status = Command::new("cmake")
            .args(&["--build", build_dir])
            .env("PATH", &self.path_variable)
            .status()
            .expect("Couldn't call the CMake executable");
        println!("Exit status of CMake/Build: {}", build_status);
    }
}
