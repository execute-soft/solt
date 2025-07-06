use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Debug command - placeholder");
    println!("{}", "Debug command - not yet implemented".yellow());
    Ok(())
}
