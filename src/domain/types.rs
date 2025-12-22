use tokio::sync::mpsc;

/// Signal to shutdown the application
pub type ShutdownSignal = mpsc::Receiver<()>;

/// Sender for shutdown signal
pub type ShutdownSender = mpsc::Sender<()>;
