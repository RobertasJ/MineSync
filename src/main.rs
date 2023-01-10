use std::{error::Error as StdError, vec, env, fs};
use sync::*;
use toml::Value;

const CONFIG_FILE_NAME: &str = "config.toml";
fn main() -> Result<(), Box<dyn StdError>> {
    // create cofig path and config
    let exec_dir = executable_dir();
    let config_path = executable_dir().join(CONFIG_FILE_NAME);


    // check for config file and if it doesnt exist then create a new config
    if !config_path.exists() {
        fs::File::create(&config_path)?;
    }
    // opens the config file if it exists otherwise create a new config with default parameters
    let mut config = config::open_config(config_path.as_path()).expect("failed opening config file");
    config::check_entries(&mut config, vec![
        ("branch", Value::String("main".to_string())),
        ("repo", Value::String("https://github.com/RobertasJ/skylore.git".to_string())),
        ("sync", Value::Boolean(true)),
        ("run_instancesync", Value::Boolean(true)),
        ("server", Value::Boolean(false))
    ]);

    let branch = config.get("branch").unwrap().as_str().unwrap();
    let repo = config.get("repo").unwrap().as_str().unwrap();
    let sync = config.get("sync").unwrap().as_bool().unwrap();
    let run_instancesync = config.get("run_instancesync").unwrap().as_bool().unwrap();
    let server = config.get("server").unwrap().as_bool().unwrap();


    // go to the executable directory
    env::set_current_dir(&exec_dir).unwrap();

    // if repo doesnt exist, create it, if sync is disabled then skip the whole if else statement
    if sync {

        if git::is_repo(&exec_dir) {
            println!(" ");
            println!(" ");
            println!("{}", color::green("Checking for updates."));

            
            execute::color(&format!("git pull origin {}", branch)).expect("git pull origin failed to execute");
            execute::color("git reset --hard HEAD").expect("git reset --hard failed to execute");
            execute::color(&format!("git switch {}", branch)).expect("git branch failed to execute");
        } else {
            let msg = format!("{}{}{}", 
            color::green("Cloning git repo into "),
            color::bold(&color::dark_green(exec_dir.to_str().unwrap()).to_string()),
            color::green(". Remember to not have any characters like \"(\" or \")\" \nin your path to the instance otherwise powershell will eat shit and die. \nYou can remove the tmp folder after the script is complete."));
            println!(" ");
            println!(" ");
            println!("{}", color::green(&msg));

            execute::color("git init").expect("git init failed to execute");
            execute::color(&format!("git remote add origin {}", repo)).expect("git add origin failed to execute");
            execute::color(&format!("git pull origin {}", branch)).expect("git pull origin failed to execute");
            execute::color("git reset --hard HEAD").expect("git reset --hard failed to execute");

        }

        
    }

    // runs instancesync and mod copying
    if run_instancesync {
        // mods syncing trough instancesync
        // java -jar [parent dir of executable]/instancesync.jar
        let msg = "Launching instancesync. It will always find removed mods if there any any mods in the localMods or/and offlineMods folders. \nThey automatically get copied back over in the next step which is the intended way for having them up to date with the repo.";
        println!(" ");
        println!(" ");
        println!("{}", color::green(&msg));
        execute::color("java -jar instancesync.jar").expect("Failed to launch instancesync.jar. check that you have java installed.");

        // move files from offliine mods and locals mods folder to mods folder
        let msg = "Copying files from offlineMods and localMods folder to mods folder.";
        println!(" ");
        println!(" ");
        println!("{}", color::green(&msg));
        #[cfg(target_os = "windows")] {
            execute::color("powershell copy-item offlineMods/* mods -ErrorAction Ignore")?;
            execute::color("powershell copy-item localMods/* mods -ErrorAction Ignore")?;
            if server {
                execute::color("powershell copy-item serverMods/* mods -ErrorAction Ignore")?;
            }
        }
        #[cfg(not(target_os = "windows"))] {
            execute::color("cp -rf offlineMods/* mods")?;
            execute::color("cp -rf localMods/* mods")?;
            if server {
                execute::color("cp -rf serverMods/* mods")?;
            }
        }
    }

    let msg = "Executing post_exit file if it exists.";
    println!(" ");
    println!(" ");
    println!("{}", color::green(&msg));

    let post_exit_file = "post_exit";

    let files = fs::read_dir(exec_dir.clone()).unwrap()
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
    if files.len() > 0 {
        match files[0].split(".").last() {
            Some("sh") => execute::default(format!("bash {}", &files[0]).as_str())?,
            Some("ps1") => execute::default(format!("powershell {}", &files[0]).as_str())?,
            Some(_) => execute::default(&files[0])?,
            _ => ()
        };
    }
    
    
    // write to config
    config::write_config(&config, config_path.to_str().unwrap()).expect("failed to write to config");

    Ok(())
}

