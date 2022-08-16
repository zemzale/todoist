use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

pub fn setup_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().expect("failed to get home directory");
    let config_dir = Path::new(&home).join(".config/api");
    if !config_dir.is_dir() {
        fs::create_dir(config_dir.to_owned()).expect("failed to create config directory");
    }
    Ok(Figment::new()
        .merge(Yaml::file(config_dir.join("config.yaml")))
        .extract()?)
}
