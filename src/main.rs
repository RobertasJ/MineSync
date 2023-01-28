#![allow(clippy::clone_double_ref)]
use std::{error::Error as StdError, vec, env, fs, path::{Path, PathBuf}, process::exit};
use eframe::{App, egui::{CentralPanel, Layout}, emath::Align, epaint::Color32};
use sync::*;
use toml::Value;

const CONFIG_FILE_NAME: &str = "config.toml";


fn main() -> Result<(), Box<dyn StdError>> {
    let exec_dir = executable_dir();
    let config_path = executable_dir().join(CONFIG_FILE_NAME);
    check_config(&config_path).unwrap();
    let mut config = config::open_config(config_path.as_path()).expect("failed opening config file");
    create_entries(&mut config);
    config::write_config(&config, config_path.to_str().unwrap()).expect("failed to write to config");
    let branch = config.get("branch").unwrap().as_str().unwrap().to_string();
    let repo = config.get("repo").unwrap().as_str().unwrap().to_string();
    let sync = config.get("sync").unwrap().as_bool().unwrap();
    let run_instancesync = config.get("run_instancesync").unwrap().as_bool().unwrap();
    let server = config.get("server").unwrap().as_bool().unwrap();

    if !(config.get("server").unwrap().as_bool().unwrap()) {


        // launch program
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Minecraft InstanceSync",
            options,
            Box::new(|_cc| Box::new(Mic::new())),
        );
    } else {
        if sync {
            if git::is_repo(&exec_dir) {
                check_updates(&repo, &branch, &exec_dir).expect("failed checking for updates");
            } else {
                create_repo(&branch, &repo, &exec_dir).expect("failed creating the repo");
            }
        }
        if run_instancesync {
            run_instance_sync(server, &exec_dir).expect("failed to run instancesync");
        }
        execute_post_exit_executable(&exec_dir).expect("failed executing/checking the post_exit executable");
    }
    

    Ok(())
}

struct Mic {
    branch: String,
    repo: String,
    run_instancesync: bool,
    server: bool,
    sync: bool,
    to_close: bool,
    exec_dir: PathBuf
}

impl Mic {
    fn new() -> Self {
        let exec_dir = executable_dir();
        let config_path = executable_dir().join(CONFIG_FILE_NAME);
        check_config(&config_path).unwrap();
        let mut config = config::open_config(config_path.as_path()).expect("failed opening config file");
        create_entries(&mut config);
        config::write_config(&config, config_path.to_str().unwrap()).expect("failed to write to config");
        let branch = config.get("branch").unwrap().as_str().unwrap().to_string();
        let repo = config.get("repo").unwrap().as_str().unwrap().to_string();
        let sync = config.get("sync").unwrap().as_bool().unwrap();
        let run_instancesync = config.get("run_instancesync").unwrap().as_bool().unwrap();
        let to_close = config.get("to_close").unwrap().as_bool().unwrap();
        let server = config.get("server").unwrap().as_bool().unwrap();
        
        Self {
            branch,
            repo,
            sync,
            run_instancesync,
            server,
            to_close,
            exec_dir
        }
    }
}

impl App for Mic {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {

            // Title
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("Minecraft InstanceSync");
            });

            ui.checkbox(&mut self.server, "Is this a server?");
            if self.server {
                ui.colored_label(Color32::RED, "WARNING: THIS WILL MAKE THE GUI NOT LAUNCH.");
            }
            ui.checkbox(&mut self.run_instancesync, "Sync mods using instancesync?");
            ui.checkbox(&mut self.sync, "Enable syncing with github?");
            ui.checkbox(&mut self.to_close, "Close application after syncing?");

            let name_label = ui.label("Repository link: ");
            ui.text_edit_singleline(&mut self.repo)
                .labelled_by(name_label.id);
            let name_label = ui.label("Repository branch: ");
            ui.text_edit_singleline(&mut self.branch)
                .labelled_by(name_label.id);

            // bottom part with info and sync button
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        if ui.button("GitHub").clicked() {
                            opener::open("https://github.com/RobertasJ/minecraft-instance-sync").expect("failed opening link.");
                        }
                        
                    });
        
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label("Contact me on discord: _Robertas#0001");
                    });
        
                });
                if ui.button("Sync").on_hover_text("This can take a while").clicked() {
                    if self.sync {
                        if git::is_repo(&self.exec_dir) {
                            check_updates(&self.repo, &self.branch, &self.exec_dir).expect("failed checking for updates");
                        } else {
                            create_repo(&self.branch, &self.repo, &self.exec_dir).expect("failed creating the repo");
                        }
                    }
                    if self.run_instancesync {
                        run_instance_sync(self.server, &self.exec_dir).expect("failed to run instancesync");
                    }
                    execute_post_exit_executable(&self.exec_dir).expect("failed executing/checking the post_exit executable");
                    if self.to_close {
                        exit(0);
                    }
                }
            });
        });
    }


    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let config_path = executable_dir().join(CONFIG_FILE_NAME);
        let mut config = config::open_config(config_path.as_path()).expect("failed opening config file");

        config.insert("repo".to_string(), Value::String(self.repo.clone()));
        config.insert("branch".to_string(), Value::String(self.branch.clone()));
        config.insert("sync".to_string(), Value::Boolean(self.sync));
        config.insert("run_instancesync".to_string(), Value::Boolean(self.run_instancesync));
        config.insert("to_close".to_string(), Value::Boolean(self.to_close));
        config.insert("server".to_string(), Value::Boolean(self.server));


        config::write_config(&config, config_path.to_str().unwrap()).expect("failed to write to config");

    }
    
}

