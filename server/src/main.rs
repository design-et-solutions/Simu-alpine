use anyhow::Result;
use config::CONFIG;
use log_manager::LogManager;

mod config;
mod models;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = LogManager::new(&CONFIG.log_level, "HOURLY", 10)?;
    server::run().await?;
    Ok(())
}
