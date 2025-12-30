use mnemo::Config;
use mnemo::arg_parser::Args;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::load(&args.config_file).unwrap_or_else(|err| {
        eprintln!("Error loading config file {}: {}", &args.config_file, err);
        std::process::exit(1);
    });

    if args.summary {
        print_summary(&config).unwrap_or_else(|err| {
            eprintln!("Error printing summary: {}", err);
            std::process::exit(1);
        });
        return Ok(());
    }

    Ok(())
}

fn print_summary(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    for dir in config.dirs.iter() {
        let dir = shellexpand::tilde(dir).to_string();
        let path = Path::new(&dir);

        if !path.is_dir() {
            println!("{} Directory non trovata: {}\n", config.emojis.warning, dir);
            continue;
        }

        println!("{} {}", config.emojis.folder, dir);
        let mut found = false;

        if let Ok(entries) = fs::read_dir(path) {
            let mut entries: Vec<_> = entries.flatten().collect();
            entries.sort_by_key(|e| e.file_name().to_string_lossy().to_lowercase());
            for entry in entries {
                let file_path = entry.path();
                if file_path.is_file() {
                    let metadata = fs::metadata(&file_path)?;
                    if metadata.permissions().mode() & 0o111 != 0 {
                        let name = file_path.file_name().unwrap().to_string_lossy();
                        let output = Command::new("whatis").arg(&name.to_string()).output()?;
                        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        match stdout.is_empty() {
                            true => {
                                println!("{: >4}  {: <19} - man description not found", config.emojis.unknown, name);
                            }
                            false => {
                                println!("{: >5}  {}", config.emojis.executable, stdout);
                            }
                        }
                        found = true;
                    }
                }
            }
        }

        if !found {
            println!("{} No executable file found in {}\n", config.emojis.info, dir);
        }

        println!();
    }
    Ok(())
}
