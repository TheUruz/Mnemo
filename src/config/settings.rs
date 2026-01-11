use yaml_rust::YamlLoader;
use std::{collections::HashMap, env, error::Error, fs, io, path::PathBuf, str::FromStr};

use crate::{mnemo::brain::knowledge_type::KnowledgeType};
use super::emojis::{self, EMOJIS};

#[derive(Debug)]
pub struct Settings {
    pub dirs: Vec<String>,
    pub knowledge_type: KnowledgeType,
    pub confidence: HashMap<String, f32>,
    pub emojis: &'static emojis::Emojis,
}

impl Settings {
    pub fn load(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        let content = match path {
            Some(p) => fs::read_to_string(p)?,
            None => Settings::get_default_config()?
        };
        Settings::try_ensure_hooks()?;
        let docs = YamlLoader::load_from_str(&content)?;
        let doc = &docs[0];
        let dirs = doc["dirs"]
            .as_vec()
            .ok_or("'dirs' invalid or missing in config file")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        let knowledge_type = KnowledgeType::from_str(doc["knowledge_type"]
            .as_str()
            .ok_or("'knowledge_type' invalid or missing in config file")?)?;    
        let confidence_values = doc["confidence"]
            .as_hash()
            .ok_or("'confidence' invalid or not a valid mapping")?
            .iter()
            .filter_map(|(k,v)| {
                Some((
                    k.as_str()?.to_string(),
                    v.as_f64().map(|f| f as f32)?
                ))
            })
            .collect::<HashMap<String, f32>>();


        Ok(Settings { dirs, emojis: &EMOJIS, knowledge_type: knowledge_type, confidence: confidence_values })
    }

    pub fn get_default_config_path() -> Result<PathBuf, io::Error> {
        dirs::config_dir()
            .map(|dir| dir.join(clap::crate_name!()))
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine config directory",
            ))
    }

    fn get_default_config() -> Result<String, io::Error> {
        let config_path = Settings::ensure_config_dir()?;
        let file_path = Settings::ensure_config_file(&config_path)?;
        fs::read_to_string(file_path)
    }

    fn ensure_config_dir() -> Result<PathBuf, io::Error> {
        let app_config_dir = Settings::get_default_config_path()?;
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir)?;
        }
        Ok(app_config_dir)
    }
    
    fn ensure_config_file(path: &PathBuf) -> Result<PathBuf, io::Error> {
        const DEFAULT_CONFIG_FILENAME: &str = "default_config.yaml";
        let file_name = DEFAULT_CONFIG_FILENAME.strip_prefix("default_").unwrap_or(DEFAULT_CONFIG_FILENAME);
        let file_path = path.join(file_name);
        if !file_path.exists() {
            let exe_dir = env::current_exe()?;
            let config_dir = exe_dir.parent().expect("Can'read executable directory").join(DEFAULT_CONFIG_FILENAME);
            let file_content = fs::read_to_string(config_dir)?;
            fs::write(&file_path, file_content)?;
        }

        Ok(file_path)
    }

    fn try_ensure_hooks() -> Result<(), io::Error> {
        const HOOK_DIRECTORY: &str = "assets/hooks";
        let dst_dir = Settings::get_default_config_path()?.join(HOOK_DIRECTORY);
        if dst_dir.exists() {
            return Ok(());
        }

        fs::create_dir_all(&dst_dir)?;
        let hooks_dir = env::current_exe()?.parent().expect("Can't read current exe parent directory").join(HOOK_DIRECTORY);
        for entry in fs::read_dir(&hooks_dir)? {
            let entry = entry?.path();
            let file_name = entry.file_name().expect("File should have a name").to_string_lossy();
            let new_name = file_name.strip_prefix("default_").unwrap_or(&file_name);
            let dst_path = dst_dir.join(new_name);
            if !entry.is_dir() {
                fs::copy(&entry, &dst_path)?;
            }
        }
        Ok(())
    }

}
