use colored::*;
use log::info;

use crate::config::{AppConfig, RedisConfig};
use crate::error::AppError;
use crate::redis_client::RedisClient;

pub async fn run(
    host: Option<String>,
    port: Option<u16>,
    password: Option<String>,
    db: Option<u8>,
    environment: Option<String>,
    timeout: Option<u64>,
    tls: bool,
) -> Result<(), AppError> {
    info!("Running connect command");

    let mut config = AppConfig::load()?;

    // Determine which environment to use
    let env_name = environment.unwrap_or_else(|| {
        config
            .default_environment
            .clone()
            .unwrap_or_else(|| "dev".to_string())
    });

    let mut redis_config = if let Some(env) = config.get_environment(&env_name) {
        env.config.clone()
    } else {
        // Create new environment with provided parameters
        RedisConfig {
            host: host.clone().unwrap_or_else(|| "localhost".to_string()),
            port: port.unwrap_or(6379),
            password: password.clone(),
            db: db.unwrap_or(0),
            timeout,
            tls,
        }
    };

    // Override with command line parameters if provided
    if let Some(host) = &host {
        redis_config.host = host.clone();
    }
    if let Some(port) = port {
        redis_config.port = port;
    }
    if let Some(password) = &password {
        redis_config.password = Some(password.clone());
    }
    if let Some(db) = db {
        redis_config.db = db;
    }
    if let Some(timeout) = timeout {
        redis_config.timeout = Some(timeout);
    }
    redis_config.tls = tls;

    println!("{}", "Connecting to Redis...".yellow());
    println!("Host: {}", redis_config.host.cyan());
    println!("Port: {}", redis_config.port.to_string().cyan());
    println!("Database: {}", redis_config.db.to_string().cyan());
    println!(
        "TLS: {}",
        if redis_config.tls {
            "Yes".green()
        } else {
            "No".red()
        }
    );

    // Test connection
    let test_config = redis_config.clone();
    match RedisClient::connect(test_config).await {
        Ok(mut client) => {
            println!("{}", "✓ Connected successfully!".green().bold());

            // Test ping
            match client.ping().await {
                Ok(response) => println!("Ping: {}", response.green()),
                Err(e) => println!("{}", format!("Ping failed: {}", e).red()),
            }

            // Show INFO
            match client.info().await {
                Ok(info) => {
                    println!("\n{}", "Redis Server Information:".bold());
                    println!("{}", "=".repeat(50));

                    let important_keys = [
                        "redis_version",
                        "os",
                        "arch_bits",
                        "process_id",
                        "uptime_in_seconds",
                        "uptime_in_days",
                        "connected_clients",
                        "used_memory_human",
                        "used_memory_peak_human",
                    ];

                    for key in &important_keys {
                        if let Some(value) = info.get(*key) {
                            println!("{}: {}", key.cyan(), value.yellow());
                        }
                    }
                }
                Err(e) => println!("{}", format!("Failed to get INFO: {}", e).red()),
            }

            // Save environment if it's new
            if config.get_environment(&env_name).is_none() {
                config.add_environment(env_name.clone(), redis_config);
                config.default_environment = Some(env_name);
                config.save()?;
                println!("{}", "Environment saved to config".green());
            }

            Ok(())
        }
        Err(e) => {
            println!("{}", format!("✗ Connection failed: {}", e).red().bold());
            Err(AppError::ConnectionError(e.to_string()))
        }
    }
}

pub async fn test_connection(environment: &str) -> Result<(), AppError> {
    let config = AppConfig::load()?;

    let redis_config = config
        .get_environment(environment)
        .ok_or_else(|| AppError::ConfigError(format!("Environment '{}' not found", environment)))?
        .config
        .clone();

    match RedisClient::connect(redis_config).await {
        Ok(mut client) => match client.ping().await {
            Ok(_) => {
                println!("{}", "✓ Connection test successful!".green().bold());
                Ok(())
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("✗ Connection test failed: {}", e).red().bold()
                );
                Err(AppError::ConnectionError(e.to_string()))
            }
        },
        Err(e) => {
            println!("{}", format!("✗ Connection failed: {}", e).red().bold());
            Err(AppError::ConnectionError(e.to_string()))
        }
    }
}