fn check_config(config_path: &Path) -> Result<(), Box<dyn StdError>>  {
    if !config_path.exists() {
        fs::File::create(config_path)?;
    }
    Ok(())
}

fn create_entries(config: &mut toml::value::Table) {
    config::check_entries(config, vec![
        ("branch", Value::String("".to_string())),
        ("repo", Value::String("".to_string())),
        ("sync", Value::Boolean(true)),
        ("run_instancesync", Value::Boolean(true)),
        ("server", Value::Boolean(false)),
        ("to_close", Value::Boolean(true))
    ]);
}

fn check_updates(repo: &str, branch: &str, exec_dir: &Path) -> Result<(), Box<(dyn StdError)>> {
    env::set_current_dir(exec_dir)?;

    println!(" ");
    println!(" ");
    println!("{}", color::green("Checking for updates."));

    if git::current_repo()?.unwrap() != repo {
        println!("{}", color::bold(&color::red("Current repository doesnt match the repository in the config file.").to_string()));
        #[cfg(target_os = "windows")] {
            execute::no_output("powershell remove-item -recurse -force .git")?;
        }
        #[cfg(not(target_os = "windows"))] {
            execute::no_output("rm -rf .git")?;
        }
        create_repo(branch, repo, exec_dir)?;
    }
    
    execute::no_output("git add *").expect("failed to run git");
    execute::no_output("git commit -a -m \"tmp commit\"").expect("failed to run git");
    execute::color(&format!("git fetch origin {}", branch)).expect("failed to run git");
    execute::color(&format!(" git merge -s recursive -X theirs origin/{}", branch)).expect("failed to run git");
    execute::no_output(&format!("git switch {}", branch)).expect("failed to run git");
    Ok(())
}

fn create_repo(branch: &str, repo: &str, exec_dir: &Path) -> Result<(), Box<dyn StdError>> {
    env::set_current_dir(exec_dir)?;

    let msg = format!("{}{}{}", 
    color::green("Cloning git repo into "),
    color::bold(&color::dark_green(exec_dir.to_str().unwrap()).to_string()),
    color::green(". Remember to not have any characters like \"(\" or \")\" \nin your path to the instance otherwise powershell will eat shit and die. \nYou can remove the tmp folder after the script is complete."));
    println!(" ");
    println!(" ");
    println!("{}", color::green(&msg));

    execute::no_output("git init").expect("failed to run git");
    execute::no_output(&format!("git remote add origin {}", repo)).expect("failed to run git");
    execute::no_output("git add *").expect("failed to run git");
    execute::no_output("git commit -a -m \"tmp commit\"").expect("failed to run git");
    execute::color(&format!("git fetch origin {}", branch)).expect("failed to run git");
    execute::color(&format!(" git merge -s recursive -X theirs origin/{}", branch)).expect("failed to run git");
    execute::no_output(&format!("git switch {}", branch)).expect("failed to run git");

    Ok(())
}

fn run_instance_sync(server: bool, exec_dir: &Path) -> Result<(), Box<dyn StdError>> {
    env::set_current_dir(exec_dir)?;

    let msg = "Launching instancesync. It will always find removed mods if there any any mods in the localMods or/and offlineMods folders. \nThey automatically get copied back over in the next step which is the intended way for having them up to date with the repo.";
    println!(" ");
    println!(" ");
    println!("{}", color::green(msg));
    execute::color("java -jar instancesync.jar").expect("Failed to launch instancesync.jar. check that you have java installed.");

    let msg = "Copying files from offlineMods and localMods folder to mods folder.";
    println!(" ");
    println!(" ");
    println!("{}", color::green(msg));
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
    
    Ok(())
}

fn execute_post_exit_executable(exec_dir: &Path) -> Result<(), Box<dyn StdError>> {
    let msg = "Executing post_exit file if it exists.";
    println!(" ");
    println!(" ");
    println!("{}", color::green(msg));

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
    if !files.is_empty() {
        match files[0].split('.').last() {
            Some("sh") => execute::default(format!("bash {}", &files[0]).as_str())?,
            Some("ps1") => execute::default(format!("powershell {}", &files[0]).as_str())?,
            Some(_) => execute::default(&files[0])?,
            _ => ()
        };
    }
    Ok(())
}

