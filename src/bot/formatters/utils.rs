//! Utility formatters.

use std::time::Duration;

use crate::domain::ubus::WifiClient;
use crate::domain::{RouterError, WifiMode};

const BITS_PER_MBPS: f64 = 1_000_000.0;
const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 3600;
const SECONDS_IN_DAY: u64 = 86400;

pub fn format_speed(bps: u64) -> String {
    let mbps = bps as f64 / BITS_PER_MBPS;
    format!("{mbps:.0} Mbps")
}

pub fn wifi_mode(client: &WifiClient) -> &'static str {
    WifiMode::from_client(client).as_str()
}

pub fn format_uptime(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);
    let days = duration.as_secs() / SECONDS_IN_DAY;
    let hours = (duration.as_secs() % SECONDS_IN_DAY) / SECONDS_IN_HOUR;
    let minutes = (duration.as_secs() % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE;

    match (days, hours) {
        (d, _) if d > 0 => format!("{days}d {hours}h {minutes}m"),
        (_, h) if h > 0 => format!("{hours}h {minutes}m"),
        _ => format!("{minutes}m"),
    }
}

pub fn format_error(error: &RouterError) -> String {
    format!("{}: {error}", super::super::messages::ERROR_PREFIX)
}
