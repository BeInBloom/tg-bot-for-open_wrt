use std::path::{Path, PathBuf};

use anyhow::Context;
use tracing_appender::non_blocking;

use super::config::Config;
use super::fs::FileSystem;

const KEY_LOG_DIR: &str = "BOT_LOG_DIR";
const KEY_LOG_FILTER: &str = "BOT_LOG";
const KEY_LOG_ANSI: &str = "BOT_LOG_ANSI";
const KEY_LOG_TARGET: &str = "BOT_LOG_TARGET";

const DEFAULT_LOG_FILTER: &str = "info";
const LOG_FILE_NAME: &str = "log";

#[must_use = "LogGuard must be held to keep logging active"]
pub struct LogGuard {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

pub fn init_with_fs<F: FileSystem>(conf: &Config, fs: &F) -> anyhow::Result<LogGuard> {
    let dir = log_dir(conf, fs)?;
    let (writer, guard) = build_file_writer(dir);
    let filter = conf.optional(KEY_LOG_FILTER).unwrap_or(DEFAULT_LOG_FILTER);
    let ansi = conf.parse_optional(KEY_LOG_ANSI).unwrap_or(false);
    let target = conf.parse_optional(KEY_LOG_TARGET).unwrap_or(true);
    init_subscriber(filter, writer, ansi, target)?;
    Ok(LogGuard { _guard: guard })
}

pub fn init(conf: &Config) -> anyhow::Result<LogGuard> {
    init_with_fs(conf, &super::fs::RealFileSystem)
}

fn log_dir<F: FileSystem>(conf: &Config, fs: &F) -> anyhow::Result<PathBuf> {
    let dir = match conf.optional(KEY_LOG_DIR) {
        Some(path) => PathBuf::from(path),
        None => fs
            .current_exe()?
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf(),
    };

    fs.create_dir_all(&dir)
        .context("failed to create log dir")?;
    Ok(dir)
}

fn build_file_writer(dir: PathBuf) -> (non_blocking::NonBlocking, non_blocking::WorkerGuard) {
    let appender = tracing_appender::rolling::never(dir, LOG_FILE_NAME);
    non_blocking(appender)
}

fn init_subscriber(
    filter: &str,
    writer: non_blocking::NonBlocking,
    ansi: bool,
    target: bool,
) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(tracing_subscriber::EnvFilter::new(filter))
        .with_ansi(ansi)
        .with_target(target)
        .with_writer(writer)
        .try_init()
        .map_err(|e| anyhow::anyhow!("failed to initialize tracing: {e}"))
}
