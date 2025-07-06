use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Sentinel command - placeholder");
    println!("{}", "Sentinel command - not yet implemented".yellow());
    Ok(())
}
