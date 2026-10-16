//! The perf-meter stopwatch clock: thread CPU user seconds.
use super::CpuClock;
use crate::osd::{CpuTimes, NativeCpuTimes};
/// Host clock with platform-specific accounting; `NativeCpuClock.now()`.
#[derive(Clone, Copy, Default)]
pub struct NativeCpuClock;
impl CpuClock for NativeCpuClock {
    fn now(&self) -> f64 {
        // The stopwatch keeps the user part and drops the system part.
        NativeCpuTimes.thread_cpu().0
    }
}
