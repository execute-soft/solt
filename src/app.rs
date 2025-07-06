use clap::Parser;
use colored::*;
use log::info;

use crate::cli::{Cli, Commands};
use crate::commands::{
    backup, bulk, cluster, config, connect, copy, debug, delete, edit, export, favorites, filter,
    get, hello, history, inspect, keys, monitor, pubsub, search, sentinel, set, stats, version,
};
use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Validate environment if provided
    if let Some(ref env) = cli.environment {
        let valid_environments = vec!["dev", "staging", "prod"];
        if !valid_environments.contains(&env.as_str()) {
            println!("{}", "Error: Invalid environment specified".red());
            println!(
                "{}",
                format!("Available environments: {}", valid_environments.join(", ")).yellow()
            );
            println!("{}", "Example: cargo run -- -e dev keys".cyan());
            return Ok(());
        }
    }

    // Set log level based on verbosity
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }

    // Initialize logging
    env_logger::init();

    match cli.command {
        None => {
            // Show welcome message and available environments
            println!("{}", "Welcome to Solt - Redis CLI Management Tool!".bold());
            println!("{}", "==================================================");
            println!("Use --help to see available commands.");
            println!();
            println!("{}", "Quick Start:".bold());
            println!("  solt connect                    # Connect to Redis");
            println!("  solt keys                       # List all keys");
            println!("  solt get <key>                  # Get a value");
            println!("  solt set <key> <value>          # Set a value");
            println!("  solt monitor                    # Monitor Redis in real-time");
            println!();
            println!("{}", "Environment Usage:".bold());
            println!("  solt -e dev keys               # Use dev environment");
            println!("  solt -e staging keys           # Use staging environment");
            println!("  solt -e prod keys              # Use production environment");
            println!();
            println!("{}", "For more information, run: solt --help".cyan());
        }
        Some(Commands::Hello { name }) => {
            hello::run(name).await?;
        }
        Some(Commands::Version) => {
            version::run().await?;
        }

        // Connection & Config commands
        Some(Commands::Connect(args)) => {
            if args.test {
                let env = cli.environment.unwrap_or_else(|| "dev".to_string());
                connect::test_connection(&env).await?;
            } else {
                connect::run(
                    args.host,
                    args.port,
                    args.password,
                    args.db,
                    cli.environment,
                    args.timeout,
                    args.tls,
                )
                .await?;
            }
        }
        Some(Commands::Config(args)) => {
            if args.show {
                config::run().await?;
            } else if let Some(name) = args.add_env {
                // For simplicity, using default values. In a real app, you'd prompt for these
                config::add_environment(
                    name,
                    "localhost".to_string(),
                    6379,
                    None,
                    0,
                    Some(30),
                    false,
                )
                .await?;
            } else if let Some(name) = args.remove_env {
                config::remove_environment(&name).await?;
            } else if let Some(name) = args.set_default {
                config::set_default_environment(&name).await?;
            } else if let Some(format) = args.output_format {
                let output_format = match format.as_str() {
                    "json" => crate::config::OutputFormat::Json,
                    "table" => crate::config::OutputFormat::Table,
                    "csv" => crate::config::OutputFormat::Csv,
                    "plain" => crate::config::OutputFormat::Plain,
                    _ => {
                        println!(
                            "{}",
                            "Invalid output format. Use: json, table, csv, plain".red()
                        );
                        return Ok(());
                    }
                };
                config::set_output_format(output_format).await?;
            } else if let Some(size) = args.history_size {
                config::set_history_size(size).await?;
            } else {
                config::run().await?;
            }
        }

        // Key Inspection commands
        Some(Commands::Keys(args)) => {
            if args.count {
                keys::count_keys(Some(args.pattern), cli.environment).await?;
            } else {
                keys::run(Some(args.pattern), cli.environment, args.detailed).await?;
            }
        }
        Some(Commands::Inspect(_args)) => {
            inspect::run().await?;
        }

        // Value Viewing commands
        Some(Commands::Get(args)) => {
            if let Some(hash_field) = args.hash_field {
                let parts: Vec<&str> = hash_field.split(':').collect();
                if parts.len() == 2 {
                    get::get_hash_field(
                        parts[0].to_string(),
                        parts[1].to_string(),
                        cli.environment,
                    )
                    .await?;
                } else {
                    println!("{}", "Hash field format should be 'key:field'".red());
                }
            } else if let Some(list_range) = args.list_range {
                let parts: Vec<&str> = list_range.split('-').collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(stop)) =
                        (parts[0].parse::<isize>(), parts[1].parse::<isize>())
                    {
                        get::get_list_range(args.key, start, stop, cli.environment).await?;
                    } else {
                        println!(
                            "{}",
                            "List range format should be 'start-stop' (numbers)".red()
                        );
                    }
                } else {
                    println!("{}", "List range format should be 'start-stop'".red());
                }
            } else {
                get::run(args.key, cli.environment, args.pretty).await?;
            }
        }
        Some(Commands::Set(args)) => {
            if let Some(hash_field) = args.hash_field {
                let parts: Vec<&str> = hash_field.split(':').collect();
                if parts.len() == 3 {
                    set::set_hash_field(
                        parts[0].to_string(),
                        parts[1].to_string(),
                        parts[2].to_string(),
                        cli.environment,
                    )
                    .await?;
                } else {
                    println!("{}", "Hash field format should be 'key:field:value'".red());
                }
            } else if let Some(push_list) = args.push_list {
                let left = push_list.to_lowercase() == "left";
                set::push_list(args.key, args.value, cli.environment, left).await?;
            } else if let Some(add_set) = args.add_set {
                set::add_to_set(args.key, add_set, cli.environment).await?;
            } else if let Some(add_zset) = args.add_zset {
                let parts: Vec<&str> = add_zset.split(':').collect();
                if parts.len() == 2 {
                    if let Ok(score) = parts[1].parse::<f64>() {
                        set::add_to_sorted_set(
                            args.key,
                            parts[0].to_string(),
                            score,
                            cli.environment,
                        )
                        .await?;
                    } else {
                        println!("{}", "Score should be a number".red());
                    }
                } else {
                    println!("{}", "Sorted set format should be 'member:score'".red());
                }
            } else {
                set::run(args.key, args.value, cli.environment, args.ttl).await?;
            }
        }

        // Search & Filter commands
        Some(Commands::Search(_args)) => {
            search::run().await?;
        }
        Some(Commands::Filter(_args)) => {
            filter::run().await?;
        }

        // Editing & Writing commands
        Some(Commands::Edit(_args)) => {
            edit::run().await?;
        }
        Some(Commands::Delete(args)) => {
            if let Some(pattern) = args.pattern {
                delete::delete_by_pattern(pattern, cli.environment, args.confirm).await?;
            } else if args.flush_db {
                delete::flush_db(cli.environment, args.confirm).await?;
            } else if args.flush_all {
                delete::flush_all(cli.environment, args.confirm).await?;
            } else {
                delete::run(args.key, cli.environment).await?;
            }
        }

        // Bulk Operations commands
        Some(Commands::Bulk(_args)) => {
            bulk::run().await?;
        }
        Some(Commands::Copy(_args)) => {
            copy::run().await?;
        }

        // Monitoring & Debug commands
        Some(Commands::Monitor(args)) => {
            if args.slowlog {
                monitor::slowlog_get(Some(args.slowlog_count), cli.environment).await?;
            } else if args.clients {
                monitor::client_list(cli.environment).await?;
            } else {
                monitor::run(cli.environment).await?;
            }
        }
        Some(Commands::Debug(_args)) => {
            debug::run().await?;
        }
        Some(Commands::Stats(_args)) => {
            stats::run().await?;
        }

        // Backup & Export commands
        Some(Commands::Backup(_args)) => {
            backup::run().await?;
        }
        Some(Commands::Export(_args)) => {
            export::run().await?;
        }

        // Pub/Sub commands
        Some(Commands::Pubsub(_args)) => {
            pubsub::run().await?;
        }

        // Cluster & Sentinel commands
        Some(Commands::Cluster(_args)) => {
            cluster::run().await?;
        }
        Some(Commands::Sentinel(_args)) => {
            sentinel::run().await?;
        }

        // UX Features commands
        Some(Commands::Favorites(_args)) => {
            favorites::run().await?;
        }
        Some(Commands::History(_args)) => {
            history::run().await?;
        }
    }

    info!("CLI application completed successfully.");
    Ok(())
}
