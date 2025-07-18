use std::collections::HashMap;

use serde::Deserialize;

pub const CONFIG_FILE: &str = ".bff.toml";

#[derive(Debug, Deserialize)]
pub struct TreeConfig {
    pub alias: HashMap<String, String>,
}

impl Default for TreeConfig {
    fn default() -> Self {
        TreeConfig {
            alias: std::collections::HashMap::new(),
        }
    }
}
