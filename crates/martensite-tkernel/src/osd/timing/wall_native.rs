//! Narrow platform FFI boundary for the wall clock.
#![allow(unsafe_code)]

#[cfg(all(test, windows))]
mod windows_contracts;

#[cfg(any(windows, test))]
mod windows_clock;

/// Unix: `gettimeofday` seconds since the first call (a function-local static
/// keeps `tv_sec` of its first call to avoid precision loss), 0 when the call fails.
#[cfg(unix)]
pub(super) fn wall_clock_seconds() -> f64 {
    use std::sync::OnceLock;
    static START_SECONDS: OnceLock<i64> = OnceLock::new();
    let start: i64 = *START_SECONDS.get_or_init(|| time_of_day().map_or(0, |(sec, _)| sec));
    time_of_day().map_or(0.0, |(sec, usec)| {
        (sec - start) as f64 + 0.000_001 * usec as f64
    })
}

#[cfg(unix)]
// time_t and suseconds_t are narrower than i64 on some Unix targets (macOS tv_usec).
#[allow(clippy::unnecessary_cast)]
fn time_of_day() -> Option<(i64, i64)> {
    let mut time: libc::timeval = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    // SAFETY: `time` is initialized and writable; a null timezone pointer is allowed.
    let status: i32 = unsafe { libc::gettimeofday(&mut time, std::ptr::null_mut()) };
    (status == 0).then_some((time.tv_sec as i64, time.tv_usec as i64))
}

/// Windows: `QueryPerformanceCounter` over the frequency read on the first call, or
/// `0.001 * GetTickCount64()` when the performance counter is unavailable.
#[cfg(windows)]
pub(super) fn wall_clock_seconds() -> f64 {
    use std::sync::OnceLock;
    static FREQUENCY: OnceLock<Option<i64>> = OnceLock::new();
    windows_clock::seconds(&FREQUENCY, &mut NativeWindowsClock)
}

#[cfg(windows)]
struct NativeWindowsClock;

#[cfg(windows)]
impl windows_clock::ClockQueries for NativeWindowsClock {
    fn frequency(&mut self) -> Option<i64> {
        use windows_sys::Win32::System::Performance::QueryPerformanceFrequency;
        let mut value: i64 = 0;
        // SAFETY: `value` is a writable i64 alive for the call.
        (unsafe { QueryPerformanceFrequency(&mut value) } != 0).then_some(value)
    }

    fn counter(&mut self) -> Option<i64> {
        use windows_sys::Win32::System::Performance::QueryPerformanceCounter;
        let mut counter: i64 = 0;
        // SAFETY: `counter` is a writable i64 alive for the call.
        (unsafe { QueryPerformanceCounter(&mut counter) } != 0).then_some(counter)
    }

    fn ticks(&mut self) -> u64 {
        use windows_sys::Win32::System::SystemInformation::GetTickCount64;
        // SAFETY: GetTickCount64 has no preconditions.
        unsafe { GetTickCount64() }
    }
}
