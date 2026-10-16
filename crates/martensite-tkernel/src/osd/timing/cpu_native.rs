//! Narrow platform FFI boundary for the process and thread CPU times.
//! Each function returns (user seconds, system seconds).
#![allow(unsafe_code)]

/// Linux (and every non-Apple Unix): `times()` ticks over `sysconf(_SC_CLK_TCK)`.
#[cfg(all(unix, not(target_vendor = "apple")))]
pub(super) fn process_cpu() -> (f64, f64) {
    tms_seconds(clock_ticks())
}

/// macOS: `times()` ticks. The tick unit comes
/// from `sysconf(_SC_CLK_TCK)`, the unit `times()` reports on Darwin.
#[cfg(target_vendor = "apple")]
pub(super) fn process_cpu() -> (f64, f64) {
    tms_seconds(clock_ticks())
}

#[cfg(unix)]
fn clock_ticks() -> f64 {
    use nix::{
        errno::Errno,
        unistd::{SysconfVar, sysconf},
    };
    let previous_errno = Errno::last_raw();
    let queried = sysconf(SysconfVar::CLK_TCK);
    // nix clears errno; preserve the native successful-query contract (#89).
    if Errno::last_raw() == 0 {
        Errno::set_raw(previous_errno);
    }
    let ticks: libc::c_long = queried.ok().flatten().unwrap_or(-1);
    ticks as f64
}

#[cfg(all(test, unix))]
mod clock_tick_contracts;

#[cfg(unix)]
fn tms_seconds(ticks: f64) -> (f64, f64) {
    let mut current: libc::tms = libc::tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    // SAFETY: `current` is initialized, aligned and writable for the whole call; like the
    // the (clock_t) return value is ignored on purpose.
    unsafe { libc::times(&mut current) };
    (
        current.tms_utime as f64 / ticks,
        current.tms_stime as f64 / ticks,
    )
}

/// Linux: only the user time of a thread is available (`CLOCK_THREAD_CPUTIME_ID`);
/// the system part stays 0, and a failing call leaves both at 0.
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
pub(super) fn thread_cpu() -> (f64, f64) {
    use rustix::time::{ClockId, DynamicClockId, clock_gettime_dynamic};
    // The fallible interface preserves the existing zero-on-error contract (#89).
    let stamp = clock_gettime_dynamic(DynamicClockId::Known(ClockId::ThreadCPUTime));
    thread_cpu_seconds(
        stamp
            .ok()
            .map(|stamp| (stamp.tv_sec as f64, stamp.tv_nsec as f64)),
    )
}

#[cfg(all(
    unix,
    not(target_vendor = "apple"),
    not(all(target_os = "linux", target_pointer_width = "64"))
))]
pub(super) fn thread_cpu() -> (f64, f64) {
    let mut stamp: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: stamp is initialized, correctly aligned, writable, and alive throughout the call.
    let status: i32 = unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut stamp) };
    thread_cpu_seconds((status == 0).then(|| (stamp.tv_sec as f64, stamp.tv_nsec as f64)))
}

#[cfg(all(unix, not(target_vendor = "apple")))]
fn thread_cpu_seconds(stamp: Option<(f64, f64)>) -> (f64, f64) {
    let Some((seconds, nanoseconds)) = stamp else {
        return (0.0, 0.0);
    };
    (seconds + 0.000_000_001 * nanoseconds, 0.0)
}

#[cfg(all(test, unix, not(target_vendor = "apple")))]
mod thread_clock_contracts;

/// macOS: the task's accumulated live-thread times (`TASK_THREAD_TIMES_INFO`).
#[cfg(target_vendor = "apple")]
pub(super) fn thread_cpu() -> (f64, f64) {
    use mach2::{
        task::task_info,
        task_info::{TASK_THREAD_TIMES_INFO, TASK_THREAD_TIMES_INFO_COUNT, task_thread_times_info},
        traps::mach_task_self,
    };
    let mut info: task_thread_times_info = task_thread_times_info::default();
    let mut count: u32 = TASK_THREAD_TIMES_INFO_COUNT;
    // SAFETY: the flavor/count match the initialized output layout; the task port belongs to this process.
    let status: i32 = unsafe {
        task_info(
            mach_task_self(),
            TASK_THREAD_TIMES_INFO,
            (&mut info as *mut task_thread_times_info).cast(),
            &mut count,
        )
    };
    if status != 0 {
        return (0.0, 0.0);
    }
    (
        f64::from(info.user_time.seconds) + 0.000_001 * f64::from(info.user_time.microseconds),
        f64::from(info.system_time.seconds) + 0.000_001 * f64::from(info.system_time.microseconds),
    )
}

#[cfg(windows)]
fn filetime_seconds(time: windows_sys::Win32::Foundation::FILETIME) -> f64 {
    let ticks: u64 = (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime);
    // FILETIME spans are converted to a signed 64-bit count; CPU spans never reach bit 63.
    0.000_000_1 * ticks as i64 as f64
}

#[cfg(windows)]
type TimesQuery = unsafe extern "system" fn(
    windows_sys::Win32::Foundation::HANDLE,
    *mut windows_sys::Win32::Foundation::FILETIME,
    *mut windows_sys::Win32::Foundation::FILETIME,
    *mut windows_sys::Win32::Foundation::FILETIME,
    *mut windows_sys::Win32::Foundation::FILETIME,
) -> windows_sys::core::BOOL;

/// `GetProcessTimes`/`GetThreadTimes` on the current object; the
/// status is ignored, so a failure reads the zero-initialized times.
#[cfg(windows)]
fn windows_times(query: TimesQuery, object: windows_sys::Win32::Foundation::HANDLE) -> (f64, f64) {
    use windows_sys::Win32::Foundation::FILETIME;
    let zero: FILETIME = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let (mut start, mut exit, mut kernel, mut user) = (zero, zero, zero, zero);
    // SAFETY: the pseudo-handle of the current process/thread needs no closing; all four
    // outputs are distinct initialized FILETIMEs.
    unsafe { query(object, &mut start, &mut exit, &mut kernel, &mut user) };
    (filetime_seconds(user), filetime_seconds(kernel))
}

#[cfg(windows)]
pub(super) fn process_cpu() -> (f64, f64) {
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, GetProcessTimes};
    // SAFETY: GetCurrentProcess returns a constant pseudo-handle.
    windows_times(GetProcessTimes, unsafe { GetCurrentProcess() })
}

#[cfg(windows)]
pub(super) fn thread_cpu() -> (f64, f64) {
    use windows_sys::Win32::System::Threading::{GetCurrentThread, GetThreadTimes};
    // SAFETY: GetCurrentThread returns a constant pseudo-handle.
    windows_times(GetThreadTimes, unsafe { GetCurrentThread() })
}
