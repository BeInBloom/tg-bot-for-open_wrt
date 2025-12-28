//! Bot message constants.
//!
//! All user-facing strings are centralized here for:
//! - Consistency across different messengers
//! - Easy localization in the future

// ============================================================================
// Response messages
// ============================================================================

pub const PONG: &str = "pong";
pub const HELP_HEADER: &str = "Available commands:";
pub const WIFI_STATUS: &str = "WiFi Status";
pub const CLIENTS_HEADER: &str = "Connected devices";
pub const NO_DEVICES: &str = "No connected devices";
pub const ERROR_PREFIX: &str = "Error";

// ============================================================================
// Radio status
// ============================================================================

pub const RADIO_ON: &str = "ON";
pub const RADIO_OFF: &str = "OFF";
