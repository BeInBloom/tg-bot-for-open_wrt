//! Bot module - multi-messenger architecture.

mod auth;
mod commands;
mod formatters;
mod handlers;
mod messages;
pub mod telegram;

use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::domain::messenger::Bot;
use crate::domain::{RouterInfo, ShutdownSignal};
use crate::infrastructure::Config;

pub use auth::UserWhitelist;
pub use telegram::TelegramBot;

const KEY_BOT_TOKEN: &str = "BOT_TOKEN";
const KEY_ALLOWED_USERS: &str = "BOT_ALLOWED_USERS";

type BotHandle = (JoinHandle<()>, tokio::sync::mpsc::Sender<()>);

pub struct BotManager {
    bots: Vec<Box<dyn Bot>>,
}

impl BotManager {
    pub fn new() -> Self {
        Self { bots: Vec::new() }
    }

    pub fn from_config<R: RouterInfo + 'static>(
        config: &Config,
        router: Arc<R>,
    ) -> anyhow::Result<Self> {
        let mut manager = Self::new();
        manager.add(create_telegram_bot(config, router)?);
        Ok(manager)
    }

    pub fn add<B: Bot + 'static>(&mut self, bot: B) {
        self.bots.push(Box::new(bot));
    }

    pub async fn run_all(self, mut shutdown_rx: ShutdownSignal) {
        if self.bots.is_empty() {
            tracing::warn!("No bots configured");
            return;
        }

        tracing::info!("Starting {} bot(s)", self.bots.len());

        let (broadcast_tx, _) = broadcast::channel::<()>(1);
        let handles = spawn_all_bots(self.bots, &broadcast_tx);

        wait_for_shutdown(&mut shutdown_rx).await;
        stop_all_bots(broadcast_tx, handles).await;

        tracing::info!("All bots stopped");
    }
}

fn create_telegram_bot<R: RouterInfo + 'static>(
    config: &Config,
    router: Arc<R>,
) -> anyhow::Result<TelegramBot<R, UserWhitelist>> {
    let token = config.required(KEY_BOT_TOKEN)?;
    let allowed_users: Vec<u64> = config.required_list(KEY_ALLOWED_USERS)?;
    let auth = UserWhitelist::from_iter(allowed_users);

    Ok(TelegramBot::new(token, router, auth))
}

fn spawn_all_bots(bots: Vec<Box<dyn Bot>>, broadcast_tx: &broadcast::Sender<()>) -> Vec<BotHandle> {
    bots.into_iter()
        .map(|bot| spawn_bot(bot, broadcast_tx))
        .collect()
}

fn spawn_bot(bot: Box<dyn Bot>, broadcast_tx: &broadcast::Sender<()>) -> BotHandle {
    let mut rx = broadcast_tx.subscribe();
    let (bot_tx, bot_rx) = tokio::sync::mpsc::channel(1);

    let handle = tokio::spawn(async move {
        tokio::select! {
            _ = bot.run(bot_rx) => {}
            _ = rx.recv() => {}
        }
    });

    (handle, bot_tx)
}

async fn wait_for_shutdown(shutdown_rx: &mut ShutdownSignal) {
    shutdown_rx.recv().await;
    tracing::info!("Shutting down all bots...");
}

async fn stop_all_bots(broadcast_tx: broadcast::Sender<()>, handles: Vec<BotHandle>) {
    drop(broadcast_tx);
    for (handle, tx) in handles {
        let _ = tx.send(()).await;
        let _ = handle.await;
    }
}

impl Default for BotManager {
    fn default() -> Self {
        Self::new()
    }
}
