use std::{fs, path::Path, process::Command, os::unix::fs::PermissionsExt, error::Error};
use crate::hooks::traits::Hookable;
use crate::hooks::{errors::HookError, shell::Shell};
use crate::config::settings::Settings;


pub struct Commands;

impl Commands {
    pub fn print_summary(config: &Settings) -> Result<(), Box<dyn Error>> {
        for dir in config.dirs.iter() {
            let dir = shellexpand::tilde(dir).to_string();
            let path = Path::new(&dir);
    
            if !path.is_dir() {
                println!("{} Directory not found: {}\n", config.emojis.warning, dir);
                continue;
            }
    
            println!("{} {}", config.emojis.folder, dir);
            let mut found = false;
    
            if let Ok(entries) = fs::read_dir(path) {
                let mut entries: Vec<_> = entries.flatten().collect();
                entries.sort_by_key(|e| e.file_name().to_string_lossy().to_lowercase());
                for entry in entries {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        let metadata = fs::metadata(&file_path)?;
                        if metadata.permissions().mode() & 0o111 != 0 {
                            let name = file_path.file_name().unwrap().to_string_lossy();
                            let output = Command::new("whatis").arg(&name.to_string()).output()?;
                            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            match stdout.is_empty() {
                                true => {
                                    println!("{: >4}  {: <19} - man description not found", config.emojis.unknown, name);
                                }
                                false => {
                                    println!("{: >5}  {}", config.emojis.executable, stdout);
                                }
                            }
                            found = true;
                        }
                    }
                }
            }
    
            if !found {
                println!("{} No executable file found in {}\n", config.emojis.info, dir);
            }
    
            println!();
        }
        Ok(())
    }

    pub fn set_shell_hook(shell_config_file: Option<&str>) -> Result<(), HookError> {
        let shell = Shell::get_shell()?;
        let config_filename = shell.set_hook(shell_config_file)?;
        println!("Shell hook applied successfully to {}. Please reopen the shell to automatically source the hook.", config_filename);
        Ok(())
    }

    pub fn hint(_command: &String) -> Result<Option<&str>, Box<dyn Error>> {
        println!("Received from hook: {_command}");
        Ok(None)
    }
}
