use std::{fs, path::Path};
use std::error;
use serde::{Serialize, Deserialize};

pub fn open_config(filepath: &Path) -> Result<Config, Box<dyn error::Error>> {
    let contents = fs::read_to_string(filepath)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn write_config(config: &Config, filepath: &Path) -> Result<(), Box<dyn error::Error>> {
    let output_string = toml::to_string(config)?;
    fs::write(filepath, output_string)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Config {
    pub branch: String,
    pub repo: String,
    pub run_instancesync: bool,
    pub server: bool,
    pub sync: bool,
    pub to_close: bool,
    pub disable_gui: bool
}

impl Config {
    pub fn default() -> Self {
        Self {
            branch: "".to_string(),
            repo: "".to_string(),
            run_instancesync: true,
            server: false,
            sync: true,
            to_close: true,
            disable_gui: false
        }
    }
}