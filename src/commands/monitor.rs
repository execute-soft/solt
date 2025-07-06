use colored::*;
use log::info;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::AppConfig;
use crate::error::AppError;
use crate::redis_client::RedisClient;

pub async fn run(environment: Option<String>) -> Result<(), AppError> {
    info!("Starting Redis monitor");

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

    println!("{}", "Starting Redis MONITOR...".yellow().bold());
    println!("{}", "Press Ctrl+C to stop".cyan());
    println!("{}", "=".repeat(80));

    // Note: This is a simplified monitor. In a real implementation,
    // you'd want to handle the stream properly with proper error handling
    // and graceful shutdown on Ctrl+C

    match client.monitor().await {
        Ok(_) => {
            println!("{}", "Monitor stopped".green());
            Ok(())
        }
        Err(e) => {
            println!("{}", format!("Monitor error: {}", e).red());
            Err(AppError::ConnectionError(format!("Monitor failed: {}", e)))
        }
    }
}

pub async fn slowlog_get(
    count: Option<usize>,
    environment: Option<String>,
) -> Result<(), AppError> {
    info!("Getting slow log entries");

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

    let count = count.unwrap_or(10);
    let entries = client.slowlog_get(count).await?;

    if entries.is_empty() {
        println!("{}", "No slow log entries found".yellow());
        return Ok(());
    }

    println!(
        "{}",
        format!("Slow Log Entries (showing {}):", entries.len()).bold()
    );
    println!("{}", "=".repeat(80));

    for entry in entries {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            - entry.timestamp;

        let time_str = if timestamp < 60 {
            format!("{}s ago", timestamp)
        } else if timestamp < 3600 {
            format!("{}m ago", timestamp / 60)
        } else {
            format!("{}h ago", timestamp / 3600)
        };

        println!("{}", format!("ID: {}", entry.id).cyan());
        println!("  Time: {}", time_str.yellow());
        println!("  Duration: {}ms", entry.duration.to_string().red());
        println!("  Command: {}", entry.command);
        println!("{}", "-".repeat(40));
    }

    Ok(())
}

pub async fn client_list(environment: Option<String>) -> Result<(), AppError> {
    info!("Getting client list");

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

    let clients = client.client_list().await?;

    if clients.is_empty() {
        println!("{}", "No clients found".yellow());
        return Ok(());
    }

    println!(
        "{}",
        format!("Connected Clients ({}):", clients.len()).bold()
    );
    println!("{}", "=".repeat(80));

    for client_info in clients {
        println!("{}", format!("Client ID: {}", client_info.id).cyan().bold());
        println!("  Address: {}", client_info.addr);
        println!("  Database: {}", client_info.db);
        println!("  Age: {}", client_info.age);
        println!("  Idle: {}", client_info.idle);
        println!("  Flags: {}", client_info.flags);
        println!("  Command: {}", client_info.cmd);
        println!("  Memory: {} bytes", client_info.omem);
        println!("{}", "-".repeat(40));
    }

    Ok(())
}
