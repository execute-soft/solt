use colored::*;
use log::info;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::redis_client::RedisClient;

pub async fn run() -> Result<(), AppError> {
    info!("Backup command invoked");
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

    println!(
        "{}",
        "Choose backup operation: [1] SAVE, [2] BGSAVE, [3] BGREWRITEAOF".cyan()
    );
    println!("{}", "1. SAVE (synchronous save, blocks Redis)".yellow());
    println!("{}", "2. BGSAVE (background save, non-blocking)".yellow());
    println!("{}", "3. BGREWRITEAOF (background AOF rewrite)".yellow());
    print!("Enter choice (1/2/3): ");
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    match choice {
        "1" => {
            println!("{}", "Running SAVE...".cyan());
            client.save(false).await?;
            println!("{}", "SAVE completed successfully.".green());
        }
        "2" => {
            println!("{}", "Running BGSAVE...".cyan());
            client.save(true).await?;
            println!("{}", "BGSAVE triggered successfully.".green());
        }
        "3" => {
            println!("{}", "Running BGREWRITEAOF...".cyan());
            client.bgrewriteaof().await?;
            println!("{}", "BGREWRITEAOF triggered successfully.".green());
        }
        _ => {
            println!("{}", "Invalid choice. Aborting.".red());
        }
    }
    Ok(())
}
