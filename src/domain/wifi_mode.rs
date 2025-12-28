//! WiFi mode detection.

use crate::domain::ubus::WifiClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiMode {
    Wifi6,
    Wifi5,
    Wifi4,
    Legacy,
}

impl WifiMode {
    pub fn from_client(client: &WifiClient) -> Self {
        match (client.he, client.vht, client.ht) {
            (true, _, _) => Self::Wifi6,
            (_, true, _) => Self::Wifi5,
            (_, _, true) => Self::Wifi4,
            _ => Self::Legacy,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Wifi6 => "WiFi 6",
            Self::Wifi5 => "WiFi 5",
            Self::Wifi4 => "WiFi 4",
            Self::Legacy => "Legacy",
        }
    }
}

impl std::fmt::Display for WifiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
