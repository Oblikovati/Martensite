//! The real process behind `Process`: the narrow FFI boundary of the Unix branch
//! (`getuid`, `getpwuid_r`, `localtime_r`) and the standard library elsewhere.
#![allow(unsafe_code)]
use super::{LocalTime, ProcessSource};
use std::io;

/// Queries the running process; `NativeProcess.process_id()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeProcess;

fn os_error(error: io::Error) -> i32 {
    error.raw_os_error().unwrap_or(0)
}

impl ProcessSource for NativeProcess {
    fn process_id(&self) -> i32 {
        std::process::id() as i32
    }
    fn is_super_user(&self) -> bool {
        is_super_user().unwrap_or(false)
    }
    fn is_super_user_checked(&self) -> Result<bool, i32> {
        is_super_user()
    }
    fn user_name(&self) -> Result<Vec<u8>, i32> {
        user_name()
    }
    fn local_time(&self) -> Result<LocalTime, i32> {
        local_time()
    }
    fn current_directory(&self) -> Result<Vec<u8>, i32> {
        std::env::current_dir()
            .map(|path| path_bytes(path.as_os_str()))
            .map_err(os_error)
    }
    fn set_current_directory(&self, path: &[u8]) -> Result<(), i32> {
        std::env::set_current_dir(bytes_path(path)).map_err(os_error)
    }
    fn executable_path(&self) -> Vec<u8> {
        executable_path()
    }
}

#[cfg(unix)]
fn path_bytes(path: &std::ffi::OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    path.as_bytes().to_vec()
}
#[cfg(not(unix))]
fn path_bytes(path: &std::ffi::OsStr) -> Vec<u8> {
    path.to_string_lossy().as_bytes().to_vec()
}
#[cfg(unix)]
fn bytes_path(path: &[u8]) -> std::path::PathBuf {
    use std::os::unix::ffi::OsStrExt;
    std::ffi::OsStr::from_bytes(path).into()
}
#[cfg(not(unix))]
fn bytes_path(path: &[u8]) -> std::path::PathBuf {
    String::from_utf8_lossy(path).into_owned().into()
}

#[cfg(target_os = "linux")]
fn executable_path() -> Vec<u8> {
    // readlink("/proc/<getpid()>/exe"); an unreadable link gives the empty path.
    let link: String = format!("/proc/{}/exe", std::process::id());
    std::fs::read_link(link).map_or_else(|_| Vec::new(), |path| path_bytes(path.as_os_str()))
}
#[cfg(not(target_os = "linux"))]
fn executable_path() -> Vec<u8> {
    // macOS: _NSGetExecutablePath then realpath; Windows: GetModuleFileNameW. The standard
    // library asks the same services; canonicalize is the realpath step.
    let resolved = std::env::current_exe().and_then(|path| {
        if cfg!(target_vendor = "apple") {
            path.canonicalize()
        } else {
            Ok(path)
        }
    });
    resolved.map_or_else(|_| Vec::new(), |path| path_bytes(path.as_os_str()))
}

#[cfg(unix)]
fn is_super_user() -> Result<bool, i32> {
    Ok(rustix::process::getuid().is_root())
}
#[cfg(windows)]
mod windows_identity;
#[cfg(windows)]
use windows_identity::{is_super_user, user_name};

#[cfg(not(any(unix, windows)))]
fn is_super_user() -> Result<bool, i32> {
    Err(120)
}

#[cfg(unix)]
fn user_name() -> Result<Vec<u8>, i32> {
    let mut entry: libc::passwd = empty_passwd();
    let mut buffer: Vec<libc::c_char> = vec![0; 16 * 1024];
    let mut found: *mut libc::passwd = std::ptr::null_mut();
    // SAFETY: every pointer refers to live, writable storage of the stated length; the
    // reentrant call writes the entry strings into `buffer` only.
    let status: i32 = unsafe {
        libc::getpwuid_r(
            libc::getuid(),
            &mut entry,
            buffer.as_mut_ptr(),
            buffer.len(),
            &mut found,
        )
    };
    if status != 0 || found.is_null() || entry.pw_name.is_null() {
        return Ok(Vec::new()); // No account entry reads as "".
    }
    // SAFETY: pw_name points into `buffer`, NUL-terminated by getpwuid_r.
    Ok(unsafe { std::ffi::CStr::from_ptr(entry.pw_name) }
        .to_bytes()
        .to_vec())
}
#[cfg(target_os = "linux")]
fn empty_passwd() -> libc::passwd {
    libc::passwd {
        pw_name: std::ptr::null_mut(),
        pw_passwd: std::ptr::null_mut(),
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: std::ptr::null_mut(),
        pw_dir: std::ptr::null_mut(),
        pw_shell: std::ptr::null_mut(),
    }
}

