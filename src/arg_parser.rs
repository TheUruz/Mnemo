use clap::{Parser, crate_version};

#[derive(Parser, Debug)]
#[command(
    version = crate_version!(),
    about = "Don't remind whatever you installed on your system? Mnemo is here to help you!", 
    long_about = "longer description of the program")]
pub struct Args {
    /// Path to the configuration file
    #[arg(short, long, value_name = "CONFIG_FILE_PATH")]
    pub config_file: Option<String>,

    /// Custom executables summary
    /// from the config directories
    #[arg(short, long)]
    pub summary: bool,
}