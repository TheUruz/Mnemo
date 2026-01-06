use std::{fs::{self, DirEntry}, path::Path, process::Command, collections::HashMap, os::unix::fs::PermissionsExt, error::Error};
use regex::Regex;

use crate::config::settings::Settings;
use super::{shell_parser, confidence_map::ConfidenceMap, knowledge_type::KnowledgeType, knowledge_unit::KnowledgeUnit};

pub fn get_confidence_map(input: &str) -> Result<ConfidenceMap, Box<dyn Error>> {
    let commands = shell_parser::collect_from_program(&input)?;
    let config = Settings::load(None)?;
    let knowledge = get_knowledge(Some(&config))?;
    let mut confidence_map = ConfidenceMap::default();
    let main_match_value = config.confidence.get("main_match").unwrap_or(&0.0);
    let secondary_match_value = config.confidence.get("secondary_match").unwrap_or(&0.0);
    let confidence_threshold = config.confidence.get("confidence_threshold").unwrap_or(&100.0);

    for cmd in commands {
        let mut confidence : f32 = 0.0;
        let mut matches = Vec::new();

        let mut prefix_string = String::new();
        if let Some(prefix) = &cmd.prefix {
            for prefix_item in &prefix.0 {
                let value = &prefix_item.to_string();
                prefix_string += format!("{} ", value).as_str();
                let prefix_matches = get_knowledge_matches(&knowledge, value);
                prefix_matches.iter().for_each(|_| confidence += secondary_match_value);
                matches.extend(prefix_matches);
            }
        }

        let mut cmd_name = "";
        if let Some(word) = &cmd.word_or_name {
            cmd_name = &word.value;
            let name_matches = get_knowledge_matches(&knowledge, cmd_name);
            name_matches.iter().for_each(|_| confidence += main_match_value);
            matches.extend(name_matches);
        }

        let mut suffix_string = String::new();
        if let Some(suffix) = &cmd.suffix {
            for suffix_item in &suffix.0 {
                let value = &suffix_item.to_string();
                suffix_string += format!("{} ", value).as_str();
                let suffix_matches = get_knowledge_matches(&knowledge, value);
                suffix_matches.iter().for_each(|_| confidence += secondary_match_value);
                matches.extend(suffix_matches);
            }
        }

        if confidence < *confidence_threshold { continue };
        let command_string = format!("{} {} {}", prefix_string, cmd_name, suffix_string);
        confidence_map.map.insert(command_string, matches);
    }
    
    Ok(confidence_map)
}

pub fn get_knowledge(config: Option<&Settings>) -> Result<HashMap<String, Vec<KnowledgeUnit>>, Box<dyn Error>> {
    let config = match config {
        Some(cfg) => &cfg,
        None => &Settings::load(None)?
    };

    let mut knowledge: HashMap<String, Vec<KnowledgeUnit>> = HashMap::new();

    for dir in config.dirs.iter() {
        let mut units = Vec::new();
        let dir = shellexpand::tilde(dir).to_string();
        let path = Path::new(&dir);

        if !path.is_dir() {
            println!("{} Directory not found: {}\n", config.emojis.warning, dir);
            continue;
        }

        if let Ok(entries) = fs::read_dir(path) {
            let mut entries: Vec<DirEntry> = entries.flatten().collect();
            entries.sort_by_key(|e| e.file_name().to_string_lossy().to_lowercase());
            for entry in entries {
                let file_path = entry.path();
                if file_path.is_file() {
                    let metadata = fs::metadata(&file_path)?;
                    if metadata.permissions().mode() & 0o111 != 0 {
                        let exe_name = file_path.file_name().unwrap().to_string_lossy();
                        let knowledge_unit = match config.knowledge_type {
                            KnowledgeType::FileSystem => get_filesystem_knowledge(exe_name.as_ref())
                        };
                        units.push(knowledge_unit);
                    }
                }
            }
        }

        knowledge.insert(dir, units);
    }

    return Ok(knowledge);
}

fn get_filesystem_knowledge(exe_name: &str) -> KnowledgeUnit {
    if let Ok(output) = Command::new("whatis").arg(exe_name).output() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            let unit_name = stdout
                .split_whitespace()
                .next()
                .map(|s| s.split('(').next().unwrap().trim())
                .unwrap_or("N/A")
                .to_string(); 
            let unit_description = stdout
                .splitn(2, '-')
                .nth(1)
                .map(|s| s.trim())
                .unwrap_or("N/A")
                .to_string();
            let unit_add_info = "".to_string();
            return KnowledgeUnit::new(unit_name, unit_description, unit_add_info);
        }
    }
    KnowledgeUnit::new(exe_name.to_string(), "N/A".to_string(), "".to_string())
}

fn get_knowledge_matches(knowledge: &HashMap<String, Vec<KnowledgeUnit>>, value: &str) -> Vec<KnowledgeUnit> {
    let knowledge = knowledge.values().flatten();
    let mut matches = Vec::new();

    let Ok(re) = compile_match_regex(value) else {
        return Vec::new();
    };

    for u in knowledge {
        if re.is_match(&u.unit_name) ||
        re.is_match(&u.unit_description)||
        re.is_match(&u.unit_additional_infos){
            matches.push(u.clone())
        };
    }
    matches
}

fn compile_match_regex(value: &str) -> Result<Regex, Box<dyn Error>> {
    let pattern = format!(r"(?i)(^|[^a-zA-Z0-9])([^a-zA-Z0-9]*{})([^a-zA-Z0-9]|$)" , regex::escape(value));
    Ok(Regex::new(&pattern)?)
}
