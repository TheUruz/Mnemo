use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ShellTypeError {
    ShellNotFound(String),
    UnsupportedShell(String),
}
impl Error for ShellTypeError {}
impl Display for ShellTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellTypeError::ShellNotFound(msg) => write!(f, "Shell {} not found", msg),
            ShellTypeError::UnsupportedShell(shell_name) => write!(f, "Unsupported shell: {}", shell_name),
        }
    }
}

#[derive(Debug)]
pub enum HookError {
    UnsupportedShell(String),
    GenerationFailed(String)
}
impl Error for HookError {}
impl Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookError::UnsupportedShell(msg) => write!(f, "Hook failed to apply: {}", msg),
            HookError::GenerationFailed(msg) => write!(f, "Hook failed to apply: {}", msg)
        }
    }
}

impl From<ShellTypeError> for HookError {
    fn from(value: ShellTypeError) -> Self {
        match value {
            ShellTypeError::ShellNotFound(msg) => HookError::GenerationFailed(msg),
            ShellTypeError::UnsupportedShell(msg) => HookError::UnsupportedShell(msg)
        }
    }
}