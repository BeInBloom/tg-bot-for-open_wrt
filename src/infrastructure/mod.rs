//! Infrastructure layer: external integrations and platform-specific code.
//!
//! Provides implementations for configuration, logging, router access, and signal handling.

pub mod config;
pub mod fs;
pub mod logging;
pub mod router;
pub mod signal;

pub use config::Config;
pub use logging::{LogGuard, init as init_logging};
pub use router::OpenWrtRouter;
pub use signal::UnixSignalHandler;
