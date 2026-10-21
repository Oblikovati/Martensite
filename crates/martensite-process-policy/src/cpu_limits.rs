//! Process-limit bookkeeping and the safe operating-system limit adapter.
use std::sync::atomic::{AtomicI64, Ordering};

/// `CPU_LIMIT`: the limit in seconds the watchdog reports (-1 for none).
static CPU_LIMIT: AtomicI64 = AtomicI64::new(0);

/// Stores the limit reported by the elapsed watchdog.
pub(crate) fn set_reported_cpu_limit(seconds: i64) {
    CPU_LIMIT.store(seconds, Ordering::Relaxed);
}

/// Read the current reported limit: initially zero, or minus one after removal.
///
/// ```
/// assert_eq!(martensite_process_policy::reported_process_limit(), 0);
/// ```
pub fn reported_cpu_limit() -> i64 {
    CPU_LIMIT.load(Ordering::Relaxed)
}

/// Applies a CPU-time cap through Rustix's checked platform interface.
#[cfg(unix)]
pub(crate) fn set_cpu_rlimit(limit: Option<u64>) -> Result<(), std::io::Error> {
    rustix::process::setrlimit(
        rustix::process::Resource::Cpu,
        rustix::process::Rlimit {
            current: limit,
            maximum: None,
        },
    )
    .map_err(std::io::Error::from)
}
