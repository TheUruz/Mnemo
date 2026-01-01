use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use crate::{Config, hooks::{errors::HookError, errors::ShellTypeError, hook_handlers, shell}};


pub struct Commands;

impl Commands {
    pub fn print_summary(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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
        let hook_script = hook_handlers::get_hook()?;
        let shell_config_path;
        match shell_config_file {
            Some(path) if !path.is_empty() => shell_config_path = shellexpand::tilde(path).to_string(),
            _ => {
                let shell = shell::get_shell().map_err(|e| match e {
                    ShellTypeError::ShellNotFound => HookError::GenerationFailed("Shell not found, perhaps $SHELL is not set or is invalid".to_string()),
                    ShellTypeError::UnsupportedShell(_) => HookError::UnsupportedShell,
                })?;
                shell_config_path = shellexpand::tilde(shell.get_default_config_path()).to_string();
            }
        }
        hook_handlers::set_hook(&hook_script, &shell_config_path)?;
        println!("Shell hook applied successfully to {}", shell_config_path);
        Ok(())
    }

    pub fn hint() {
        todo!();
    }
}
