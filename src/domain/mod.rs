//! Domain layer: traits, types, and error definitions.

pub mod error;
pub mod messenger;
pub mod router;
pub mod signal;
pub mod types;
pub mod ubus;
pub mod wifi_mode;

pub use error::RouterError;
pub use router::{RouterInfo, RouterStatus};
pub use signal::SignalHandler;
pub use types::ShutdownSignal;
pub use wifi_mode::WifiMode;
