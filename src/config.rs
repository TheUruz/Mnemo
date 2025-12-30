use yaml_rust::YamlLoader;
use std::fs;
use std::error::Error;

use crate::emojis::EMOJIS;
use crate::emojis;


#[derive(Debug)]
pub struct Config {
    pub dirs: Vec<String>,
    pub emojis: &'static emojis::Emojis,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
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
}
