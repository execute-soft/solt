use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Running version command");

    println!("{}", "CLI Starter".blue().bold());
    println!("Version: {}", env!("CARGO_PKG_VERSION").cyan());
    println!("Author: {}", env!("CARGO_PKG_AUTHORS").yellow());
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION").green());

    info!("Version command completed successfully");
    Ok(())
}
