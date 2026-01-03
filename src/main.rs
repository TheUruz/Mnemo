use mnemo::{config::{settings::Settings, emojis::EMOJIS}, mnemo::{arg_parser::Args, commands::Commands}};
use clap::{CommandFactory, Parser};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
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

    if args.hint.is_some() {
        let hint_result = Commands::hint(args.hint.as_ref().expect("Command should be present"));
        match hint_result {
            Ok(r) => {
                match r {
                    Some(h) => println!("{} : Perhaps you were looking for this executable? '{}'", EMOJIS.mnemo, h),
                    _ => (),
                }
                return Ok(())
            },
            Err(e) => {
                eprintln!("Error fetching hints: {}", e);
                return Err(e);
            }
        }
    }

    Args::command().print_help()?;
    Ok(())
}
