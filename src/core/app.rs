//! Application orchestration.

use crate::bot::BotManager;
use crate::domain::SignalHandler;
use crate::domain::types::{ShutdownSender, ShutdownSignal};
use crate::infrastructure::{Config, LogGuard};

pub struct App<S: SignalHandler> {
    _log_guard: LogGuard,
    signal_handler: S,
    bot_manager: BotManager,
}

impl<S: SignalHandler> App<S> {
    pub fn new(
        config: &Config,
        signal_handler: S,
        bot_manager: BotManager,
    ) -> anyhow::Result<Self> {
        let log_guard = crate::infrastructure::init_logging(config)?;

        Ok(Self {
            _log_guard: log_guard,
            signal_handler,
            bot_manager,
        })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("Application started");

        let (shutdown_tx, shutdown_rx) = create_shutdown_channel();
        let handle = tokio::spawn(self.bot_manager.run_all(shutdown_rx));

        wait_for_signal(&self.signal_handler).await;

        let _ = shutdown_tx.send(()).await;
        await_completion(handle).await;

        tracing::info!("Shutdown complete");
        Ok(())
    }
}

fn create_shutdown_channel() -> (ShutdownSender, ShutdownSignal) {
    tokio::sync::mpsc::channel(1)
}

async fn wait_for_signal<S: SignalHandler>(handler: &S) {
    let signal = handler.wait_for_shutdown().await;
    tracing::info!("Received {signal}, stopping...");
}

async fn await_completion(handle: tokio::task::JoinHandle<()>) {
    match handle.await {
        Ok(()) => tracing::info!("All bots stopped"),
        Err(e) => tracing::error!("Bot manager panicked: {e}"),
    }
}
