//! WiFi clients formatters.

use crate::domain::RouterInfo;
use crate::domain::ubus::{WifiClient, WifiInterface, WirelessStatus};

use super::super::messages::{CLIENTS_HEADER, NO_DEVICES};
use super::utils::{format_error, format_speed, wifi_mode};

pub async fn format_wifi_clients<R: RouterInfo>(router: &R) -> String {
    let wireless = match router.wireless_status().await {
        Ok(w) => w,
        Err(e) => return format_error(&e),
    };

    let (lines, total) = collect_clients_info(router, &wireless).await;
    format_clients_output(lines, total)
}

pub async fn collect_clients_info<R: RouterInfo>(
    router: &R,
    wireless: &WirelessStatus,
) -> (Vec<String>, usize) {
    let mut lines = Vec::new();
    let mut total_clients = 0;

    for radio in wireless.0.values() {
        for iface in &radio.interfaces {
            if let Some((iface_lines, count)) =
                collect_interface_clients(router, iface, &radio.config.band).await
            {
                lines.extend(iface_lines);
                total_clients += count;
            }
        }
    }

    (lines, total_clients)
}

async fn collect_interface_clients<R: RouterInfo>(
    router: &R,
    iface: &WifiInterface,
    band: &str,
) -> Option<(Vec<String>, usize)> {
    let clients_info = router.wifi_clients(&iface.ifname).await.ok()?;
    let count = clients_info.clients.len();

    if count == 0 {
        return None;
    }

    let ssid = &iface.config.ssid;
    let mut lines = vec![format!("\n{ssid} ({band}) - {count} devices")];

    for (mac, client) in &clients_info.clients {
        lines.push(format_client_info(mac, client));
    }

    Some((lines, count))
}

pub fn format_client_info(mac: &str, client: &WifiClient) -> String {
    let speed = format_speed(client.rate.tx);
    let mode = wifi_mode(client);
    format!("\n  {mac}\n    {speed} | {mode} | {}dBm", client.signal)
}

pub fn format_clients_output(mut lines: Vec<String>, total: usize) -> String {
    let mut result = vec![CLIENTS_HEADER.to_string()];

    if total == 0 {
        result.push(format!("\n\n{NO_DEVICES}"));
    } else {
        result.push(format!("\nTotal: {total}"));
        result.append(&mut lines);
    }

    result.join("")
}
