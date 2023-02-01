#![allow(clippy::clone_double_ref)]
use std::{error::Error as StdError, env, fs, path::{Path, PathBuf}, process::exit};
use eframe::{App, egui::{CentralPanel, Layout}, emath::Align, epaint::Color32};
use sync::*;

const CONFIG_FILE_NAME: &str = "config.toml";



fn main() -> Result<(), Box<dyn StdError>> {
    
    check_config(&executable_dir().join(CONFIG_FILE_NAME))?;
    let config = config::open_config(&executable_dir().join(CONFIG_FILE_NAME)).expect("failed opening config file");
    
    if config.disable_gui {
        
        
        // launch program
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Minecraft InstanceSync",
            options,
            Box::new(|_cc| Box::new(Mic::from(config))),
        );
    } else {
        let exec_dir = executable_dir();
        if config.sync {
            if git::is_repo(&exec_dir) {
                check_updates(&config.repo, &config.branch, &exec_dir).expect("failed checking for updates");
            } else {
                create_repo(&config.branch, &config.repo, &exec_dir).expect("failed creating the repo");
            }
        }
        if config.run_instancesync {
            run_instance_sync(config.server, &exec_dir).expect("failed to run instancesync");
        }
        execute_post_exit_executable(&exec_dir).expect("failed executing/checking the post_exit executable");
    }
    

    Ok(())
}

enum Page {
    Main,
    _RepoChanges
}

struct Mic {
    config: config::Config,
    _page: Page,
    config_path: PathBuf,
    exec_path: PathBuf
}

impl Mic {
    fn from(config: config::Config) -> Self {
        Self {
            config,
            _page: Page::Main,
            config_path: executable_dir().join(CONFIG_FILE_NAME),
            exec_path: executable_dir()
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
            
            ui.checkbox(&mut self.config.server, "Is this a server?");
            if self.config.server {
                ui.colored_label(Color32::RED, "WARNING: THIS WILL MAKE THE GUI NOT LAUNCH.");
            }
            ui.checkbox(&mut self.config.run_instancesync, "Sync mods using instancesync?");
            ui.checkbox(&mut self.config.sync, "Enable syncing with github?");
            ui.checkbox(&mut self.config.to_close, "Close application after syncing?");
            
            let name_label = ui.label("Repository link: ");
            ui.text_edit_singleline(&mut self.config.repo)
                .labelled_by(name_label.id);
            let name_label = ui.label("Repository branch: ");
            ui.text_edit_singleline(&mut self.config.branch)
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
                    if self.config.sync {
                        if git::is_repo(&self.exec_path) {
                            check_updates(&self.config.repo, &self.config.branch, &self.exec_path).expect("failed checking for updates");
                        } else {
                            create_repo(&self.config.branch, &self.config.repo, &self.exec_path).expect("failed creating the repo");
                        }
                    }
                    if self.config.run_instancesync {
                        run_instance_sync(self.config.server, &self.exec_path).expect("failed to run instancesync");
                    }
                    execute_post_exit_executable(&self.exec_path).expect("failed executing/checking the post_exit executable");
                    if self.config.to_close {
                        exit(0);
                    }
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        config::write_config(&self.config, &self.config_path).expect("failed writing to config");
        
    }
    
    
    
}

fn check_config(config_path: &Path) -> Result<(), Box<dyn StdError>>  {
    if !config_path.exists() {
        fs::File::create(config_path)?;
    }
    Ok(())
}

fn check_updates(repo: &str, branch: &str, exec_dir: &Path) -> Result<(), Box<(dyn StdError)>> {
    env::set_current_dir(exec_dir)?;
    
    println!(" ");
    println!(" ");
    println!("{}", color::green("Checking for updates."));
    
    if git::current_repo().expect("not a git repo.") != repo {
        println!("{}", color::bold(&color::red("Current repository doesnt match the repository in the config file.").to_string()));
        #[cfg(target_os = "windows")] {
            execute::no_output("powershell remove-item -recurse -force .git")?;
        }
        #[cfg(not(target_os = "windows"))] {
            execute::no_output("rm -rf .git")?;
        }
        create_repo(branch, repo, exec_dir)?;
        return Ok(())
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

