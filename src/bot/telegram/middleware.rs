//! Telegram middleware filters.

use std::sync::Arc;

use teloxide::prelude::*;

use crate::domain::messenger::{AuthFilter, UserId};

pub fn auth_filter<A: AuthFilter + 'static>(auth: Arc<A>) -> impl Fn(Message) -> bool + Clone {
    move |msg: Message| {
        msg.from
            .as_ref()
            .map(|u| auth.is_allowed(UserId::new(u.id.0)))
            .unwrap_or(false)
    }
}

pub fn logging_filter() -> impl Fn(Message) -> bool + Clone {
    |msg: Message| {
        if let Some(user) = &msg.from {
            let username = user.username.as_deref().unwrap_or("unknown");
            let text = msg.text().unwrap_or("[no text]");
            tracing::info!(user_id = user.id.0, username, "Command: {text}");
        }
        true
    }
}
