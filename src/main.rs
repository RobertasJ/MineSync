use std::{error::Error as StdError, vec, env, fs};
use crossterm::style::Stylize;
use sync::*;
use toml::Value;

const CONFIG_FILE_NAME: &str = "config.toml";
fn main() -> Result<(), Box<dyn StdError>> {
    // create cofig path and config
    let binding = std::env::current_exe()?;
    let executable_dir = binding.parent().unwrap().to_path_buf();
    let config_path = executable_dir.join(CONFIG_FILE_NAME);

    // check for config file and if it doesnt exist then create a new config
    if !config_path.exists() {
        fs::File::create(&config_path)?;
    }

    let mut config = config::open_config(config_path.to_str().unwrap()).expect("failed opening config file");
    config::check_entries(&mut config, vec![
        ("branch", Value::String("main".to_string())),
        ("repo", Value::String("https://github.com/RobertasJ/skylore.git".to_string())),
        ("sync", Value::Boolean(true)),
        ("run_instancesync", Value::Boolean(true)),
        ("color", Value::Boolean(false))
    ]);

    // go to the executable directory
    env::set_current_dir(&executable_dir).unwrap();

    // if repo doesnt exist, create it, if sync is disabled then skip the whole if else statement
    if config.get("sync").unwrap().as_bool().unwrap() {

        if git::is_repo(&executable_dir) {
            println!(" ");
            println!(" ");
            println!("{}", if config.get("color").unwrap().as_bool().unwrap() { "Checking for updates.".green() } else { "Checking for updates.".stylize() });
            println!(" ");
            run_command_with_stdout("git", vec!["switch", config.get("branch").unwrap().as_str().unwrap()], config.get("color").unwrap().as_bool().unwrap())?;
            run_command_with_stdout("git", vec!["pull"], config.get("color").unwrap().as_bool().unwrap())?;

        } else {
            let msg = format!("Cloning git repo into {}. Remember to not have any characters like \"(\" or \")\" \nin your path to the instance otherwise powershell will eat shit and die. \nYou can remove the tmp folder after the script is complete.", 
            executable_dir.to_str().unwrap());
            println!(" ");
            println!(" ");
            println!("{}", if config.get("color").unwrap().as_bool().unwrap() { msg.green() } else { msg.stylize() });
            println!(" ");

            // git clone --branch [config branch] [config repo] [tmp dir]
            run_command_with_stdout("git", vec!["clone", "--branch", config.get("branch").unwrap().as_str().unwrap(), 
            config.get("repo").unwrap().as_str().unwrap(), "tmp"], config.get("color").unwrap().as_bool().unwrap())?;
            // move contents of tmp to executable parent dir
            #[cfg(target_os = "windows")] {
                run_command_with_stdout("powershell", vec!["move-item", "tmp/.git", "."], config.get("color").unwrap().as_bool().unwrap())?;
                run_command_with_stdout("powershell", vec!["move-item", "tmp/*", "."], config.get("color").unwrap().as_bool().unwrap())?;
            }
            #[cfg(not(target_os = "windows"))] {
                run_command_with_stdout("mv", vec!["-rf", "tmp/.git", "."], config.get("color").unwrap().as_bool().unwrap())?;
                run_command_with_stdout("mv", vec!["-rf", "tmp/*", "."], config.get("color").unwrap().as_bool().unwrap())?;
            }
            
        }
    }
    if config.get("run_instancesync").unwrap().as_bool().unwrap() {
        // mods syncing trough instancesync
        // java -jar [parent dir of executable]/instancesync.jar
        let msg = "Launching instancesync. It will always find removed mods if there any any mods in the localMods or/and offlineMods folders. \nThey automatically get copied back over in the next step which is the intended way for having them up to date with the repo.";
        println!(" ");
        println!(" ");
        println!("{}", if config.get("color").unwrap().as_bool().unwrap() { msg.green() } else { msg.stylize() });
        println!(" ");
        run_command_with_stdout("java", vec!["-jar", 
        "instancesync.jar"], config.get("color").unwrap().as_bool().unwrap()).expect("Failed to launch isntancesync.jar. check that you have java installed.");

        // move files from offliine mods and locals mods folder to mods folder
        let msg = "Copying files from offlineMods and localMods folder to mods folder.";
        println!(" ");
        println!(" ");
        println!("{}", if config.get("color").unwrap().as_bool().unwrap() { msg.green() } else { msg.stylize() });
        println!(" ");
        #[cfg(target_os = "windows")] {
        run_command_with_stdout("powershell", vec!["copy-item", "offlineMods/*", "mods", "-ErrorAction", "Ignore"], config.get("color").unwrap().as_bool().unwrap())?;
        run_command_with_stdout("powershell", vec!["copy-item", "localMods/*", "mods", "-ErrorAction", "Ignore"], config.get("color").unwrap().as_bool().unwrap())?;
        run_command_with_stdout("powershell", vec!["copy-item", "serverMods/*", "mods", "-ErrorAction", "Ignore"], config.get("color").unwrap().as_bool().unwrap())?;

        }
        #[cfg(not(target_os = "windows"))] {
            run_command_with_stdout("cp", vec!["-rf", "offlineMods/*", "mods" ], config.get("color").unwrap().as_bool().unwrap())?;
            run_command_with_stdout("cp", vec!["-rf", "localMods/*", "mods" ], config.get("color").unwrap().as_bool().unwrap())?;
            run_command_with_stdout("cp", vec!["-rf", "serverMods/*", "mods" ], config.get("color").unwrap().as_bool().unwrap())?;
        }
    }
    let msg = "Executing post_exit file if it exists.";
    println!(" ");
    println!(" ");
    println!("{}", if config.get("color").unwrap().as_bool().unwrap() { msg.green() } else { msg.stylize() });
    println!(" ");
    
    let post_exit_file = "post_exit";

    let files = fs::read_dir(executable_dir.clone()).unwrap()
    .filter_map(|entry| {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.starts_with(post_exit_file) {
            Some(file_name)
        } else {
            None
        }
    })
    .collect::<Vec<_>>();
    let executable_dir_string = executable_dir.clone();
    let executable_dir_string = executable_dir_string.to_str().unwrap();
    let executable_dir_string = executable_dir_string.to_string() + "/";
    if files.len() > 0 {
        match files[0].split(".").last() {
            Some("sh") => run_command_with_stdout("bash", vec![&((executable_dir_string + &files[0]))], config.get("color").unwrap().as_bool().unwrap())?,
            Some("ps1") => run_command_with_stdout("powershell", vec![&(executable_dir_string + &files[0])], config.get("color").unwrap().as_bool().unwrap())?,
            Some(_) => run_command_with_stdout(&(executable_dir_string + &files[0]), vec![], config.get("color").unwrap().as_bool().unwrap())?,
            _ => ()
        };
    }
    
    
    // write to config
    config::write_config(&config, config_path.to_str().unwrap()).expect("failed to write to config");

    Ok(())
}

