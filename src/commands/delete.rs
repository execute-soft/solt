use colored::*;
use log::info;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::redis_client::RedisClient;

pub async fn run(key: String, environment: Option<String>) -> Result<(), AppError> {
    info!("Deleting key: {}", key);

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

    let deleted = client.delete_key(&key).await?;

    if deleted {
        println!(
            "{}",
            format!("✓ Successfully deleted key '{}'", key)
                .green()
                .bold()
        );
    } else {
        println!("{}", format!("Key '{}' not found", key).yellow());
    }

    Ok(())
}

pub async fn delete_by_pattern(
    pattern: String,
    environment: Option<String>,
    confirm: bool,
) -> Result<(), AppError> {
    info!("Deleting keys by pattern: {}", pattern);

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

    // First, get the keys that match the pattern
    let keys = client.keys(&pattern).await?;

    if keys.is_empty() {
        println!(
            "{}",
            format!("No keys found matching pattern '{}'", pattern).yellow()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Found {} keys matching pattern '{}'", keys.len(), pattern)
            .cyan()
            .bold()
    );

    if !confirm {
        println!("{}", "Keys to be deleted:".yellow());
        for key in &keys {
            println!("  • {}", key);
        }
        println!("{}", "Use --confirm to proceed with deletion".red().bold());
        return Ok(());
    }

    // Delete the keys
    let deleted_count = client.delete_keys_by_pattern(&pattern).await?;

    println!(
        "{}",
        format!("✓ Successfully deleted {} keys", deleted_count)
            .green()
            .bold()
    );

    Ok(())
}

pub async fn flush_db(environment: Option<String>, confirm: bool) -> Result<(), AppError> {
    info!("Flushing database");

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

    if !confirm {
        println!(
            "{}",
            "WARNING: This will delete ALL keys in the current database!"
                .red()
                .bold()
        );
        println!("{}", "Use --confirm to proceed".red());
        return Ok(());
    }

    // Use FLUSHDB command
    let result: String = redis::cmd("FLUSHDB")
        .query_async(&mut client.connection)
        .await?;

    println!(
        "{}",
        format!("✓ Database flushed: {}", result).green().bold()
    );

    Ok(())
}

pub async fn flush_all(environment: Option<String>, confirm: bool) -> Result<(), AppError> {
    info!("Flushing all databases");

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

    if !confirm {
        println!(
            "{}",
            "WARNING: This will delete ALL keys in ALL databases!"
                .red()
                .bold()
        );
        println!("{}", "Use --confirm to proceed".red());
        return Ok(());
    }

    // Use FLUSHALL command
    let result: String = redis::cmd("FLUSHALL")
        .query_async(&mut client.connection)
        .await?;

    println!(
        "{}",
        format!("✓ All databases flushed: {}", result)
            .green()
            .bold()
    );

    Ok(())
}
