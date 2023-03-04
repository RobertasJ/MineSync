use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(default)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Config {
    pub folders: Vec<Folder>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            folders: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, TS, Debug, Clone)]
#[serde(default)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Folder {
    pub branch: Option<String>,
    pub repo: Option<String>,
    pub run_instancesync: bool,
    pub server: bool,
    pub sync: bool,
    pub name: String,
    pub path: Option<PathBuf>,
    pub path_shortname: Option<String>,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            name: format!("Folder{}", rand::random::<u16>()),
            branch: None,
            repo: None,
            run_instancesync: true,
            server: false,
            sync: true,
            path: None,
            path_shortname: None,
        }
    }
}

pub fn open_or_else_create() -> Config {
    if let Some(config_path) = dirs::config() {
        // config_path normally is somewhere in C:\Users\{username}\AppData\Roaming
        if !config_path.exists() {
            create_config(&config_path);
        }
        open_config(config_path)
    } else {
        // honestly i dont know how this could happen and how to fix it
        panic!("Could not find config path, contact the developer please. i honestly have no idea how this can happen.");
    }
}

fn open_config(config_path: PathBuf) -> Config {
    let string = fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_json::from_str(&string).unwrap();
    config
}

fn create_config(config_path: &PathBuf) {
    let config = Config::default();
    let string = serde_json::to_string_pretty(&config).unwrap();
    println!("Created config file at: {:?}", config_path);

    // so fs::write can create the config file
    if let Some(parent) = config_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
        }
    }
    fs::write(config_path, string).unwrap();
}

pub fn save(config: &Config) {
    if let Some(config_path) = dirs::config() {
        let string = serde_json::to_string_pretty(&config).unwrap();
        fs::write(config_path, string).unwrap();
    } else {
        // honestly i dont know how this could happen and how to fix it
        panic!("Could not find config path, contact the developer please. i honestly have no idea how this can happen.");
    }
}
