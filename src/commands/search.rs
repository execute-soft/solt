use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Search command - placeholder");
    println!("{}", "Search command - not yet implemented".yellow());
    Ok(())
}
