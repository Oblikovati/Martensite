//! Process-global CPU resource settings; geometry operations never use this state.
use std::sync::Mutex;
use std::sync::atomic::{AtomicI64, Ordering};

static REPORTED_CPU_LIMIT: AtomicI64 = AtomicI64::new(0);
static CPU_LIMIT_WRITES: Mutex<()> = Mutex::new(());

/// Read the last requested process limit, including a refused request.
///
/// ```
/// assert_eq!(martensite_tkernel::osd::reported_process_cpu_limit(), 0);
/// ```
pub fn reported_process_cpu_limit() -> i64 {
    REPORTED_CPU_LIMIT.load(Ordering::SeqCst)
}

/// Apply a process-wide CPU cap, retaining its diagnostic value on failure.
///
/// Unix installs the exit action before lowering the cap (#278). Other platforms
/// retain the requested value for a caller-owned elapsed watchdog. Concurrent
/// setters serialize; the signal action never reads the lock. This is process
/// resource configuration, not an operation-local deadline or a restoration guard.
/// ```no_run
/// martensite_tkernel::osd::set_process_cpu_limit(Some(200))?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn set_process_cpu_limit(seconds: Option<i64>) -> Result<(), std::io::Error> {
    let _transaction = CPU_LIMIT_WRITES
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    // The requested value survives OS refusal; publish before a new cap can fire.
    REPORTED_CPU_LIMIT.store(seconds.unwrap_or(-1), Ordering::SeqCst);
    apply_cpu_resource(seconds)
}

#[cfg(unix)]
fn apply_cpu_resource(seconds: Option<i64>) -> Result<(), std::io::Error> {
    super::timing::install_process_cpu_exit()?;
    rustix::process::setrlimit(
        rustix::process::Resource::Cpu,
        rustix::process::Rlimit {
            current: seconds.map(|value| value as u64),
            maximum: None,
        },
    )
    .map_err(std::io::Error::from)
}

#[cfg(not(unix))]
fn apply_cpu_resource(_seconds: Option<i64>) -> Result<(), std::io::Error> {
    Ok(())
}
