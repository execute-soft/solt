use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}
