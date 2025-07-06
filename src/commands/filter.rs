use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Filter command - placeholder");
    println!("{}", "Filter command - not yet implemented".yellow());
    Ok(())
}
