//! Messenger abstraction for multi-platform bot support.

#![allow(dead_code)]

use async_trait::async_trait;

pub struct ChatId(pub String);

impl ChatId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        Self(id.to_string())
    }
}

pub struct MessageId(pub String);

impl MessageId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

impl UserId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

#[async_trait]
pub trait Messenger: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn send_message(&self, chat_id: &ChatId, text: &str) -> Result<MessageId, Self::Error>;
}

#[async_trait]
pub trait Bot: Send + Sync {
    async fn run(self: Box<Self>, shutdown: crate::domain::ShutdownSignal);
}

pub trait AuthFilter: Send + Sync {
    fn is_allowed(&self, user_id: UserId) -> bool;
}
