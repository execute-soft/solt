use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("Cluster command - placeholder");
    println!("{}", "Cluster command - not yet implemented".yellow());
    Ok(())
}
