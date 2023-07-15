use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

type TomlDeserializeError = toml::de::Error;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub options: Options,
    pub hm5: Hm5,
    pub sniper: Sniper,
}

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub enabled: bool,
    pub console: bool,
}

/// Hitman: Absolution options.
#[derive(Deserialize, Serialize)]
pub struct Hm5 {
    pub url: String,
    pub skiplauncher: bool,
}

/// Hitman: Sniper Challenge (2012) options.
#[derive(Deserialize, Serialize)]
pub struct Sniper {
    pub url: String,
    pub skiplauncher: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    FileRead,
    FileWrite,
    Parse,
}

impl Config {
    pub fn load() -> Result<Config, ConfigError> {
        if !Path::new("./cobra.toml").exists() {
            // TODO: Update these URLs to the Peacock instance
            let config = Config {
                options: Options {
                    enabled: true,
                    console: false,
                },
                hm5: Hm5 {
                    url: String::from("http://localhost/hm5"),
                    skiplauncher: true,
                },
                sniper: Sniper {
                    url: String::from("http://localhost/sniper"),
                    skiplauncher: false,
                },
            };

            match fs::write("./cobra.toml", toml::to_string_pretty(&config).unwrap()) {
                Ok(_) => return Ok(config),
                Err(_) => return Err(ConfigError::FileWrite),
            }
        }

        match fs::read_to_string("./cobra.toml") {
            Ok(str) => {
                let config: Result<Config, TomlDeserializeError> = toml::from_str(&str);
                match config {
                    Ok(config) => Ok(config),
                    Err(_) => Err(ConfigError::Parse),
                }
            }
            Err(_) => Err(ConfigError::FileRead),
        }
    }
}
