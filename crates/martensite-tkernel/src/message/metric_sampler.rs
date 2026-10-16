//! The clock and memory seam behind alert metric sampling.
//!
//! Metrics come from the wall clock, the process and thread CPU times and the
//! memory counters; a report samples through this trait, so tests
//! inject deterministic values. The host sampler delegates to the OSD types, which own
//! the platform code.
use crate::osd::{CpuTimes, MemCounter, MemInfo, NativeCpuTimes, NativeWallClock, WallClock};

/// Source of the metric values stored in alert meters; inject a named fake in tests.
pub trait MetricSampler: Send + Sync {
    /// Seconds since the first call; `sampler.wall_clock_time()`.
    fn wall_clock_time(&self) -> f64;
    /// Process CPU `(user, system)` seconds; `sampler.process_cpu()`.
    fn process_cpu(&self) -> (f64, f64);
    /// Thread CPU `(user, system)` seconds; `sampler.thread_cpu()`.
    fn thread_cpu(&self) -> (f64, f64);
    /// Memory statistics with exactly `counters` active, updated once, read as
    /// precise MiB (-1.0 when unavailable); `sampler.memory_mib(&[MemCounter::Virtual])`.
    fn memory_mib(&self, counters: &[MemCounter]) -> Vec<f64>;
}

/// The host clocks and process memory statistics through `osd::NativeWallClock`,
/// `osd::NativeCpuTimes` and `osd::MemInfo`; `Arc::new(NativeMetricSampler)`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeMetricSampler;
impl MetricSampler for NativeMetricSampler {
    fn wall_clock_time(&self) -> f64 {
        NativeWallClock.now()
    }
    fn process_cpu(&self) -> (f64, f64) {
        NativeCpuTimes.process_cpu()
    }
    fn thread_cpu(&self) -> (f64, f64) {
        NativeCpuTimes.thread_cpu()
    }
    fn memory_mib(&self, counters: &[MemCounter]) -> Vec<f64> {
        // The meter builds inactive memory statistics, activates the requested
        // counters and updates once.
        let mut info: MemInfo = MemInfo::new(false);
        info.set_active(false);
        for &counter in counters {
            info.set_counter_active(counter, true);
        }
        info.update();
        counters
            .iter()
            .map(|&counter| info.value_precise_mib(counter))
            .collect()
    }
}
