use std::{error::Error as StdError, vec, env, fs, path::{Path}};
use sync::*;
use toml::Value;

const CONFIG_FILE_NAME: &str = "config.toml";
fn main() -> Result<(), Box<dyn StdError>> {
    // environment variables and config variables creation

    let exec_dir = executable_dir();
    let config_path = executable_dir().join(CONFIG_FILE_NAME);
    
    check_config(&config_path)?;
    let mut config = config::open_config(config_path.as_path()).expect("failed opening config file");
    create_entries(&mut config);

    let branch = config.get("branch").unwrap().as_str().unwrap();
    let repo = config.get("repo").unwrap().as_str().unwrap();
    let sync = config.get("sync").unwrap().as_bool().unwrap();
    let run_instancesync = config.get("run_instancesync").unwrap().as_bool().unwrap();
    let server = config.get("server").unwrap().as_bool().unwrap();


    
    // program logic
    if sync {
        if git::is_repo(&exec_dir) {
            check_updates(repo, branch, &exec_dir).expect("failed checking for updates");
        } else {
            create_repo(branch, repo, &exec_dir).expect("failed creating the repo");
        }
    }

    if run_instancesync {
        run_instance_sync(server, &exec_dir).expect("failed to run instancesync");
    }

    execute_post_exit_executable(&exec_dir).expect("failed executing/checking the post_exit executable");
    Ok(())
}


fn check_config(config_path: &Path) -> Result<(), Box<dyn StdError>>  {
    if !config_path.exists() {
        fs::File::create(&config_path)?;
    }
    Ok(())
}

fn create_entries(config: &mut toml::value::Table) {
    config::check_entries(config, vec![
        ("branch", Value::String("main".to_string())),
        ("repo", Value::String("https://github.com/TeamAOF/skylore.git".to_string())),
        ("sync", Value::Boolean(true)),
        ("run_instancesync", Value::Boolean(true)),
        ("server", Value::Boolean(false))
    ]);
}

