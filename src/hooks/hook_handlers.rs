use crate::hooks::errors::ShellTypeError;
use crate::hooks::errors::HookError;
use crate::hooks::shell::Shell;
use std::{fs::OpenOptions, io::Write};
use crate::hooks::shell;

pub fn get_hook() -> Result<String, HookError> {
    let shell = shell::get_shell().map_err(|e| match e {
        ShellTypeError::ShellNotFound => HookError::GenerationFailed("Shell not found, perhaps $SHELL is not set or is invalid".to_string()),
        ShellTypeError::UnsupportedShell(_) => HookError::UnsupportedShell,
    })?;
    match shell {
        Shell::Bash => Ok(get_bash_hook()),
        Shell::Zsh => Ok(get_zsh_hook())
    }
}

pub fn set_hook(hook: &String, shell_config_path: &str) -> Result<(), HookError> {
    let existing = std::fs::read_to_string(shell_config_path).unwrap_or_default();
    if existing.contains(hook) {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(shell_config_path)
        .map_err(|e| HookError::ApplicationFailed(format!("Failed to open shell config file: {}", e)))?;

    writeln!(file, "\n# Mnemo Hook\n{}\n", hook)
        .map_err(|e| HookError::ApplicationFailed(format!("Failed to write to shell config file: {}", e)))?;

    Ok(())
}

fn get_bash_hook() -> String {
    // TODO: source the bash file in ~/.config/mnemo/hooks/bash_hook.sh
    String::from("# bash_script here")
}

fn get_zsh_hook() -> String {
    // TODO: source the bash file in ~/.config/mnemo/hooks/bash_hook.sh
    String::from("# zsh_script here")
}