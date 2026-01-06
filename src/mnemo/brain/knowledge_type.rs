use std::{error::Error, str::FromStr};

#[derive(Debug)]
pub enum KnowledgeType {
    FileSystem
}

impl FromStr for KnowledgeType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "filesystem" => Ok(KnowledgeType::FileSystem),
            _ => Err(format!("Unsupported knowledge_type: {}", s).into())
        }
    }
}