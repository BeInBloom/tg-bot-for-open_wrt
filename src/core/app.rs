use tokio::signal;

use crate::{
    domain::types::{ShutdownSender, ShutdownSignal},
    infrastructure::{Config, LogGuard},
};

/// Основная структура приложения
pub struct App {
    _log_guard: LogGuard,
}

impl App {
    /// Создает и инициализирует приложение
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        let log_guard = crate::infrastructure::init_logging(config)?;

        Ok(Self {
            _log_guard: log_guard,
        })
    }

    /// Запускает приложение
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("Application started");

        // Shutdown channel: для передачи сигнала завершения в бот
        let (shutdown_tx, shutdown_rx): (ShutdownSender, ShutdownSignal) =
            tokio::sync::mpsc::channel(1);

        // Запускаем бота в отдельной задаче
        let bot_handle = tokio::spawn(run_bot(shutdown_rx));

        // Ждем сигнал завершения от ОС
        Self::wait_for_shutdown_signal().await;

        tracing::info!("Shutdown signal received, stopping application...");

        // Отправляем сигнал боту на завершение
        let _ = shutdown_tx.send(()).await;

        // Ждем завершения бота (graceful shutdown)
        match bot_handle.await {
            Ok(_) => tracing::info!("Bot stopped gracefully"),
            Err(e) => tracing::error!("Bot task panicked: {}", e),
        }

        tracing::info!("Application shutdown complete");

        Ok(())
    }

    /// Ждет сигналы завершения от ОС (SIGINT, SIGTERM, SIGHUP)
    async fn wait_for_shutdown_signal() {
        use signal::unix::{SignalKind, signal};

        // Создаем слушателей для разных сигналов
        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");
        let mut sighup = signal(SignalKind::hangup()).expect("failed to install SIGHUP handler");

        tokio::select! {
            _ = sigterm.recv() => {
                tracing::info!("Received SIGTERM");
            }
            _ = sigint.recv() => {
                tracing::info!("Received SIGINT (Ctrl+C)");
            }
            _ = sighup.recv() => {
                tracing::info!("Received SIGHUP");
            }
        }
    }
}

/// Основной цикл бота
async fn run_bot(mut shutdown_rx: ShutdownSignal) {
    tracing::info!("Bot loop started");

    loop {
        tokio::select! {
            // Проверяем сигнал shutdown
            _ = shutdown_rx.recv() => {
                tracing::info!("Bot received shutdown signal");
                break;
            }
            // Симулируем работу бота
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(2)) => {
                tracing::info!("Bot is processing messages...");
                // Здесь будет реальная логика бота:
                // - Получение обновлений от Telegram
                // - Обработка команд
                // - и т.д.
            }
        }
    }

    tracing::info!("Bot loop finished");
}
