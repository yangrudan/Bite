use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsConfig {
    pub tags: Vec<String>,
    pub recipes: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Recipe {
    pub name: String,
    pub image_path: String,
    pub tags: Vec<String>,
}

impl Recipe {
    pub fn new(name: String, image_path: String, tags: Vec<String>) -> Self {
        Self { name, image_path, tags }
    }
}
