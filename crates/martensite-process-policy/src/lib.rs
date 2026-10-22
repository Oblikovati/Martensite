//! Process-wide CPU limits and an elapsed watchdog for dedicated processes.
//!
//! This shared adapter changes process-global state. It supplies no operation-local
//! deadline, RAII restoration, or concurrent per-case budgeting.
mod cpu_limits;
mod process_limit;

#[cfg(feature = "case-runner")]
pub mod case_runner;

pub use cpu_limits::reported_cpu_limit as reported_process_limit;
pub use process_limit::{ProcessLimitError, apply_process_limit, set_process_limit};
