use crate::app::AppResult;
use std::fs;
use std::path::Path;
use toml::Value;

pub fn read_config(config_path: &Path) -> AppResult<Vec<String>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Value = toml::from_str(&config_content)?;

    let profiles = config["aws_profiles"]
        .as_array()
        .ok_or("aws_profiles should be an array")?
        .iter()
        .filter_map(|v| v.as_str())
        .map(String::from)
        .collect();

    Ok(profiles)
}
