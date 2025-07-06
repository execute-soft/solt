use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Inspect command - placeholder");
    println!("{}", "Inspect command - not yet implemented".yellow());
    Ok(())
}
