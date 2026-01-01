use std::str::FromStr;
use crate::hooks::errors::ShellTypeError;

pub enum Shell {
    Bash,
    Zsh
}

impl Shell {
    pub fn get_default_config_path(&self) -> &str {
        match self {
            Shell::Bash => "~/.bashrc",
            Shell::Zsh => "~/.zshrc",
        }
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

pub fn get_shell() -> Result<Shell, ShellTypeError> {
    if let Ok(shell) = std::env::var("SHELL") {
        if let Some(shell_name) = shell.split('/').last() {
            return Shell::from_str(shell_name);
        }
    }
    Err(ShellTypeError::ShellNotFound)
}