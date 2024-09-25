use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub extract_chall_file: bool,
    pub keep_chall_file: bool,
    pub experimental_features: bool,
}

// experimental-features
#[derive(Debug, Deserialize)]
pub struct Colors {}

impl Config {
    // Method to ensure that `keep_chall_file` is valid based on `extract_chall_file`
    fn validate_config(&mut self) {
        if !self.extract_chall_file && !self.keep_chall_file {
            #[cfg(debug_assertions)]
            log::warn!("Both `extract_chall_file` and `keep_chall_file` are set to false. Setting `keep_chall_file` to true.");
            self.keep_chall_file = true;
        }
    }

    pub fn read_config() -> Result<Self, anyhow::Error> {
        let config = std::fs::read_to_string(Self::get_config_path())?;
        let mut config: Config = toml::from_str(&config)?;
        config.validate_config(); // Ensure `keep_chall_file` is valid
        Ok(config)
    }

    pub fn read_or_new_config() -> Self {
        match Self::read_config() {
            Ok(mut config) => {
                config.validate_config(); // Ensure `keep_chall_file` is valid
                config
            }
            Err(_) => {
                let mut config = Config {
                    extract_chall_file: true,
                    keep_chall_file: true,
                    experimental_features: false,
                };

                config.validate_config(); // Ensure `keep_chall_file` is valid

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
