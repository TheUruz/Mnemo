use yaml_rust::YamlLoader;
use std::env;
use std::{fs, io, error::Error, path::Path, path::PathBuf};
use crate::emojis::EMOJIS;
use crate::emojis;

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
        Config::try_ensure_hooks()?;
        let docs = YamlLoader::load_from_str(&content)?;
        let doc = &docs[0];
        let dirs = doc["dirs"]
            .as_vec()
            .ok_or("'dirs' invalid or missing in config file")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        Ok(Config { dirs, emojis: &EMOJIS })
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
    
    fn ensure_config_file(path: &PathBuf) -> Result<PathBuf, io::Error> {
        const DEFAULT_CONFIG_FILENAME: &str = "default_config.yaml";
        let file_name = DEFAULT_CONFIG_FILENAME.strip_prefix("default_").unwrap_or(DEFAULT_CONFIG_FILENAME);
        let file_path = path.join(file_name);
        if !Path::new(&file_path).exists() {
            let file_content = fs::read_to_string(DEFAULT_CONFIG_FILENAME)?;
            fs::write(&file_path, file_content)?;
        }
        Ok(file_path)
    }

    fn try_ensure_hooks() -> Result<(), io::Error> {
        const HOOK_DIRECTORY: &str = "assets/hooks";
        let dst_dir = Config::get_default_config_path()?.join(HOOK_DIRECTORY);
        if dst_dir.exists() {
            return Ok(());
        }
        fs::create_dir_all(&dst_dir)?;
        let hooks_dir = env::current_exe()?.parent().expect("Can't read parent exe parent directory").join(HOOK_DIRECTORY);
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
