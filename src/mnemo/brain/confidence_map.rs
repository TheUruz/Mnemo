use std::collections::HashMap;

use crate::mnemo::brain::knowledge_unit::KnowledgeUnit;

pub struct ConfidenceMap {
    pub map: HashMap<String, Vec<KnowledgeUnit>>
}

impl ConfidenceMap {
    pub fn from_map(map: HashMap<String, Vec<KnowledgeUnit>>) -> Self {
        Self {
            map: map
        }
    }
}

impl Default for ConfidenceMap {
    fn default() -> Self {
        Self { 
            map: Default::default()
        }
    }
}