use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub github_user_token: String,
    week_starts_sunday: bool,
}

const CFG_FILE_NAME: &str = "dono.toml";

impl Config {
    pub fn new() -> Result<Config, Error> {
        let xdg_dir = dirs::config_dir().expect("config dir not found");
        let config_dir = xdg_dir.join("dono");
        if !config_dir.exists() {
            fs::create_dir(&config_dir)?;
        }
        let config_file = config_dir.join(CFG_FILE_NAME);

        // read file to string from XDG_CONFIG_HOME/dono.toml
        let config_content = match fs::read_to_string(&config_file) {
            Ok(content) => content,
            Err(_) => {
                // create config file if not exists
                let config = Config {
                    github_user_token: String::from(""),
                    week_starts_sunday: true,
                };
                let config_str = toml::to_string(&config).unwrap();
                fs::write(&config_file, config_str)?;
                return Err(anyhow!(
                    "generated config file, please add your GitHub user token"
                ));
            }
        };

        // parse config file
        let config: Config = match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(_) => {
                return Err(anyhow!(
                    "config file is invalid, please check your config file"
                ));
            }
        };

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.github_user_token.is_empty() {
            return Err(anyhow!("github user token is empty"));
        }

        Ok(())
    }
}
