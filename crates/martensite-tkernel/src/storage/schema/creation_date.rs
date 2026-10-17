//! The clock behind the schema creation date (`strftime("%m/%d/%Y")` of local time).
//! Narrow platform FFI boundary for the local calendar date.
#![allow(unsafe_code)]
use crate::tcollection::AsciiString;

/// Supplies the local calendar date a schema stamps on written data.
pub trait CreationDateSource: Send + Sync {
    /// Local `(month, day, year)`; `clock.local_date()`.
    fn local_date(&self) -> (u32, u32, i32);

    /// The `%m/%d/%Y` text; `clock.creation_date()`.
    fn creation_date(&self) -> AsciiString {
        let (month, day, year) = self.local_date();
        AsciiString::new(&format!("{month:02}/{day:02}/{year}"))
    }
}

/// The host's local time zone, as `localtime` reads it; `Arc::new(NativeCreationDate)`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeCreationDate;

impl CreationDateSource for NativeCreationDate {
    fn local_date(&self) -> (u32, u32, i32) {
        native_local_date()
    }
}

#[cfg(unix)]
fn native_local_date() -> (u32, u32, i32) {
    // SAFETY: `time` accepts a null pointer; `parts` is a zeroed, writable `tm` that
    // `localtime_r` fills and that outlives the call.
    let parts = unsafe {
        let now = libc::time(std::ptr::null_mut());
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let mut parts = crate::native_calendar::empty_calendar();
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        let mut parts: libc::tm = std::mem::zeroed();
        libc::localtime_r(&now, &mut parts);
        parts
    };
    (
        (parts.tm_mon + 1) as u32,
        parts.tm_mday as u32,
        parts.tm_year + 1900,
    )
}

#[cfg(windows)]
fn native_local_date() -> (u32, u32, i32) {
    use windows_sys::Win32::{Foundation::SYSTEMTIME, System::SystemInformation::GetLocalTime};
    let mut now = SYSTEMTIME::default();
    // SAFETY: `now` is initialized, writable and lives through `GetLocalTime`.
    unsafe { GetLocalTime(&mut now) };
    windows_calendar_date([
        now.wYear,
        now.wMonth,
        now.wDayOfWeek,
        now.wDay,
        now.wHour,
        now.wMinute,
        now.wSecond,
        now.wMilliseconds,
    ])
}

#[cfg(windows)]
fn windows_calendar_date(fields: [u16; 8]) -> (u32, u32, i32) {
    let [year, month, _, day, _, _, _, _] = fields;
    (u32::from(month), u32::from(day), i32::from(year))
}

#[cfg(all(test, windows))]
mod windows_contracts;

#[cfg(all(test, windows))]
#[test]
fn windows_creation_date_initialization_preserves_zero_fields() {
    let now = windows_sys::Win32::Foundation::SYSTEMTIME::default();
    assert_eq!(
        [
            now.wYear,
            now.wMonth,
            now.wDayOfWeek,
            now.wDay,
            now.wHour,
            now.wMinute,
            now.wSecond,
            now.wMilliseconds
        ],
        [0; 8]
    );
}
