use tokio::sync::mpsc;

pub type ShutdownSignal = mpsc::Receiver<()>;
pub type ShutdownSender = mpsc::Sender<()>;
