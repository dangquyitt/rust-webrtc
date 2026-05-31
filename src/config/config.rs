use std::{fmt::format, fs};

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub port: u16,
    pub jwt_key: String,
    pub jwt_expires: i64,
}

impl Config {
    pub fn load_config(path: &'static str) -> Result<Config, anyhow::Error> {
        let cfg_str =
            fs::read_to_string(path).with_context(|| format!("reading config file: {}", path))?;

        let config: Config = serde_yaml::from_str(&cfg_str)
            .with_context(|| format!("parsing yaml config: {}", cfg_str))?;

        Ok(config)
    }
}
