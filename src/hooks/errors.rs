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
pub enum HookGenerationError {
    UnsupportedShell,
    GenerationFailed(String),
}
impl Error for HookGenerationError {}
impl Display for HookGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookGenerationError::UnsupportedShell => write!(f, "Unsupported shell for hook generation"),
            HookGenerationError::GenerationFailed(msg) => write!(f, "Hook generation failed: {}", msg),
        }
    }
}