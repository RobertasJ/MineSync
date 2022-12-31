extern crate rlib;

use std::{error::Error as StdError, vec};
use rlib::*;

const CONFIG_FILE_NAME: &str = "config.toml";
fn main() -> Result<(), Box<dyn StdError>> {
    // create cofig path and config
    let binding = std::env::current_exe()?;
    let script_dir = binding.parent().unwrap().to_path_buf();
    let config_path = script_dir.join(CONFIG_FILE_NAME);
    let config = rlib::config::open_config(config_path.to_str().unwrap());


    // if statement if executableis run for the first time
    if git::is_repo(&script_dir.to_str().unwrap()) == false {

        // git clone --branch [config branch] [config repo] [tmp dir]
        run_command_with_stdout("git", vec!["clone", "--branch", config.get("branch").unwrap().as_str().unwrap(),
            config.get("repo").unwrap().as_str().unwrap(), 
            script_dir.join("tmp").to_str().unwrap()])?;
        // move contents of tmp to executable parent dir
        run_command_with_stdout("powershell", vec!["move-item",  
            script_dir.join("tmp").join("*").to_str().unwrap(),
            script_dir.to_str().unwrap()])?;
        // delete temporary directory
        run_command_with_stdout("powershell", vec!["remove-item", script_dir.join("tmp/").to_str().unwrap()])?;
        // java -jar [parent dir of executable]/instancesync.jar
        run_command_with_stdout("java", vec!["-jar", 
        script_dir.join("instancesync.jar").to_str().unwrap()])?;

        // use fs extra to move files from offliine mods and locals mods folder to mods folder
        let mut dir1 = script_dir.join("localMods");
        let mut dir2 = script_dir.join("offlineMods");
        dir2.set_file_name("*");
        dir1.set_file_name("*");

        let items = vec![dir1.to_str().unwrap(), dir2.to_str().unwrap()];
        fs_extra::copy_items(&items, "mods", &fs_extra::dir::CopyOptions::new())?;
    }


    
    // git pull
    
    // write to config
    // rlib::config::write_config(&config_path, &config);

    Ok(())
}

