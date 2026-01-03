use clap::{Parser, crate_version};

#[derive(Parser, Debug)]
#[command(
    version = crate_version!(),
    about = "Don't remind whatever you installed on your system? Mnemo is here to help you!", 
    long_about = r#"Mnemo is a command-line tool that provides a summary of custom executables installed on your system.
The executalbes are fetched from the directories listed in config files.
It also provides a shell hooks to suggest executables in these directories based on word match"#,
    )]
pub struct Args {
    /// Path to the configuration file.
    /// This command must be used with either summary or hint command to produce a meaningful result.
    #[arg(short, long, value_name = "CONFIG_FILE_PATH")]
    pub config_file: Option<String>,

    /// Custom executables summary.
    /// Executables are read from the directories in config file.
    #[arg(short, long)]
    pub summary: bool,

    /// Set a shell hook for the default shell.
    /// If a path to a shell config file is provided, that file will be used, otherwise it will be searched in userś home directory. (e.g. ~/.bashrc, ~/.zshrc)
    #[arg(long, value_name = "SHELL_CONFIG_FILE", num_args = 0..=1, default_missing_value = Some(""), require_equals = true)]
    pub set_shell_hook: Option<String>,

    /// Get a Mnemo hint based on a provided command.
    /// Directories specified in the configuration file will be scanned for matches.
    #[arg(short='H', long, value_name = "$COMMAND")]
    pub hint: Option<String>
}