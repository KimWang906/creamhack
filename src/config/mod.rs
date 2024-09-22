use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub extract_chall_file: bool,
}

// experimental-features
#[derive(Debug, Deserialize)]
pub struct Colors {}

impl Config {
    pub fn read_config() -> Result<Self, anyhow::Error> {
        let config = std::fs::read_to_string(Self::get_config_path())?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }

    pub fn read_or_new_config() -> Self {
        match Self::read_config() {
            Ok(config) => config,
            Err(_) => {
                let config = Config {
                    extract_chall_file: true,
                };

                let config_raw = toml::to_string(&config)
                    .context("Failed to serialize config")
                    .unwrap();

                let mut config_path = dirs::config_dir().unwrap();
                config_path.push("creamhack");

                if !config_path.exists() {
                    std::fs::create_dir_all(&config_path)
                        .context("Failed to create config directory")
                        .unwrap();
                }
                config_path.push("config.toml");

                std::fs::write(config_path, config_raw)
                    .context("Failed to write config")
                    .unwrap();
                config
            }
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap();
        path.push("creamhack/config.toml");
        path
    }
}
