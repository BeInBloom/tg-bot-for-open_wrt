//! Bot commands definition.
//!
//! Defines available commands using Teloxide's BotCommands derive macro.

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Ping,
    Status,
    Wifi,
    Clients,
    Help,
}
