use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::error::CptbError;

#[derive(Deserialize)]
pub struct CMakeEntry {
    pub name: String,
    pub path: String,
    pub generator: Option<String>,
}

#[derive(Deserialize)]
pub struct CompilerEntry {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct KitEntry {
    pub name: String,
    pub compiler: String,
    pub cmake: String,
}

#[derive(Deserialize)]
pub struct KitsFileStructure {
    pub compilers: HashMap<String, CompilerEntry>,
    pub cmake: HashMap<String, CMakeEntry>,
    pub kits: HashMap<String, KitEntry>,
}

#[derive(Deserialize)]
struct CptbSettings {
    pub default_kit: String,
}

pub struct Settings {
    pub kits: KitsFileStructure,
    pub default_kit: String,
}

fn get_kits<P: AsRef<Path>>(settings_dir: P) -> Result<KitsFileStructure, CptbError> {
    let kits_file_path = settings_dir.as_ref().join("kits.json");
    let file = File::open(kits_file_path)?;
    let reader = BufReader::new(file);
    let kits: KitsFileStructure = serde_json::from_reader(reader)?;
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

    pub fn from_home() -> Result<Settings, CptbError> {
        let home_dir =
            dirs::home_dir().expect("cptb is not supported on platforms without Home directories");
        let cptb_config_dir = format!("{}/{}", home_dir.to_str().expect(""), ".cptb");
        Settings::from_path(&cptb_config_dir)
    }

    pub fn default_cmake_dir(&self) -> Option<String> {
        let default_kit = self.kits.kits.get(&self.default_kit)?;
        let cmake_entry = self.kits.cmake.get(&default_kit.cmake)?;
        Some(cmake_entry.path.clone())
    }

    pub fn default_cmake_generator(&self) -> Option<String> {
        let default_kit = self.kits.kits.get(&self.default_kit)?;
        let cmake_entry = self.kits.cmake.get(&default_kit.cmake)?;
        cmake_entry.generator.clone()
    }

    pub fn default_toolchain_dir(&self) -> Option<String> {
        let default_kit = self.kits.kits.get(&self.default_kit)?;
        let compiler_entry = self.kits.compilers.get(&default_kit.compiler)?;
        Some(compiler_entry.path.clone())
    }
}
