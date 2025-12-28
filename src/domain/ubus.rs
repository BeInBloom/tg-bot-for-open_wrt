//! OpenWRT ubus response types.

#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SystemInfo {
    pub localtime: u64,
    pub uptime: u64,
    pub load: [u32; 3],
    pub memory: MemoryInfo,
    pub root: StorageInfo,
    pub tmp: StorageInfo,
    pub swap: SwapInfo,
}

#[derive(Debug, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub free: u64,
    pub shared: u64,
    pub buffered: u64,
    pub available: u64,
    pub cached: u64,
}

#[derive(Debug, Deserialize)]
pub struct StorageInfo {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub avail: u64,
}

#[derive(Debug, Deserialize)]
pub struct SwapInfo {
    pub total: u64,
    pub free: u64,
}

#[derive(Debug, Deserialize)]
pub struct BoardInfo {
    pub kernel: String,
    pub hostname: String,
    pub system: String,
    pub model: String,
    pub board_name: String,
    pub rootfs_type: String,
    pub release: ReleaseInfo,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseInfo {
    pub distribution: String,
    pub version: String,
    pub revision: String,
    pub target: String,
    pub description: String,
    pub builddate: String,
}

#[derive(Debug, Deserialize)]
pub struct WirelessStatus(pub HashMap<String, RadioInfo>);

#[derive(Debug, Deserialize)]
pub struct RadioInfo {
    pub up: bool,
    pub disabled: bool,
    pub config: RadioConfig,
    pub interfaces: Vec<WifiInterface>,
}

#[derive(Debug, Deserialize)]
pub struct RadioConfig {
    pub band: String,
    pub channel: String,
    pub htmode: String,
}

#[derive(Debug, Deserialize)]
pub struct WifiInterface {
    pub section: String,
    pub ifname: String,
    pub config: WifiInterfaceConfig,
    #[serde(default)]
    pub stations: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct WifiInterfaceConfig {
    pub ssid: String,
    pub encryption: String,
}

#[derive(Debug, Deserialize)]
pub struct HostapdClients {
    pub freq: u32,
    pub clients: HashMap<String, WifiClient>,
}

#[derive(Debug, Deserialize)]
pub struct WifiClient {
    pub auth: bool,
    pub assoc: bool,
    pub authorized: bool,
    pub signal: i32,
    pub ht: bool,
    pub vht: bool,
    pub he: bool,
    pub bytes: ClientTraffic,
    pub rate: ClientRate,
}

#[derive(Debug, Deserialize)]
pub struct ClientTraffic {
    pub rx: u64,
    pub tx: u64,
}

#[derive(Debug, Deserialize)]
pub struct ClientRate {
    pub rx: u64,
    pub tx: u64,
}
