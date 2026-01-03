use std::{fs::OpenOptions, str::FromStr, io::Write};
use crate::hooks::errors::HookError;
use super::{errors::ShellTypeError, traits::Hookable, hook_handlers};

pub enum Shell {
    Bash,
    Zsh
}

impl Shell {
    pub fn get_shell() -> Result<Shell, ShellTypeError> {
        let shell_env = std::env::var("SHELL");
        match shell_env {
            Ok(shell_name) => {
                if let Some(shell_name) = shell_name.split('/').last() {
                    return Shell::from_str(shell_name);
                }
                Err(ShellTypeError::UnsupportedShell(shell_name.to_string()))
            },
            _ => Err(ShellTypeError::ShellNotFound(String::from("Shell not found, perhaps $SHELL is not set or is invalid")))
        }
    }

    pub fn get_default_config_path(&self) -> &str {
        match self {
            Shell::Bash => "~/.bashrc",
            Shell::Zsh => "~/.zshrc",
        }
    }
}

impl Hookable for Shell {
    type Err = HookError;

    fn get_hook(&self) -> &'static str {
        match self {
            Shell::Bash => hook_handlers::get_bash_hook(),
            Shell::Zsh => hook_handlers::get_zsh_hook()
        }
    }

    fn set_hook(&self, shell_config_path: Option<&str>) -> Result<String, Self::Err> {
        let shell_config_cow = shellexpand::tilde(match shell_config_path {
            Some(p) if !p.is_empty() => p,
            _ => &self.get_default_config_path(),
        });

        let shell_config_path = shell_config_cow.into_owned();

        let hook = self.get_hook();
        let existing = std::fs::read_to_string(&shell_config_path)
            .map_err(|e| HookError::GenerationFailed(e.to_string()))?;
        
        if existing.contains(&hook) {
            return Ok(shell_config_path);
        }

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&shell_config_path)
            .map_err(|e| HookError::GenerationFailed(format!("Failed to open shell config file: {}", e)))?;

        writeln!(file, "\n# Mnemo Hook\n{}\n", hook)
            .map_err(|e| HookError::GenerationFailed(format!("Failed to write to shell config file: {}", e)))?;

        Ok(shell_config_path)
    }
}

impl FromStr for Shell {
    type Err = ShellTypeError;

    fn from_str(input: &str) -> Result<Shell, Self::Err> {
        match input.to_lowercase().as_str() {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            _ => Err(ShellTypeError::UnsupportedShell(input.to_string())),
        }
    }
}