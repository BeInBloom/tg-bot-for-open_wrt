use async_trait::async_trait;

use super::{
    RouterError,
    ubus::{BoardInfo, HostapdClients, SystemInfo, WirelessStatus},
};

pub struct RouterStatus {
    pub system: SystemInfo,
    pub board: BoardInfo,
}

#[async_trait]
pub trait SystemInfoProvider: Send + Sync {
    async fn system_info(&self) -> Result<SystemInfo, RouterError>;
    async fn board_info(&self) -> Result<BoardInfo, RouterError>;

    async fn status(&self) -> Result<RouterStatus, RouterError> {
        let system = self.system_info().await?;
        let board = self.board_info().await?;
        Ok(RouterStatus { system, board })
    }
}

#[async_trait]
pub trait WifiInfoProvider: Send + Sync {
    async fn wireless_status(&self) -> Result<WirelessStatus, RouterError>;
    async fn wifi_clients(&self, iface: &str) -> Result<HostapdClients, RouterError>;
}

pub trait RouterInfo: SystemInfoProvider + WifiInfoProvider {}

impl<T: SystemInfoProvider + WifiInfoProvider> RouterInfo for T {}
