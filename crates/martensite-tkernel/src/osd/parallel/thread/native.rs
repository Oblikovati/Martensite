//! The narrow platform boundary of the thread family: thread identifiers, Windows thread
//! priority and the online processor count.
#![allow(unsafe_code)]
#[cfg(any(unix, windows))]
use std::thread::JoinHandle;

#[cfg(all(test, unix))]
mod processor_count_contracts;

/// The current thread id: `pthread_self()` on Unix, `GetCurrentThreadId()` on Windows.
#[cfg(unix)]
pub(crate) fn current_thread_id() -> usize {
    // Preserve pthread identity rather than substituting a kernel thread ID (#89).
    let thread: libc::pthread_t = nix::sys::pthread::pthread_self();
    thread as usize
}

/// The current thread id: `pthread_self()` on Unix, `GetCurrentThreadId()` on Windows.
#[cfg(windows)]
#[forbid(unsafe_code)]
pub(crate) fn current_thread_id() -> usize {
    winsafe::GetCurrentThreadId() as usize
}

/// The identifier `Run` records: the `pthread_t` on Unix, the Win32 thread id on Windows.
#[cfg(unix)]
pub(crate) fn thread_id<T>(handle: &JoinHandle<T>) -> usize {
    use std::os::unix::thread::JoinHandleExt;
    // RawPthread is u64 on Linux and already usize on macOS.
    #[allow(clippy::unnecessary_cast)]
    let thread: usize = handle.as_pthread_t() as usize;
    thread
}

/// The identifier `Run` records: the `pthread_t` on Unix, the Win32 thread id on Windows.
#[cfg(windows)]
pub(crate) fn thread_id<T>(handle: &JoinHandle<T>) -> usize {
    use std::os::windows::io::AsRawHandle;
    // SAFETY: the handle is owned by the live JoinHandle for the duration of the call.
    unsafe { windows_sys::Win32::System::Threading::GetThreadId(handle.as_raw_handle()) as usize }
}

/// `SetThreadPriority` on Windows; the pthread build only stores the priority.
#[cfg(windows)]
pub(crate) fn apply_priority<T>(handle: &JoinHandle<T>, priority: i32) {
    use std::os::windows::io::AsRawHandle;
    // SAFETY: the handle is owned by the live JoinHandle; a refused priority is ignored,
    // on purpose.
    unsafe {
        windows_sys::Win32::System::Threading::SetThreadPriority(handle.as_raw_handle(), priority);
    }
}

/// `SetThreadPriority` on Windows; the pthread build only stores the priority.
#[cfg(unix)]
pub(crate) fn apply_priority<T>(_handle: &JoinHandle<T>, _priority: i32) {}

/// `sysconf(_SC_NPROCESSORS_ONLN)`, -1 on error.
#[cfg(target_os = "linux")]
pub(crate) fn online_processors() -> i32 {
    let count = crate::native_config::native_sysconf(nix::unistd::SysconfVar::_NPROCESSORS_ONLN);
    count as i32
}

/// `sysconf(_SC_NPROCESSORS_ONLN)`, -1 on error.
#[cfg(all(unix, not(target_os = "linux")))]
pub(crate) fn online_processors() -> i32 {
    // SAFETY: sysconf only reads a configuration value.
    let count: libc::c_long = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
    count as i32
}

/// `GetSystemInfo().dwNumberOfProcessors` (the WOW64 `GetNativeSystemInfo` branch reports
/// the same count for a 64-bit Rust process).
#[cfg(windows)]
#[forbid(unsafe_code)]
pub(crate) fn online_processors() -> i32 {
    // The dependency's safe wrapper preserves the all-zero initial state (#89).
    let info = winsafe::GetSystemInfo();
    windows_processor_count(info.dwNumberOfProcessors)
}

#[cfg(windows)]
fn windows_processor_count(count: u32) -> i32 {
    count as i32
}

#[cfg(all(test, windows))]
mod windows_query_contracts;

#[cfg(all(test, windows))]
mod system_info_tests {
    #[test]
    fn native_processor_query_returns_a_positive_count() {
        assert!(super::online_processors() > 0);
    }
}

/// WebAssembly thread identity mapped deterministically from `std::thread::ThreadId`
/// without `thread_local!` (#631).
#[cfg(not(any(unix, windows)))]
pub(crate) fn current_thread_id() -> usize {
    use std::sync::{Mutex, OnceLock};
    static IDS: OnceLock<Mutex<Vec<(std::thread::ThreadId, usize)>>> = OnceLock::new();
    let current = std::thread::current().id();
    let mut table = IDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    if let Some(&(_, id)) = table.iter().find(|&&(tid, _)| tid == current) {
        return id;
    }
    let next_id = table.len() + 1;
    table.push((current, next_id));
    next_id
}

/// Browser WebAssembly logical processor count from `navigator.hardwareConcurrency`
/// (falls back to 1 when unavailable) (#631).
#[cfg(target_arch = "wasm32")]
pub(crate) fn online_processors() -> i32 {
    let pooled = super::super::platform_thread::wasm_worker_pool_size();
    if pooled > 0 {
        return i32::try_from(pooled.saturating_add(1)).unwrap_or(i32::MAX);
    }
    browser_hardware_concurrency().unwrap_or(1)
}

#[cfg(target_arch = "wasm32")]
fn browser_hardware_concurrency() -> Option<i32> {
    let global = js_sys::global();
    let nav = js_sys::Reflect::get(&global, &wasm_bindgen::JsValue::from_str("navigator")).ok()?;
    let cores = js_sys::Reflect::get(
        &nav,
        &wasm_bindgen::JsValue::from_str("hardwareConcurrency"),
    )
    .ok()?
    .as_f64()?;
    if !cores.is_finite() || cores < 1.0 {
        return None;
    }
    Some((cores.trunc() as u32).clamp(1, i32::MAX as u32) as i32)
}

#[cfg(not(any(unix, windows, target_arch = "wasm32")))]
pub(crate) fn online_processors() -> i32 {
    1
}
