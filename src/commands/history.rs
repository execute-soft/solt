use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("History command - placeholder");
    println!("{}", "History command - not yet implemented".yellow());
    Ok(())
}
