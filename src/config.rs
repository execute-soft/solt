use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
// use std::time::Duration; // Remove unused import

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: u8,
    pub timeout: Option<u64>, // in seconds
    pub tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub config: RedisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub environments: HashMap<String, Environment>,
    pub default_environment: Option<String>,
    pub favorites: Vec<String>,
    pub history_size: usize,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "plain")]
    Plain,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut environments = HashMap::new();

        // Default development environment
        environments.insert(
            "dev".to_string(),
            Environment {
                name: "dev".to_string(),
                config: RedisConfig {
                    host: "localhost".to_string(),
                    port: 6379,
                    password: None,
                    db: 0,
                    timeout: Some(30),
                    tls: false,
                },
            },
        );

        // Default staging environment
        environments.insert(
            "staging".to_string(),
            Environment {
                name: "staging".to_string(),
                config: RedisConfig {
                    host: "localhost".to_string(),
                    port: 6379,
                    password: None,
                    db: 1,
                    timeout: Some(30),
                    tls: false,
                },
            },
        );

        // Default production environment
        environments.insert(
            "prod".to_string(),
            Environment {
                name: "prod".to_string(),
                config: RedisConfig {
                    host: "localhost".to_string(),
                    port: 6379,
                    password: None,
                    db: 2,
                    timeout: Some(30),
                    tls: false,
                },
            },
        );

        Self {
            environments,
            default_environment: Some("dev".to_string()),
            favorites: Vec::new(),
            history_size: 1000,
            output_format: OutputFormat::Table,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, anyhow::Error> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: AppConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = AppConfig::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf, anyhow::Error> {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home.join(".solt").join("config.toml"))
    }

    pub fn get_environment(&self, name: &str) -> Option<&Environment> {
        self.environments.get(name)
    }

    pub fn add_environment(&mut self, name: String, config: RedisConfig) {
        self.environments
            .insert(name.clone(), Environment { name, config });
    }

    pub fn remove_environment(&mut self, name: &str) -> bool {
        self.environments.remove(name).is_some()
    }
}

impl RedisConfig {
    pub fn to_redis_url(&self) -> String {
        let auth = if let Some(ref password) = self.password {
            format!(":{}@", password)
        } else {
            String::new()
        };

        let protocol = if self.tls { "rediss" } else { "redis" };
        format!(
            "{}://{}{}:{}/{}",
            protocol, auth, self.host, self.port, self.db
        )
    }
}
