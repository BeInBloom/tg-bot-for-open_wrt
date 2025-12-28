//! Bot factories for creating messenger bots from config.

use std::sync::Arc;

use crate::domain::RouterInfo;
use crate::infrastructure::Config;

use super::auth::UserWhitelist;
use super::telegram::TelegramBot;

const KEY_BOT_TOKEN: &str = "BOT_TOKEN";
const KEY_ALLOWED_USERS: &str = "BOT_ALLOWED_USERS";

pub fn create_telegram_bot<R: RouterInfo + 'static>(
    config: &Config,
    router: Arc<R>,
) -> anyhow::Result<TelegramBot<R, UserWhitelist>> {
    let token = config.required(KEY_BOT_TOKEN)?;
    let allowed_users: Vec<u64> = config.required_list(KEY_ALLOWED_USERS)?;
    let auth = UserWhitelist::from_iter(allowed_users);

    Ok(TelegramBot::new(token, router, auth))
}
