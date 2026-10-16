//! The clocks the timing classes read, behind project-owned traits so that tests inject
//! named fakes; the `Native*` implementations are the platform calls.
use super::{cpu_native, wall_native};

/// CPU-time source of `Chronometer`; implement for deterministic tests.
pub trait CpuTimes: Send + Sync {
    /// (user, system) seconds of the process; `times.process_cpu()`.
    fn process_cpu(&self) -> (f64, f64);
    /// (user, system) seconds of the calling thread; `times.thread_cpu()`.
    fn thread_cpu(&self) -> (f64, f64);
}

/// The host CPU clocks: Linux `times()`/`CLOCK_THREAD_CPUTIME_ID` (thread system time
/// is 0), macOS `times()`/`TASK_THREAD_TIMES_INFO`, Windows `GetProcessTimes`/
/// Example: `NativeCpuTimes.process_cpu()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeCpuTimes;
impl CpuTimes for NativeCpuTimes {
    fn process_cpu(&self) -> (f64, f64) {
        cpu_native::process_cpu()
    }
    fn thread_cpu(&self) -> (f64, f64) {
        cpu_native::thread_cpu()
    }
}

/// Wall-clock source of `Timer`; implement for deterministic tests.
pub trait WallClock: Send + Sync {
    /// Seconds on a monotonic-in-practice scale; `clock.now()`.
    fn now(&self) -> f64;
}

/// The host wall clock: Unix `gettimeofday` relative to the first call's second,
/// Windows `QueryPerformanceCounter`; `NativeWallClock.now()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeWallClock;
impl WallClock for NativeWallClock {
    fn now(&self) -> f64 {
        wall_native::wall_clock_seconds()
    }
}
