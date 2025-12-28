//! OpenWRT router via ubus.

use std::process::Output;

use async_trait::async_trait;
use tokio::process::Command;

use crate::domain::{
    RouterError,
    router::{SystemInfoProvider, WifiInfoProvider},
    ubus::{BoardInfo, HostapdClients, SystemInfo, WirelessStatus},
};

pub struct OpenWrtRouter;

impl OpenWrtRouter {
    pub fn new() -> Self {
        Self
    }

    async fn ubus_call<T: serde::de::DeserializeOwned>(
        &self,
        service: &str,
        method: &str,
    ) -> Result<T, RouterError> {
        let output = self.execute_ubus(service, method).await?;
        self.check_success(&output)?;
        self.parse_response(&output)
    }

    async fn execute_ubus(&self, service: &str, method: &str) -> Result<Output, RouterError> {
        Command::new("ubus")
            .args(["call", service, method])
            .output()
            .await
            .map_err(|source| RouterError::Spawn {
                cmd: "ubus",
                source,
            })
    }

    fn check_success(&self, output: &Output) -> Result<(), RouterError> {
        if output.status.success() {
            Ok(())
        } else {
            Err(RouterError::NonZeroExit {
                cmd: "ubus",
                code: output.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }

    fn parse_response<T: serde::de::DeserializeOwned>(
        &self,
        output: &Output,
    ) -> Result<T, RouterError> {
        serde_json::from_slice(&output.stdout).map_err(|source| RouterError::Json {
            cmd: "ubus",
            source,
        })
    }
}

impl Default for OpenWrtRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SystemInfoProvider for OpenWrtRouter {
    async fn system_info(&self) -> Result<SystemInfo, RouterError> {
        self.ubus_call("system", "info").await
    }

    async fn board_info(&self) -> Result<BoardInfo, RouterError> {
        self.ubus_call("system", "board").await
    }
}

#[async_trait]
impl WifiInfoProvider for OpenWrtRouter {
    async fn wireless_status(&self) -> Result<WirelessStatus, RouterError> {
        self.ubus_call("network.wireless", "status").await
    }

    async fn wifi_clients(&self, iface: &str) -> Result<HostapdClients, RouterError> {
        self.ubus_call(&format!("hostapd.{iface}"), "get_clients")
            .await
    }
}
