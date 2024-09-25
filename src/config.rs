use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coworker {
    pub first_name: String,
    pub last_name: String,
    pub city: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub user_city: String,
    pub coworkers: Vec<Coworker>,
}

impl Config {
    pub fn new(user_city: String) -> Self {
        Config {
            user_city,
            coworkers: Vec::new(),
        }
    }

    pub fn add_coworker(&mut self, first_name: String, last_name: String, city: String) {
        let coworker = Coworker {
            first_name,
            last_name,
            city,
        };
        self.coworkers.push(coworker);
    }
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    dirs::home_dir()
        .ok_or_else(|| "Unable to determine home directory".into())
        .map(|path| path.join(".team_view_config.json"))
}

pub fn read_config(config_path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn write_config(
    config_path: &PathBuf,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_json = serde_json::to_string_pretty(&config)?;
    std::fs::write(config_path, config_json).map_err(|e| e.into())
}
