#[derive(Clone)]
pub struct KnowledgeUnit {
    pub unit_name: String,
    pub unit_description: String,
    pub unit_additional_infos: String
}

impl KnowledgeUnit {
    pub fn new(unit_name: String, unit_description: String, unit_additional_infos: String) -> Self {
        Self {
            unit_name: unit_name,
            unit_description: unit_description,
            unit_additional_infos: unit_additional_infos
        }
    }

    pub fn print(&self) -> () {
        println!()
    }
}