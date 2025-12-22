use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use tracing_appender::non_blocking;

use super::config::Config;

const KEY_LOG_DIR: &str = r#"BOT_LOG_DIR"#;
const KEY_LOG_FILTER: &str = r#"BOT_LOG"#;
const KEY_LOG_ANSI: &str = r#"BOT_LOG_ANSI"#;
const KEY_LOG_TARGET: &str = r#"BOT_LOG_TARGET"#;

const DEFAULT_LOG_FILTER: &str = r#"info"#;
const DEFAULT_EXE_PARENT_DIR: &str = r#"."#;
const LOG_FILE_NAME: &str = r#"log"#;

const DEFAULT_LOG_ANSI: bool = false;
const DEFAULT_LOG_TARGET: bool = true;

const ERR_CREATE_LOG_DIR: &str = r#"failed to create log dir"#;
const ERR_INIT_TRACING: &str = r#"failed to initialize tracing subscriber"#;

#[must_use = "LogGuard must be held to keep logging active"]
pub struct LogGuard {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

fn exe_dir() -> anyhow::Result<PathBuf> {
    let exe = std::env::current_exe()?;

    Ok(exe
        .parent()
        .unwrap_or_else(|| Path::new(DEFAULT_EXE_PARENT_DIR))
        .to_path_buf())
}

fn log_dir(conf: &Config) -> anyhow::Result<PathBuf> {
    conf.optional(KEY_LOG_DIR)
        .map(PathBuf::from)
        .map_or_else(exe_dir, Ok)
        .and_then(|dir| {
            fs::create_dir_all(&dir).context(ERR_CREATE_LOG_DIR)?;
            Ok(dir)
        })
}

pub fn init(conf: &Config) -> anyhow::Result<LogGuard> {
    let dir = log_dir(conf)?;
    let (writer, guard) = build_file_writer(dir);
    let filter = resolve_filter(conf);
    let ansi = resolve_log_ansi(conf);
    let target = resolve_log_target(conf);
    init_compact_subscriber(filter, writer, ansi, target)?;
    Ok(LogGuard { _guard: guard })
}

fn build_file_writer(dir: PathBuf) -> (non_blocking::NonBlocking, non_blocking::WorkerGuard) {
    let appender = tracing_appender::rolling::never(dir, LOG_FILE_NAME);
    non_blocking(appender)
}

fn resolve_filter(conf: &Config) -> &str {
    conf.optional(KEY_LOG_FILTER).unwrap_or(DEFAULT_LOG_FILTER)
}

fn resolve_log_ansi(conf: &Config) -> bool {
    conf.parse_optional(KEY_LOG_ANSI)
        .unwrap_or(DEFAULT_LOG_ANSI)
}

fn resolve_log_target(conf: &Config) -> bool {
    conf.parse_optional(KEY_LOG_TARGET)
        .unwrap_or(DEFAULT_LOG_TARGET)
}

fn init_compact_subscriber(
    filter: &str,
    writer: non_blocking::NonBlocking,
    ansi: bool,
    target: bool,
) -> anyhow::Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::new(filter);

    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .with_ansi(ansi)
        .with_target(target)
        .with_writer(writer)
        .try_init()
        .map_err(|e| anyhow::anyhow!("{ERR_INIT_TRACING}: {e}"))
}
