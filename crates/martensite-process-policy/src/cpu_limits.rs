//! One foundation-owned process resource record supplies CPU and elapsed diagnostics.

/// Read the requested limit, initially zero and minus one after removal.
///
/// ```
/// assert_eq!(martensite_process_policy::reported_process_limit(), 0);
/// ```
pub fn reported_cpu_limit() -> i64 {
    martensite_tkernel::osd::reported_process_cpu_limit()
}

/// Install the signal action before a CPU cap can expire; keep all requests serialized.
pub(crate) fn set_cpu_rlimit(limit: Option<i64>) -> Result<(), std::io::Error> {
    martensite_tkernel::osd::set_process_cpu_limit(limit)
}
