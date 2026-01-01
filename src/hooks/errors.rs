use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ShellTypeError {
    ShellNotFound,
    UnsupportedShell(String),
}
impl Error for ShellTypeError {}
impl Display for ShellTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellTypeError::ShellNotFound => write!(f, "Shell not found"),
            ShellTypeError::UnsupportedShell(shell) => write!(f, "Unsupported shell: {}", shell),
        }
    }
}

#[derive(Debug)]
pub enum HookError {
    UnsupportedShell,
    GenerationFailed(String),
    ApplicationFailed(String)
}
impl Error for HookError {}
impl Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookError::UnsupportedShell => write!(f, "Unsupported shell for hook generation"),
            HookError::GenerationFailed(msg) => write!(f, "Hook generation failed: {}", msg),
            HookError::ApplicationFailed(msg) => write!(f, "Hook application failed: {}", msg),
        }
    }
}