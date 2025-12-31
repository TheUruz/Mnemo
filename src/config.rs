use yaml_rust::YamlLoader;
use std::{fs, io, error::Error, path::Path, path::PathBuf};
use crate::emojis::EMOJIS;
use crate::emojis;

const DEFAULT_CONFIG_FILENAME: &str = "config.yaml";
const DEFAULT_CONFIG_FILE: &str = "dirs:\n  - /usr/local/bin";

#[derive(Debug)]
pub struct Config {
    pub dirs: Vec<String>,
    pub emojis: &'static emojis::Emojis,
}

impl Config {
    pub fn load(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        let content;
        match path {
            Some(p) => content = fs::read_to_string(p)?,
            None => content = Config::get_default_config()?
        }

        let docs = YamlLoader::load_from_str(&content)?;
        let doc = &docs[0];

        let dirs = doc["dirs"]
            .as_vec()
            .ok_or("Campo 'dirs' mancante o non valido")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        Ok(Config { dirs, emojis: &EMOJIS })
    }

    fn get_default_config() -> Result<String, io::Error> {
        let config_path = Config::ensure_config_dir()?;
        let file_path = Config::ensure_config_file(&config_path)?;
        fs::read_to_string(file_path)
    }

    fn ensure_config_dir() -> Result<PathBuf, io::Error> {
        let app_config_dir = Config::get_default_config_path()?;
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir)?;
        }
        Ok(app_config_dir)
    }
    
    fn get_default_config_path() -> Result<PathBuf, io::Error> {
        dirs::config_dir()
            .map(|dir| dir.join(clap::crate_name!()))
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine config directory",
            ))
    }

    fn ensure_config_file(path: &PathBuf) -> Result<PathBuf, io::Error> {
        let file_path = path.join(DEFAULT_CONFIG_FILENAME);
        if !Path::new(&file_path).exists() {
            fs::write(&file_path, DEFAULT_CONFIG_FILE)?;
        }
        Ok(file_path)
    }

}
