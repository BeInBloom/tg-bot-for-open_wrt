//! Universal command handlers.
//!
//! Platform-agnostic handlers that return response strings.

use teloxide::utils::command::BotCommands;

use crate::domain::RouterInfo;

use super::commands::Command;
use super::formatters::{format_status, format_wifi_clients, format_wifi_status};
use super::messages::{HELP_HEADER, PONG};

pub fn ping_response() -> String {
    PONG.to_string()
}

pub fn help_response() -> String {
    format!("{HELP_HEADER}\n{}", Command::descriptions())
}

pub async fn status_response<R: RouterInfo>(router: &R) -> String {
    format_status(router).await
}

pub async fn wifi_response<R: RouterInfo>(router: &R) -> String {
    format_wifi_status(router).await
}

pub async fn clients_response<R: RouterInfo>(router: &R) -> String {
    format_wifi_clients(router).await
}
