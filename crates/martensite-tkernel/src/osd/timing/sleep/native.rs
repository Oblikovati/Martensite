//! Thin platform boundary for the OS `usleep` and `Sleep` calls.
#![allow(unsafe_code)]

#[cfg(all(test, unix))]
mod contracts;

#[cfg(any(windows, test))]
mod windows_milliseconds;

#[cfg(all(test, windows))]
mod windows_contracts;

#[cfg(unix)]
pub(super) fn sec_sleep(seconds: i32) {
    let microseconds = seconds.wrapping_mul(1_000).wrapping_mul(1_000) as libc::useconds_t;
    // SAFETY: usleep receives only a scalar duration and touches no caller memory.
    let _ = unsafe { libc::usleep(microseconds) };
}

#[cfg(unix)]
pub(super) fn milli_sec_sleep(milliseconds: i32) {
    let microseconds = milliseconds.wrapping_mul(1_000) as libc::useconds_t;
    // SAFETY: usleep receives only a scalar duration and touches no caller memory.
    let _ = unsafe { libc::usleep(microseconds) };
}

#[cfg(windows)]
pub(super) fn sec_sleep(seconds: i32) {
    let milliseconds = windows_milliseconds::from_seconds(seconds);
    sleep_windows(milliseconds);
}

#[cfg(windows)]
pub(super) fn milli_sec_sleep(milliseconds: i32) {
    sleep_windows(windows_milliseconds::from_milliseconds(milliseconds));
}

#[cfg(all(not(unix), not(windows)))]
pub(super) fn sec_sleep(seconds: i32) {
    std::thread::sleep(super::sec_sleep_duration(seconds));
}

#[cfg(all(not(unix), not(windows)))]
pub(super) fn milli_sec_sleep(milliseconds: i32) {
    std::thread::sleep(super::milli_sec_sleep_duration(milliseconds));
}

#[cfg(windows)]
#[forbid(unsafe_code)]
fn sleep_windows(milliseconds: u32) {
    // Preserve zero yielding and the native infinite sentinel without conversion (#89).
    winsafe::Sleep(milliseconds);
}
