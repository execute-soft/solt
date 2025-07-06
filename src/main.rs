mod app;
mod cli;
mod commands;
mod config;
mod error;
mod redis_client;

use error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    app::run().await
}
