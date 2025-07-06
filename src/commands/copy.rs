use colored::*;
use log::info;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::redis_client::RedisClient;

pub async fn run() -> Result<(), AppError> {
    info!("Copy command invoked");
    let config = AppConfig::load()?;
    let env = config
        .default_environment
        .clone()
        .unwrap_or_else(|| "dev".to_string());
    let redis_config = config
        .get_environment(&env)
        .ok_or_else(|| AppError::ConfigError(format!("Environment '{}' not found", env)))?
        .config
        .clone();
    let mut client = RedisClient::connect(redis_config).await?;

    println!("{}", "Copy keys between databases or environments".cyan());
    println!("{}", "1. Copy within same environment".yellow());
    println!("{}", "2. Copy between different environments".yellow());
    print!("Enter choice (1/2): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    match choice {
        "1" => {
            copy_within_environment(&mut client).await?;
        }
        "2" => {
            copy_between_environments(&config).await?;
        }
        _ => {
            println!("{}", "Invalid choice. Aborting.".red());
        }
    }
    Ok(())
}

async fn copy_within_environment(client: &mut RedisClient) -> Result<(), AppError> {
    println!("{}", "Copying within same environment".cyan());
    print!("Enter source key pattern (e.g., 'user:*'): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let source_pattern = input.trim();

    print!("Enter destination prefix (e.g., 'backup:'): ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let dest_prefix = input2.trim();

    println!(
        "{}",
        format!(
            "Copying keys matching '{}' to '{}*'",
            source_pattern, dest_prefix
        )
        .cyan()
    );
    let keys = client.keys(source_pattern).await?;
    if keys.is_empty() {
        println!("{}", "No keys found matching the pattern.".yellow());
        return Ok(());
    }

    println!("{}", format!("Found {} keys to copy", keys.len()).green());
    let mut copied = 0;
    for key in keys {
        let dest_key = format!("{}{}", dest_prefix, key);
        // Get value from source key
        if let Some(value) = client.get_string(&key).await? {
            // Set value in destination key
            match client.set_string(&dest_key, &value, None).await {
                Ok(_) => {
                    println!("{}", format!("Copied '{}' -> '{}'", key, dest_key).green());
                    copied += 1;
                }
                Err(e) => {
                    println!("{}", format!("Error copying '{}': {}", key, e).red());
                }
            }
        } else {
            println!(
                "{}",
                format!("Key '{}' not found or is not a string", key).yellow()
            );
        }
    }
    println!(
        "{}",
        format!("Copy operation completed. {} keys copied.", copied).green()
    );
    Ok(())
}

async fn copy_between_environments(config: &AppConfig) -> Result<(), AppError> {
    println!("{}", "Copying between environments".cyan());
    println!("Available environments:");
    for (name, _) in &config.environments {
        println!("  - {}", name);
    }

    print!("Enter source environment: ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let source_env = input.trim();

    print!("Enter destination environment: ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let dest_env = input2.trim();

    let source_config = config
        .get_environment(source_env)
        .ok_or_else(|| {
            AppError::ConfigError(format!("Source environment '{}' not found", source_env))
        })?
        .config
        .clone();
    let dest_config = config
        .get_environment(dest_env)
        .ok_or_else(|| {
            AppError::ConfigError(format!("Destination environment '{}' not found", dest_env))
        })?
        .config
        .clone();

    print!("Enter source key pattern (e.g., 'user:*'): ");
    io::stdout().flush().unwrap();
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).unwrap();
    let source_pattern = input3.trim();

    println!(
        "{}",
        format!(
            "Copying from '{}' to '{}' with pattern '{}'",
            source_env, dest_env, source_pattern
        )
        .cyan()
    );

    // Connect to source environment
    let mut source_client = RedisClient::connect(source_config).await?;
    let keys = source_client.keys(source_pattern).await?;
    if keys.is_empty() {
        println!("{}", "No keys found matching the pattern.".yellow());
        return Ok(());
    }

    println!("{}", format!("Found {} keys to copy", keys.len()).green());

    // Connect to destination environment
    let mut dest_client = RedisClient::connect(dest_config).await?;
    let mut copied = 0;
    for key in keys {
        // Get value from source
        if let Some(value) = source_client.get_string(&key).await? {
            // Set value in destination
            match dest_client.set_string(&key, &value, None).await {
                Ok(_) => {
                    println!("{}", format!("Copied '{}'", key).green());
                    copied += 1;
                }
                Err(e) => {
                    println!("{}", format!("Error copying '{}': {}", key, e).red());
                }
            }
        } else {
            println!(
                "{}",
                format!("Key '{}' not found or is not a string", key).yellow()
            );
        }
    }
    println!(
        "{}",
        format!("Cross-environment copy completed. {} keys copied.", copied).green()
    );
    Ok(())
}
