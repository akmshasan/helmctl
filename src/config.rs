use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub default_environment: Option<String>,
    pub default_concurrency: Option<u8>,
    pub default_timeout: Option<u32>,
    pub auto_update_repos: Option<bool>,
    pub preferred_context: Option<String>,
    pub log_level: Option<String>,
    pub repositories: Option<Vec<Repository>>,
}

#[derive(Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, String> {
        if !Path::new(path).exists() {
            return Ok(Config::default());
        }

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

        serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(path, content).map_err(|e| format!("Failed to write config file: {}", e))
    }

    pub fn default_config() -> Config {
        Config {
            default_environment: Some("development".to_string()),
            default_concurrency: Some(2),
            default_timeout: Some(300),
            auto_update_repos: Some(true),
            preferred_context: None,
            log_level: Some("info".to_string()),
            repositories: Some(vec![
                Repository {
                    name: "bitnami".to_string(),
                    url: "https://charts.bitnami.com/bitnami".to_string(),
                },
                Repository {
                    name: "stable".to_string(),
                    url: "https://charts.helm.sh/stable".to_string(),
                },
            ]),
        }
    }
}
