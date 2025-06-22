use std::collections::HashMap;

#[derive(Debug)]
pub struct PodSpec {
    pub entity: String,
    pub agent_path: String,
    pub memory_path: String,
    pub hooks: HashMap<String, String>,
    pub capabilities: Vec<String>,
}
