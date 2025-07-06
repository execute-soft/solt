use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Stats command - placeholder");
    println!("{}", "Stats command - not yet implemented".yellow());
    Ok(())
}