#[cfg(target_os = "macos")]
fn empty_passwd() -> libc::passwd {
    // libc 0.2.190 generates safe field defaults, including null string pointers.
    libc::passwd::default()
}

#[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
fn empty_passwd() -> libc::passwd {
    // SAFETY: passwd is a plain C struct of integers and pointers; all-zero is valid.
    unsafe { std::mem::zeroed() }
}

#[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
#[test]
fn account_initialization_preserves_null_strings_and_zero_ids() {
    let entry = empty_passwd();
    assert_eq!((entry.pw_uid, entry.pw_gid), (0, 0));
    for pointer in [
        entry.pw_name,
        entry.pw_passwd,
        entry.pw_gecos,
        entry.pw_dir,
        entry.pw_shell,
    ] {
        assert!(pointer.is_null());
    }
}

#[cfg(all(test, target_os = "macos"))]
#[test]
fn apple_account_initialization_preserves_class_and_expiry() {
    let entry = empty_passwd();
    assert_eq!((entry.pw_change, entry.pw_expire), (0, 0));
    assert!(entry.pw_class.is_null());
}
#[cfg(not(any(unix, windows)))]
fn user_name() -> Result<Vec<u8>, i32> {
    // Until implemented, ERROR_CALL_NOT_IMPLEMENTED is recorded instead.
    Err(120)
}

#[cfg(unix)]
fn local_time() -> Result<LocalTime, i32> {
    let seconds: libc::time_t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as libc::time_t);
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let mut broken = crate::native_calendar::empty_calendar();
    // SAFETY: tm is a plain C struct; localtime_r fills it from a valid time_t.
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    let mut broken: libc::tm = unsafe { std::mem::zeroed() };
    // SAFETY: both pointers refer to live locals for the duration of the call.
    if unsafe { libc::localtime_r(&seconds, &mut broken) }.is_null() {
        return Err(io::Error::last_os_error().raw_os_error().unwrap_or(0));
    }
    Ok(calendar_local_time(&broken))
}

#[cfg(unix)]
fn calendar_local_time(broken: &libc::tm) -> LocalTime {
    LocalTime {
        month: broken.tm_mon + 1,
        day: broken.tm_mday,
        year: broken.tm_year + 1900,
        hour: broken.tm_hour,
        minute: broken.tm_min,
        second: broken.tm_sec,
        millisecond: 0,
    }
}
#[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
#[test]
fn calendar_conversion_preserves_offsets_and_zero_milliseconds() {
    let mut calendar = crate::native_calendar::empty_calendar();
    calendar.tm_year = 124;
    calendar.tm_mon = 1;
    calendar.tm_mday = 29;
    calendar.tm_hour = 13;
    calendar.tm_min = 5;
    calendar.tm_sec = 59;
    let actual = calendar_local_time(&calendar);
    assert_eq!((actual.month, actual.day, actual.year), (2, 29, 2024));
    assert_eq!((actual.hour, actual.minute, actual.second), (13, 5, 59));
    assert_eq!(actual.millisecond, 0);
}
#[cfg(windows)]
#[forbid(unsafe_code)]
fn local_time() -> Result<LocalTime, i32> {
    // The Windows system date uses GetLocalTime, including milliseconds.
    // CI job 114325767580 exposed the previous ERROR_CALL_NOT_IMPLEMENTED fallback.
    let clock = winsafe::GetLocalTime();
    Ok(system_time_local_time([
        clock.wYear,
        clock.wMonth,
        clock.wDayOfWeek,
        clock.wDay,
        clock.wHour,
        clock.wMinute,
        clock.wSecond,
        clock.wMilliseconds,
    ]))
}

#[cfg(windows)]
fn system_time_local_time(fields: [u16; 8]) -> LocalTime {
    let [year, month, _, day, hour, minute, second, millisecond] = fields;
    LocalTime {
        month: i32::from(month),
        day: i32::from(day),
        year: i32::from(year),
        hour: i32::from(hour),
        minute: i32::from(minute),
        second: i32::from(second),
        millisecond: i32::from(millisecond),
    }
}

#[cfg(all(test, windows))]
mod local_time_contracts;

// TODO(platform): hosts with neither the Unix nor the Windows clock API.
#[cfg(not(any(unix, windows)))]
fn local_time() -> Result<LocalTime, i32> {
    // Hosts without either supported clock API remain explicitly unsupported.
    Err(120)
}
