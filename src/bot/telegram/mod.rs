//! Telegram bot implementation.

mod middleware;

use std::sync::Arc;

use teloxide::prelude::*;

use crate::domain::messenger::{AuthFilter, Bot};
use crate::domain::{RouterInfo, ShutdownSignal};

use super::commands::Command;
use super::handlers;

pub struct TelegramBot<R, A>
where
    R: RouterInfo + 'static,
    A: AuthFilter + 'static,
{
    bot: teloxide::Bot,
    router: Arc<R>,
    auth: Arc<A>,
}

impl<R, A> TelegramBot<R, A>
where
    R: RouterInfo + 'static,
    A: AuthFilter + 'static,
{
    pub fn new(token: &str, router: Arc<R>, auth: A) -> Self {
        Self {
            bot: teloxide::Bot::new(token),
            router,
            auth: Arc::new(auth),
        }
    }

    fn build_handler(&self) -> teloxide::dispatching::UpdateHandler<teloxide::RequestError> {
        let auth = Arc::clone(&self.auth);

        Update::filter_message()
            .filter(middleware::logging_filter())
            .filter(middleware::auth_filter(auth))
            .filter_command::<Command>()
            .branch(dptree::case![Command::Ping].endpoint(telegram_ping))
            .branch(dptree::case![Command::Help].endpoint(telegram_help))
            .branch(dptree::case![Command::Status].endpoint(telegram_status::<R>))
            .branch(dptree::case![Command::Wifi].endpoint(telegram_wifi::<R>))
            .branch(dptree::case![Command::Clients].endpoint(telegram_clients::<R>))
    }
}

#[async_trait::async_trait]
impl<R, A> Bot for TelegramBot<R, A>
where
    R: RouterInfo + 'static,
    A: AuthFilter + 'static,
{
    async fn run(self: Box<Self>, mut shutdown: ShutdownSignal) -> anyhow::Result<()> {
        tracing::info!("Starting Telegram bot");

        let handler = self.build_handler();
        let mut dispatcher = Dispatcher::builder(self.bot, handler)
            .dependencies(dptree::deps![self.router])
            .build();

        tokio::select! {
            _ = dispatcher.dispatch() => {
                tracing::info!("Telegram dispatcher finished");
            }
            _ = shutdown.recv() => {
                tracing::info!("Telegram bot received shutdown signal");
            }
        }

        tracing::info!("Telegram bot stopped");
        Ok(())
    }
}

async fn telegram_ping(bot: teloxide::Bot, msg: Message) -> Result<(), teloxide::RequestError> {
    bot.send_message(msg.chat.id, handlers::ping_response())
        .await?;
    Ok(())
}

async fn telegram_help(bot: teloxide::Bot, msg: Message) -> Result<(), teloxide::RequestError> {
    bot.send_message(msg.chat.id, handlers::help_response())
        .await?;
    Ok(())
}

async fn telegram_status<R: RouterInfo>(
    bot: teloxide::Bot,
    msg: Message,
    router: Arc<R>,
) -> Result<(), teloxide::RequestError> {
    let response = handlers::status_response(router.as_ref()).await;
    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}

async fn telegram_wifi<R: RouterInfo>(
    bot: teloxide::Bot,
    msg: Message,
    router: Arc<R>,
) -> Result<(), teloxide::RequestError> {
    let response = handlers::wifi_response(router.as_ref()).await;
    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}

async fn telegram_clients<R: RouterInfo>(
    bot: teloxide::Bot,
    msg: Message,
    router: Arc<R>,
) -> Result<(), teloxide::RequestError> {
    let response = handlers::clients_response(router.as_ref()).await;
    bot.send_message(msg.chat.id, response).await?;
    Ok(())
}
