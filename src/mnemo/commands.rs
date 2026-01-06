use std::error::Error;
use crate::config::{emojis::EMOJIS, settings::Settings};
use crate::hooks::{traits::Hookable, errors::HookError, shell::Shell};
use crate::mnemo::brain::{knowledge, knowledge_unit::KnowledgeUnit};

pub struct Commands;

impl Commands {
    pub fn print_summary(config: &Settings) -> Result<(), Box<dyn Error>> {
        let knowledge = knowledge::get_knowledge(Some(config))?;

        for (dir, units) in knowledge {
            if units.is_empty() {
                println!("{} Directory was empty", EMOJIS.folder);
                continue;
            }
            println!("{} {}", EMOJIS.folder, dir);

            for u in units {
                if u.unit_name == "N/A" || u.unit_description == "N/A" {
                    println!("{: >4}  {: <19} - {}", config.emojis.unknown, u.unit_name, u.unit_description);
                    continue;
                }
                println!("{: >5}  {} - {}", config.emojis.executable, u.unit_name, u.unit_description);
            }
        }
        println!();
        Ok(())
    }

    pub fn set_shell_hook(shell_config_file: Option<&str>) -> Result<(), HookError> {
        let shell = Shell::get_shell()?;
        let config_filename = shell.set_hook(shell_config_file)?;
        println!("Shell hook applied successfully to {}. Please reopen the shell to automatically source the hook.", config_filename);
        Ok(())
    }

    pub fn hint(command: &String) -> Result<(), Box<dyn Error>> {
        let confidence_map = knowledge::get_confidence_map(command)?;
        let filtered_map: Vec<(String, Vec<&KnowledgeUnit>)> = confidence_map.map
            .iter()
            .map(|(cmd, values)| {
                let filtered: Vec<&KnowledgeUnit> = values.iter()
                    .filter(|k| !cmd.trim().starts_with(&k.unit_name))
                    .collect();
                (cmd.clone(), filtered)
            })
            .filter(|(_, filtered_values)| !filtered_values.is_empty())
            .collect();

        if filtered_map.is_empty() {
            return Ok(());
        }

        let cmd_width = filtered_map.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
        println!("\n{} : Were you looking for one of these executable(s)?", EMOJIS.mnemo);
        for (cmd, values) in filtered_map {
            if let Some((first, rest)) = values.split_first() {
                println!("{:>4}{:<width$} {:^8} {:<3} {}", "-", cmd, "->", EMOJIS.executable, first.unit_name, width = cmd_width);
                for v in rest {
                    println!("{:>4}{:<width$} {:^8} {:<3} {}", "", "", "->", EMOJIS.executable, v.unit_name, width = cmd_width);
                }
            }
        }
        println!();
        Ok(())
    }
}
