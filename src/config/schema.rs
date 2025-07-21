use std::collections::HashMap;

use serde::Deserialize;

pub const CONFIG_FILE: &str = ".bff.toml";

#[derive(Debug, Deserialize, Default)]
pub struct TreeConfig {
    pub alias: HashMap<String, String>,
}
