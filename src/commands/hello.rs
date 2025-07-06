use colored::*;
use log::info;

use crate::error::AppError;

pub async fn run(name: String) -> Result<(), AppError> {
    info!("Running hello command with name: {}", name);

    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting.green().bold());

    // Example of different colored outputs
    println!("{}", "This is a success message".green());
    println!("{}", "This is a warning message".yellow());
    println!("{}", "This is an error message".red());

    info!("Hello command completed successfully");
    Ok(())
}
