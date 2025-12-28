//! Message formatters for bot responses.
//!
//! Pure functions that format router data into human-readable strings.
//! These formatters are platform-agnostic and can be used with any messenger.

mod clients;
mod status;
mod utils;
mod wifi;

pub use clients::format_wifi_clients;
pub use status::format_status;
pub use wifi::format_wifi_status;
