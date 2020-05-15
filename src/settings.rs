use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::error::CptbError;

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

#[derive(Deserialize)]
struct CptbSettings {
    default_kit: String,
}

pub struct Settings {
    kits: HashMap<String, Kit>,
    default_kit: String
}

fn get_kits<P: AsRef<Path>>(settings_dir: P) -> Result<HashMap<String, Kit>, CptbError> {
    let kits_file_path = settings_dir.as_ref().join("kits.json");
    let file = File::open(kits_file_path)?;
    let reader = BufReader::new(file);
    let kits: HashMap<String, Kit> = serde_json::from_reader(reader)?;
    Ok(kits)
}

fn get_settings<P: AsRef<Path>>(settings_dir: P) -> Result<CptbSettings, CptbError> {
    let settings_file_path = settings_dir.as_ref().join("settings.json");
    let file = File::open(settings_file_path)?;
    let reader = BufReader::new(file);
    let settings: CptbSettings = serde_json::from_reader(reader)?;
    Ok(settings)
}

impl Settings {
    pub fn from_path<P: AsRef<Path>>(settings_dir: P) -> Result<Settings, CptbError> {
        Ok(Settings {
            kits: get_kits(&settings_dir)?,
            default_kit: get_settings(&settings_dir)?.default_kit,
        })
    }

    pub fn default_cmake_dir(&self) -> String {
        let default_kit = self.kits.get(&self.default_kit);
        match default_kit {
            Some(kit) => kit.cmake.path.clone(),
            None => "".to_owned()
        }
    }

    pub fn default_cmake_generator(&self) -> Option<String> {
        let default_kit = self.kits.get(&self.default_kit);
        default_kit?.cmake.generator.clone()
    }

    pub fn default_toolchain_dir(&self) -> String {
        let default_kit = self.kits.get(&self.default_kit);
        match default_kit {
            Some(kit) => kit.toolchain.clone(),
            None => "".to_owned()
        }
    }
}
