//! Telegram bot for OpenWRT router management.

mod bot;
mod core;
mod domain;
mod infrastructure;

use std::sync::Arc;

use bot::BotManager;
use core::App;
use infrastructure::{Config, OpenWrtRouter, UnixSignalHandler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_config()?;

    let config = Config::new();
    let router = Arc::new(OpenWrtRouter::new());
    let bot_manager = BotManager::from_config(&config, router)?;

    let app = App::new(&config, UnixSignalHandler::new(), bot_manager)?;
    app.run().await
}

fn load_config() -> anyhow::Result<()> {
    dotenvy::from_path("./config")?;
    Ok(())
}
