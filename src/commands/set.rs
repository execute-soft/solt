use colored::*;
use log::info;
use std::time::Duration;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::redis_client::RedisClient;

pub async fn run(
    key: String,
    value: String,
    environment: Option<String>,
    ttl: Option<u64>,
) -> Result<(), AppError> {
    info!("Setting string value for key: {}", key);

    let config = AppConfig::load()?;
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let redis_config = config
        .get_environment(&env_name)
        .ok_or_else(|| AppError::ConfigError(format!("Environment '{}' not found", env_name)))?
        .config
        .clone();

    let mut client = RedisClient::connect(redis_config).await?;

    let ttl_duration = ttl.map(Duration::from_secs);
    client.set_string(&key, &value, ttl_duration).await?;

    println!(
        "{}",
        format!("✓ Successfully set key '{}'", key).green().bold()
    );
    if let Some(ttl) = ttl {
        println!("TTL: {} seconds", ttl.to_string().cyan());
    }

    Ok(())
}

pub async fn set_hash_field(
    key: String,
    field: String,
    value: String,
    environment: Option<String>,
) -> Result<(), AppError> {
    info!("Setting hash field: {}:{} = {}", key, field, value);

    let config = AppConfig::load()?;
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let redis_config = config
        .get_environment(&env_name)
        .ok_or_else(|| AppError::ConfigError(format!("Environment '{}' not found", env_name)))?
        .config
        .clone();

    let mut client = RedisClient::connect(redis_config).await?;

    client.set_hash_field(&key, &field, &value).await?;

    println!(
        "{}",
        format!("✓ Successfully set hash field '{}:{}'", key, field)
            .green()
            .bold()
    );

    Ok(())
}

pub async fn push_list(
    key: String,
    value: String,
    environment: Option<String>,
    left: bool,
) -> Result<(), AppError> {
    info!("Pushing to list: {} (left: {})", key, left);

    let config = AppConfig::load()?;
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let redis_config = config
        .get_environment(&env_name)
        .ok_or_else(|| AppError::ConfigError(format!("Environment '{}' not found", env_name)))?
        .config
        .clone();

    let mut client = RedisClient::connect(redis_config).await?;

    let new_length = client.push_list(&key, &value, left).await?;

    let direction = if left { "left" } else { "right" };
    println!(
        "{}",
        format!(
            "✓ Successfully pushed to {} of list '{}' (new length: {})",
            direction, key, new_length
        )
        .green()
        .bold()
    );

    Ok(())
}

pub async fn add_to_set(
    key: String,
    member: String,
    environment: Option<String>,
) -> Result<(), AppError> {
    info!("Adding member to set: {} = {}", key, member);

    let config = AppConfig::load()?;
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let redis_config = config
        .get_environment(&env_name)
        .ok_or_else(|| AppError::ConfigError(format!("Environment '{}' not found", env_name)))?
        .config
        .clone();

    let mut client = RedisClient::connect(redis_config).await?;

    let was_new = client.add_to_set(&key, &member).await?;

    if was_new {
        println!(
            "{}",
            format!(
                "✓ Successfully added new member '{}' to set '{}'",
                member, key
            )
            .green()
            .bold()
        );
    } else {
        println!(
            "{}",
            format!("Member '{}' already exists in set '{}'", member, key).yellow()
        );
    }

    Ok(())
}

pub async fn add_to_sorted_set(
    key: String,
    member: String,
    score: f64,
    environment: Option<String>,
) -> Result<(), AppError> {
    info!(
        "Adding member to sorted set: {} = {} (score: {})",
        key, member, score
    );

    let config = AppConfig::load()?;
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let redis_config = config
        .get_environment(&env_name)
        .ok_or_else(|| AppError::ConfigError(format!("Environment '{}' not found", env_name)))?
        .config
        .clone();

    let mut client = RedisClient::connect(redis_config).await?;

    let was_new = client.add_to_sorted_set(&key, &member, score).await?;

    if was_new {
        println!(
            "{}",
            format!(
                "✓ Successfully added new member '{}' to sorted set '{}' with score {}",
                member, key, score
            )
            .green()
            .bold()
        );
    } else {
        println!(
            "{}",
            format!(
                "Updated member '{}' in sorted set '{}' with score {}",
                member, key, score
            )
            .yellow()
        );
    }

    Ok(())
}
