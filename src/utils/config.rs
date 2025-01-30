use std::path::Path;

use config::{builder::DefaultState, ConfigBuilder, ConfigError, ValueKind};
use serde_json::{Map, Value};

use crate::models::config::RunConfiguration;

use super::io::{create_file_if_not_exist, is_path_exist};


pub fn ensure_config_file(file: &Path) {
    if !is_path_exist(file) {
        let _ = create_file_if_not_exist(file);
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

pub fn is_valid_field(key: &str) -> bool {
    let self_json: Value = serde_json::to_value(&_SELF).unwrap();
    let map: &Map<String, Value> = self_json.as_object().unwrap();
    return map.contains_key(key);
}

static _SELF: RunConfiguration = RunConfiguration {
    format: None,
    pretty_json: None,
};

pub fn merge_toml_tables(table1: &toml::Table, table2: &toml::Table) -> toml::Table {
    let mut merged_table = table1.clone();

    for (key, value) in table2 {
        if let Some(existing_value) = merged_table.get_mut(key) {
            match (&mut *existing_value, value) {
                (toml::Value::Table(existing_table), toml::Value::Table(new_table)) => {
                    let merged_subtable = merge_toml_tables(existing_table, new_table);
                    *existing_value = toml::Value::Table(merged_subtable);
                }
                _ => {
                    // 如果值不是表，直接覆盖
                    *existing_value = value.clone();
                }
            }
        } else {
            merged_table.insert(key.clone(), value.clone());
        }
    }

    merged_table
}
