//! Router status formatters.

use crate::domain::ubus::MemoryInfo;
use crate::domain::{RouterInfo, RouterStatus};

use super::utils::{format_error, format_uptime};

const BYTES_IN_MB: u64 = 1024 * 1024;
const LOAD_DIVISOR: f64 = 100.0;

pub async fn format_status<R: RouterInfo>(router: &R) -> String {
    match router.status().await {
        Ok(status) => format_router_status(&status),
        Err(e) => format_error(&e),
    }
}

pub fn format_router_status(status: &RouterStatus) -> String {
    let header = format!(
        "[{}]\n{} | {}\n{}",
        status.board.hostname,
        status.board.release.distribution,
        status.board.release.version,
        status.board.model
    );

    let uptime = format_uptime(status.system.uptime);
    let memory = format_memory(&status.system.memory);
    let load = format_load(&status.system.load);

    format!("{header}\n\nUptime: {uptime}\n{memory}\nLoad: {load}")
}

fn format_memory(mem: &MemoryInfo) -> String {
    let used_mb = (mem.total - mem.available) / BYTES_IN_MB;
    let total_mb = mem.total / BYTES_IN_MB;
    let percent = ((mem.total - mem.available) * 100) / mem.total;
    format!("RAM: {used_mb} / {total_mb} MB ({percent}%)")
}

fn format_load(load: &[u32; 3]) -> String {
    format!(
        "{:.2} {:.2} {:.2}",
        load[0] as f64 / LOAD_DIVISOR,
        load[1] as f64 / LOAD_DIVISOR,
        load[2] as f64 / LOAD_DIVISOR
    )
}
