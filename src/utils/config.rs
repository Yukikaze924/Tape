use config::{builder::DefaultState, ConfigBuilder, ConfigError, ValueKind};

use super::io::{create_file_if_not_exist, is_path_exist};


pub fn ensure_config_file(filepath: &str) {
    if !is_path_exist(filepath) {
        create_file_if_not_exist(filepath);
    }
    else {
        return;
    }
}

pub fn set_defaults<V>(mut builder: config::ConfigBuilder<config::builder::DefaultState>, defaults: Vec<(&str, V)>) -> Result<ConfigBuilder<DefaultState>, ConfigError>
where
    ValueKind : From<V>
{
    for (key, value) in defaults {
        let config_value: ValueKind = value.into();
        builder = builder.set_default(key, config_value)?;
    }
    return Ok(builder);
}