mod bot;
mod core;
mod domain;
mod infrastructure;

use core::App;
use infrastructure::Config;

const CONFIG_PATH: &str = r#"./config"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Загружаем конфигурацию из файла
    dotenvy::from_path(CONFIG_PATH)?;

    // Создаем конфигурацию из environment variables
    let config = Config::new();

    // Создаем и запускаем приложение
    let app = App::new(&config)?;
    app.run().await?;

    Ok(())
}
