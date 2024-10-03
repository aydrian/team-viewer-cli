use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coworker {
    pub name: String,
    pub city: String,
}

impl fmt::Display for Coworker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.city)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
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

    pub fn add_coworker(&mut self, name: String, city: String) -> Result<(), String> {
        if name.is_empty() || city.is_empty() {
            return Err("Name and city cannot be empty".to_string());
        }
        if city.chars().any(char::is_numeric) {
            return Err("City name should not contain numbers".to_string());
        }
        let coworker = Coworker { name, city };
        self.coworkers.push(coworker);
        Ok(())
    }

    pub fn remove_coworker(&mut self, name: &str) -> bool {
        let initial_len = self.coworkers.len();
        self.coworkers.retain(|c| c.name != name);
        self.coworkers.len() < initial_len
    }

    pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        dirs::home_dir()
            .ok_or_else(|| "Unable to determine home directory".into())
            .map(|path| path.join(".team_view_config.json"))
    }

    pub fn read_config() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        let content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn write_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        let config_json = serde_json::to_string_pretty(&self)?;
        fs::write(config_path, config_json).map_err(|e| e.into())
    }

    pub fn setup() -> Result<Self, Box<dyn std::error::Error>> {
        print!("Enter your city: ");
        io::stdout().flush()?;
        let mut user_city = String::new();
        io::stdin().read_line(&mut user_city)?;
        user_city = user_city.trim().to_string();

        let config = Self::new(user_city);
        config.write_config()?;

        Ok(config)
    }
}
