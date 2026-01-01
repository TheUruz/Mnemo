use std::str::FromStr;
use crate::hooks::errors::ShellTypeError;

pub enum Shell {
    Bash,
    Zsh
}

impl FromStr for Shell {
    type Err = ShellTypeError;

    fn from_str(input: &str) -> Result<Shell, Self::Err> {
        match input {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            _ => Err(ShellTypeError::UnsupportedShell(input.to_string())),
        }
    }
}