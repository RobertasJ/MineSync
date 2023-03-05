use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, collections::HashMap, hash::Hash};
use ts_rs::TS;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, TS, Debug)]
#[serde(default)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Config {
    pub folders: HashMap<String, Folder>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            folders: HashMap::new(),
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
    pub path: Option<PathBuf>,
    pub path_shortname: Option<String>,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            branch: None,
            repo: None,
            run_instancesync: true,
            server: false,
            path: None,
            path_shortname: None,
        }
    }
}
lazy_static! {
    static ref CONFIG_PATH: PathBuf = dirs::config();
}

/// Opens the config file. if it doesnt exist it creates it.
pub fn open_or_else_create() -> Config {
    if !CONFIG_PATH.exists() {
        create_config();
    }
    open_config()
}

/// Opens the config file. if it doesnt exist it panics. 
/// Best used with the [`open_or_else_create`] function to avoid the possibility of panicing.
fn open_config() -> Config {
    let string = fs::read_to_string(&*CONFIG_PATH).unwrap();
    serde_json::from_str(&string).unwrap()
}


/// Creates the config file and then creates the config.
/// Best used with the [`open_or_else_create`] function.

fn create_config() {
    let config = Config::default();
    let string = serde_json::to_string_pretty(&config).unwrap();

    // so fs::write can create the config file
    if let Some(parent) = CONFIG_PATH.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).unwrap();
        }
    }
    fs::write(&*CONFIG_PATH, string).unwrap();
}

/// This function takes in a config and serializes it to a json in pretty format.
pub fn save(config: &Config) {
        let string = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&*CONFIG_PATH, string).unwrap();
}
