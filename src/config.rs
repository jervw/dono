use ansi_term::Color;
use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub github_user_token: String,
    pub native_colors: bool,
    pub fill: String,
    pub empty: String,
    pub colors: Colors,
    pub week_start_day: String,
}

#[derive(Deserialize, Serialize)]
pub struct Colors {
    pub empty: String,
    pub low: String,
    pub medium: String,
    pub high: String,
    pub max: String,
}

const CFG_DIR_NAME: &str = "dono";
const CFG_FILE_NAME: &str = "dono.toml";

impl Config {
    pub fn new() -> Result<Config, Error> {
        let xdg_dir = dirs::config_dir().expect("Could not find config directory.");
        let config_dir = xdg_dir.join(CFG_DIR_NAME);
        if !config_dir.exists() {
            fs::create_dir(&config_dir)?;
        }
        let config_file = config_dir.join(CFG_FILE_NAME);

        // read file to string from XDG_CONFIG_HOME/dono.toml
        let config_content = match fs::read_to_string(&config_file) {
            Ok(content) => content,
            Err(_) => {
                // create config file if it doesn't exist
                let config = Config::default();
                let config_str = toml::to_string(&config).unwrap();
                fs::write(&config_file, config_str)?;

                let url = "https://github.com/settings/tokens";
                println!("Config file created at: {}", config_file.display());
                println!("Please edit the file and add your GitHub personal access token.");
                println!(
                    "Generate a personal access token at ({}).",
                    Color::White.dimmed().underline().paint(url)
                );
                std::process::exit(0);
            }
        };

        // parse config file
        let config: Config = match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(_) => {
                return Err(anyhow!(
                    "Config file is invalid, please check your config file."
                ));
            }
        };

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), Error> {
        // validate github user token
        if self.github_user_token.is_empty() {
            return Err(anyhow!(
                "GitHub user token field in configuration file is empty."
            ));
        }

        // validate colors that they are valid hex color codes
        let colors = vec![
            &self.colors.empty,
            &self.colors.low,
            &self.colors.medium,
            &self.colors.high,
            &self.colors.max,
        ];

        for color in colors {
            if !color.starts_with('#') {
                return Err(anyhow!("color {color} is not a valid hex color code",));
            }
            if color.len() != 7 {
                return Err(anyhow!("color {color} is not a valid hex color code",));
            }
            if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(anyhow!("color {color} is not a valid hex color code"));
            }
        }

        Ok(())
    }

    pub fn rewrite_config_file(&self) -> Result<(), Error> {
        let xdg_dir = dirs::config_dir().expect("Could not find config directory.");
        let config_dir = xdg_dir.join(CFG_DIR_NAME);
        let config_file = config_dir.join(CFG_FILE_NAME);

        let config_str = toml::to_string(&self).unwrap();
        fs::write(&config_file, config_str)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            github_user_token: String::from(""),
            native_colors: false,
            fill: "■".to_string(),
            empty: "■".to_string(),
            colors: Colors {
                empty: String::from("#161b22"),
                low: String::from("#0e4429"),
                medium: String::from("#006d32"),
                high: String::from("#26a641"),
                max: String::from("#39d353"),
            },
            week_start_day: String::from("Sunday"),
        }
    }
}
