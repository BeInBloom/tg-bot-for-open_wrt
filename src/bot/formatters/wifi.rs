//! WiFi status formatters.

use crate::domain::RouterInfo;
use crate::domain::ubus::{RadioInfo, WirelessStatus};

use super::super::messages::{RADIO_OFF, RADIO_ON, WIFI_STATUS};
use super::utils::format_error;

pub async fn format_wifi_status<R: RouterInfo>(router: &R) -> String {
    match router.wireless_status().await {
        Ok(wireless) => format_wireless_status(&wireless),
        Err(e) => format_error(&e),
    }
}

pub fn format_wireless_status(wireless: &WirelessStatus) -> String {
    let mut lines = vec![WIFI_STATUS.to_string()];

    for (name, radio) in &wireless.0 {
        lines.push(format_radio_status(name, radio));
    }

    lines.join("")
}

fn format_radio_status(name: &str, radio: &RadioInfo) -> String {
    let status = if radio.up && !radio.disabled {
        RADIO_ON
    } else {
        RADIO_OFF
    };
    let band = &radio.config.band;
    let channel = &radio.config.channel;

    let mut result = String::new();
    for iface in &radio.interfaces {
        let ssid = &iface.config.ssid;
        result.push_str(&format!(
            "\n[{status}] {ssid} ({band})\n    Radio: {name} | Channel: {channel}"
        ));
    }

    result
}
