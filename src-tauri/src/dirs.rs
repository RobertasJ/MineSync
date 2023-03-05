#![allow(unused)]

use directories::ProjectDirs;
use std::{io, path::PathBuf};
const CONFIG_NAME: &str = "config.json";

/// shorthand for getting the dir from the tauri api
pub fn exec() -> PathBuf {
    tauri::api::path::executable_dir().expect("idk when this can be None. hint, its the dirs module where this is erroring")
}

/// shorthand for getting the dir from the tauri api
pub fn config() -> PathBuf {
    tauri::api::path::config_dir().expect("idk when this can be None. hint, its the dirs module where this is erroring").join(CONFIG_NAME)
}
