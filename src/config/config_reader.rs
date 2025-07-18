use std::fs;

use crate::config::config;
use crate::error::BFFError;

pub fn read_config() -> Result<config::TreeConfig, BFFError> {
    if !fs::exists(config::CONFIG_NAME)? {
        return Err(BFFError::NoConfig);
    }

    let toml_str = fs::read_to_string(config::CONFIG_NAME)?;
    let config: config::TreeConfig = toml::from_str(&toml_str)?;

    Ok(config)
}
