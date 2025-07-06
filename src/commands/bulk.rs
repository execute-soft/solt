use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Bulk command - placeholder");
    println!("{}", "Bulk command - not yet implemented".yellow());
    Ok(())
}
