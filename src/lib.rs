use std::path::PathBuf;

pub mod config;
pub mod git;
pub mod color;
pub mod execute;
pub mod pages;

pub fn executable_dir() -> PathBuf {
    let path = std::env::current_exe().unwrap();
    path.parent().unwrap().to_path_buf()
}
