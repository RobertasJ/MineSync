#![allow(unused)]

use directories::ProjectDirs;
use std::{io, path::PathBuf};
const CONFIG_NAME: &str = "config.json";

pub fn exec() -> io::Result<PathBuf> {
    let path = std::env::current_exe()?;
    Ok(path.parent().unwrap().to_path_buf())
}

pub fn config() -> Option<PathBuf> {
    // if running debug the use the debug config path
    if let Some(proj_dirs) = ProjectDirs::from("com", "RobertasJ", "mine-sync") {
        if cfg!(debug_assertions) {
            return Some(proj_dirs.config_dir().join("debug").join(CONFIG_NAME));
        }
        Some(proj_dirs.config_dir().join(CONFIG_NAME))
    } else {
        None
    }
}
