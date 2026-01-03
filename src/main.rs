use mnemo::{config::settings::Settings, execution::{arg_parser::Args, commands::Commands}};
use clap::{CommandFactory, Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Settings::load(args.config_file).unwrap_or_else(|err| {
        eprintln!("Error loading config file: {}", err);
        std::process::exit(1);
    });

    if args.summary {
        Commands::print_summary(&config).unwrap_or_else(|err| {
            eprintln!("Error printing summary: {}", err);
            std::process::exit(1);
        });
        return Ok(());
    }

    if args.set_shell_hook.is_some() {
        Commands::set_shell_hook(args.set_shell_hook.as_deref()).unwrap_or_else(|err| {
            eprintln!("Error setting shell hook: {}", err);
            std::process::exit(1);
        });
        return Ok(());
    }

    Args::command().print_help()?;
    Ok(())
}
