use std::fs;

use crate::cli::error::BFFError;
use crate::config::schema;

pub fn read_config() -> Result<schema::TreeConfig, BFFError> {
    if !fs::exists(schema::CONFIG_FILE)? {
        return Err(BFFError::NoConfig);
    }

    let toml_str = fs::read_to_string(schema::CONFIG_FILE)?;
    let config: schema::TreeConfig = toml::from_str(&toml_str)?;

    Ok(config)
}
