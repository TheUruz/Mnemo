use crate::hooks::errors::ShellTypeError;
use crate::hooks::errors::HookGenerationError;
use crate::hooks::shell::Shell;
use std::str::FromStr;

pub fn generate_hook() -> Result<String, HookGenerationError> {
    let shell = get_shell().map_err(|e| match e {
        ShellTypeError::ShellNotFound => HookGenerationError::GenerationFailed("Shell not found, perhaps $SHELL is not set or is invalid".to_string()),
        ShellTypeError::UnsupportedShell(_) => HookGenerationError::UnsupportedShell,
    })?;
    match shell {
        Shell::Bash => Ok(generate_bash_hook()),
        Shell::Zsh => Ok(generate_zsh_hook())
    }
}

fn get_shell() -> Result<Shell, ShellTypeError> {
    if let Ok(shell) = std::env::var("SHELL") {
        if let Some(shell_name) = shell.split('/').last() {
            return Shell::from_str(shell_name);
        }
    }
    Err(ShellTypeError::ShellNotFound)
}

fn generate_bash_hook() -> String {
    String::from("- bash_script")
}

fn generate_zsh_hook() -> String {
    String::from("- zsh_script")
}