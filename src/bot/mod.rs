//! Bot module - multi-messenger architecture.

mod auth;
mod commands;
pub mod factory;
mod formatters;
mod handlers;
mod messages;
pub mod telegram;

use futures::future::join_all;
use tokio::task::JoinHandle;

use crate::domain::ShutdownSignal;
use crate::domain::messenger::Bot;

type BotHandle = (
    JoinHandle<anyhow::Result<()>>,
    tokio::sync::mpsc::Sender<()>,
);

pub struct BotManager {
    bots: Vec<Box<dyn Bot>>,
}

impl BotManager {
    pub fn new() -> Self {
        Self { bots: Vec::new() }
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

        let handles = self.spawn_all();
        wait_for_shutdown(&mut shutdown_rx).await;
        stop_all(handles).await;

        tracing::info!("All bots stopped");
    }

    fn spawn_all(self) -> Vec<BotHandle> {
        self.bots.into_iter().map(spawn_bot).collect()
    }
}

fn spawn_bot(bot: Box<dyn Bot>) -> BotHandle {
    let (tx, rx) = tokio::sync::mpsc::channel(1);
    let handle = tokio::spawn(async move { bot.run(rx).await });
    (handle, tx)
}

async fn wait_for_shutdown(shutdown_rx: &mut ShutdownSignal) {
    shutdown_rx.recv().await;
    tracing::info!("Shutting down all bots...");
}

async fn stop_all(handles: Vec<BotHandle>) {
    join_all(handles.iter().map(|(_, tx)| tx.send(()))).await;
    join_all(handles.into_iter().map(|(h, _)| h)).await;
}

impl Default for BotManager {
    fn default() -> Self {
        Self::new()
    }
}
