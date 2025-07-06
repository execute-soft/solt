use colored::*;
use log::info;
use tabled::{Table, Tabled};

use crate::config::AppConfig;
use crate::error::AppError;
use crate::redis_client::RedisClient;

#[derive(Tabled)]
struct KeyRow {
    #[tabled(rename = "Key")]
    key: String,
    #[tabled(rename = "Type")]
    key_type: String,
    #[tabled(rename = "TTL")]
    ttl: String,
    #[tabled(rename = "Memory")]
    memory: String,
    #[tabled(rename = "Encoding")]
    encoding: String,
}

pub async fn run(
    pattern: Option<String>,
    environment: Option<String>,
    detailed: bool,
) -> Result<(), AppError> {
    info!("Running keys command with pattern: {:?}", pattern);

    let config = AppConfig::load()?;
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let redis_config = match config.get_environment(&env_name) {
        Some(env) => env.config.clone(),
        None => {
            println!(
                "{}",
                format!("Error: Environment '{}' not found", env_name).red()
            );
            println!("{}", "Available environments:".yellow());
            for env_name in config.environments.keys() {
                println!("  • {}", env_name.cyan());
            }
            println!();
            println!("{}", "To add a new environment, use:".cyan());
            println!("  solt config --add-env <name>");
            return Ok(());
        }
    };

    let mut client = RedisClient::connect(redis_config).await?;

    let pattern = pattern.unwrap_or_else(|| "*".to_string());
    let keys = client.keys(&pattern).await?;

    println!(
        "{}",
        format!("Found {} keys matching pattern '{}'", keys.len(), pattern)
            .cyan()
            .bold()
    );

    if keys.is_empty() {
        println!("{}", "No keys found.".yellow());
        return Ok(());
    }

    if detailed {
        // Get detailed information for each key
        let mut key_infos = Vec::new();
        let progress = indicatif::ProgressBar::new(keys.len() as u64);
        progress.set_message("Getting key details...");

        for key in &keys {
            match client.key_info(key).await {
                Ok(info) => key_infos.push(info),
                Err(e) => {
                    println!(
                        "{}",
                        format!("Error getting info for key '{}': {}", key, e).red()
                    );
                }
            }
            progress.inc(1);
        }
        progress.finish_with_message("Key details retrieved");

        // Display as table
        let rows: Vec<KeyRow> = key_infos
            .into_iter()
            .map(|info| KeyRow {
                key: info.key,
                key_type: info.key_type,
                ttl: info
                    .ttl
                    .map(|t| {
                        if t == -1 {
                            "No expiry".to_string()
                        } else if t == -2 {
                            "Key doesn't exist".to_string()
                        } else {
                            format!("{}s", t)
                        }
                    })
                    .unwrap_or_else(|| "Unknown".to_string()),
                memory: info
                    .memory_usage
                    .map(|m| format!("{} bytes", m))
                    .unwrap_or_else(|| "Unknown".to_string()),
                encoding: info.encoding,
            })
            .collect();

        let table = Table::new(rows).to_string();
        println!("{}", table);
    } else {
        // Simple list
        for key in keys {
            println!("• {}", key.cyan());
        }
    }

    Ok(())
}

pub async fn count_keys(
    pattern: Option<String>,
    environment: Option<String>,
) -> Result<(), AppError> {
    info!("Counting keys with pattern: {:?}", pattern);

    let config = AppConfig::load()?;
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let redis_config = match config.get_environment(&env_name) {
        Some(env) => env.config.clone(),
        None => {
            println!(
                "{}",
                format!("Error: Environment '{}' not found", env_name).red()
            );
            println!("{}", "Available environments:".yellow());
            for env_name in config.environments.keys() {
                println!("  • {}", env_name.cyan());
            }
            println!();
            println!("{}", "To add a new environment, use:".cyan());
            println!("  solt config --add-env <name>");
            return Ok(());
        }
    };

    let mut client = RedisClient::connect(redis_config).await?;

    let pattern = pattern.unwrap_or_else(|| "*".to_string());
    let keys = client.keys(&pattern).await?;

    println!(
        "{}",
        format!("Found {} keys matching pattern '{}'", keys.len(), pattern)
            .green()
            .bold()
    );

    // Group by type if we have keys
    if !keys.is_empty() {
        let mut type_counts = std::collections::HashMap::new();

        for key in &keys {
            match client.key_info(key).await {
                Ok(info) => {
                    *type_counts.entry(info.key_type).or_insert(0) += 1;
                }
                Err(_) => {
                    *type_counts.entry("unknown".to_string()).or_insert(0) += 1;
                }
            }
        }

        println!("\n{}", "Breakdown by type:".bold());
        for (key_type, count) in type_counts {
            println!("• {}: {}", key_type.cyan(), count.to_string().yellow());
        }
    }

    Ok(())
}
