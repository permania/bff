use std::fs;

use crate::cli::error::BFFError;
use crate::config::config;

pub fn read_config() -> Result<config::TreeConfig, BFFError> {
    if !fs::exists(config::CONFIG_FILE)? {
        return Err(BFFError::NoConfig);
    }

    let toml_str = fs::read_to_string(config::CONFIG_FILE)?;
    let config: config::TreeConfig = toml::from_str(&toml_str)?;

    Ok(config)
}
