use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {
    info!("PubSub command - placeholder");
    println!("{}", "PubSub command - not yet implemented".yellow());
    Ok(())
}
