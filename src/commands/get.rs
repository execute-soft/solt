use colored::*;
use log::info;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::redis_client::RedisClient;

pub async fn run(key: String, environment: Option<String>, pretty: bool) -> Result<(), AppError> {
    info!("Getting value for key: {}", key);

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

    // First get key info to determine type
    let key_info = client.key_info(&key).await?;

    println!("{}", format!("Key: {}", key).bold());
    println!("{}", format!("Type: {}", key_info.key_type).cyan());

    match key_info.key_type.as_str() {
        "string" => {
            if let Some(value) = client.get_string(&key).await? {
                if pretty {
                    match client.pretty_print_json(&value) {
                        Ok(pretty_value) => {
                            println!("{}", "Value (pretty-printed):".bold());
                            println!("{}", pretty_value);
                        }
                        Err(_) => {
                            println!("{}", "Value:".bold());
                            println!("{}", value);
                        }
                    }
                } else {
                    println!("{}", "Value:".bold());
                    println!("{}", value);
                }
            } else {
                println!("{}", "Key not found or value is nil".red());
            }
        }
        "hash" => {
            let hash = client.get_hash(&key).await?;
            if hash.is_empty() {
                println!("{}", "Hash is empty".yellow());
            } else {
                println!("{}", "Hash fields:".bold());
                for (field, value) in hash {
                    println!("  {}: {}", field.cyan(), value);
                }
            }
        }
        "list" => {
            let list = client.get_list(&key, 0, -1).await?;
            if list.is_empty() {
                println!("{}", "List is empty".yellow());
            } else {
                println!("{}", format!("List ({} items):", list.len()).bold());
                for (i, item) in list.iter().enumerate() {
                    println!("  [{}]: {}", i, item);
                }
            }
        }
        "set" => {
            let set = client.get_set(&key).await?;
            if set.is_empty() {
                println!("{}", "Set is empty".yellow());
            } else {
                println!("{}", format!("Set ({} members):", set.len()).bold());
                for member in set {
                    println!("  • {}", member);
                }
            }
        }
        "zset" => {
            let zset = client.get_sorted_set(&key, 0, -1, true).await?;
            if zset.is_empty() {
                println!("{}", "Sorted set is empty".yellow());
            } else {
                println!("{}", format!("Sorted set ({} members):", zset.len()).bold());
                for (member, score) in zset {
                    println!("  • {} (score: {})", member, score);
                }
            }
        }
        _ => {
            println!(
                "{}",
                format!("Unsupported key type: {}", key_info.key_type).red()
            );
        }
    }

    Ok(())
}

pub async fn get_hash_field(
    key: String,
    field: String,
    environment: Option<String>,
) -> Result<(), AppError> {
    info!("Getting hash field: {}:{}", key, field);

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

    // Get the entire hash and find the specific field
    let hash = client.get_hash(&key).await?;

    if let Some(value) = hash.get(&field) {
        println!("{}", format!("Field: {}:{}", key, field).bold());
        println!("{}", "Value:".bold());
        println!("{}", value);
    } else {
        println!(
            "{}",
            format!("Field '{}' not found in hash '{}'", field, key).red()
        );
    }

    Ok(())
}

pub async fn get_list_range(
    key: String,
    start: isize,
    stop: isize,
    environment: Option<String>,
) -> Result<(), AppError> {
    info!("Getting list range: {} [{}-{}]", key, start, stop);

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

    let list = client.get_list(&key, start, stop).await?;

    if list.is_empty() {
        println!("{}", "No items found in the specified range".yellow());
    } else {
        println!(
            "{}",
            format!("List range [{}-{}] ({} items):", start, stop, list.len()).bold()
        );
        for (i, item) in list.iter().enumerate() {
            let actual_index = start + i as isize;
            println!("  [{}]: {}", actual_index, item);
        }
    }

    Ok(())
}
