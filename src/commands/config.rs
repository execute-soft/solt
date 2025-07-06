use colored::*;
use log::info;
use tabled::{Table, Tabled};

use crate::config::{AppConfig, OutputFormat, RedisConfig};
use crate::error::AppError;

#[derive(Tabled)]
struct EnvironmentRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Host")]
    host: String,
    #[tabled(rename = "Port")]
    port: String,
    #[tabled(rename = "Database")]
    db: String,
    #[tabled(rename = "TLS")]
    tls: String,
    #[tabled(rename = "Default")]
    default: String,
}

pub async fn run() -> Result<(), AppError> {
    info!("Running config command");

    let config = AppConfig::load()?;

    println!("{}", "Current Configuration:".bold());
    println!("{}", "=".repeat(50));

    // Show default environment
    if let Some(default_env) = &config.default_environment {
        println!("Default Environment: {}", default_env.cyan().bold());
    }

    // Show output format
    let format_str = match config.output_format {
        OutputFormat::Json => "JSON",
        OutputFormat::Table => "Table",
        OutputFormat::Csv => "CSV",
        OutputFormat::Plain => "Plain",
    };
    println!("Output Format: {}", format_str.cyan());
    println!("History Size: {}", config.history_size.to_string().cyan());

    // Show environments
    println!("\n{}", "Environments:".bold());
    println!("{}", "=".repeat(50));

    let mut rows = Vec::new();
    for (name, env) in &config.environments {
        let is_default = config
            .default_environment
            .as_ref()
            .map_or(false, |d| d == name);
        rows.push(EnvironmentRow {
            name: name.clone(),
            host: env.config.host.clone(),
            port: env.config.port.to_string(),
            db: env.config.db.to_string(),
            tls: if env.config.tls {
                "Yes".green().to_string()
            } else {
                "No".red().to_string()
            },
            default: if is_default {
                "✓".green().to_string()
            } else {
                "".to_string()
            },
        });
    }

    let table = Table::new(rows).to_string();
    println!("{}", table);

    // Show favorites
    if !config.favorites.is_empty() {
        println!("\n{}", "Favorites:".bold());
        println!("{}", "=".repeat(50));
        for favorite in &config.favorites {
            println!("• {}", favorite.cyan());
        }
    }

    Ok(())
}

pub async fn add_environment(
    name: String,
    host: String,
    port: u16,
    password: Option<String>,
    db: u8,
    timeout: Option<u64>,
    tls: bool,
) -> Result<(), AppError> {
    info!("Adding environment: {}", name);

    let mut config = AppConfig::load()?;

    let redis_config = RedisConfig {
        host,
        port,
        password,
        db,
        timeout,
        tls,
    };

    config.add_environment(name.clone(), redis_config);
    config.save()?;

    println!(
        "{}",
        format!("✓ Environment '{}' added successfully!", name)
            .green()
            .bold()
    );
    Ok(())
}

pub async fn remove_environment(name: &str) -> Result<(), AppError> {
    info!("Removing environment: {}", name);

    let mut config = AppConfig::load()?;

    if config.remove_environment(name) {
        config.save()?;
        println!(
            "{}",
            format!("✓ Environment '{}' removed successfully!", name)
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            format!("✗ Environment '{}' not found!", name).red().bold()
        );
    }

    Ok(())
}

pub async fn set_default_environment(name: &str) -> Result<(), AppError> {
    info!("Setting default environment: {}", name);

    let mut config = AppConfig::load()?;

    if config.get_environment(name).is_some() {
        config.default_environment = Some(name.to_string());
        config.save()?;
        println!(
            "{}",
            format!("✓ Default environment set to '{}'", name)
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            format!("✗ Environment '{}' not found!", name).red().bold()
        );
    }

    Ok(())
}

pub async fn set_output_format(format: OutputFormat) -> Result<(), AppError> {
    info!("Setting output format");

    let mut config = AppConfig::load()?;
    config.output_format = format;
    config.save()?;

    let format_str = match config.output_format {
        OutputFormat::Json => "JSON",
        OutputFormat::Table => "Table",
        OutputFormat::Csv => "CSV",
        OutputFormat::Plain => "Plain",
    };

    println!(
        "{}",
        format!("✓ Output format set to {}", format_str)
            .green()
            .bold()
    );
    Ok(())
}

pub async fn set_history_size(size: usize) -> Result<(), AppError> {
    info!("Setting history size: {}", size);

    let mut config = AppConfig::load()?;
    config.history_size = size;
    config.save()?;

    println!(
        "{}",
        format!("✓ History size set to {}", size).green().bold()
    );
    Ok(())
}
