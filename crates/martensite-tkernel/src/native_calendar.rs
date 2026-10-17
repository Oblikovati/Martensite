//! Initialized native calendar storage without changing local-time conversion.

#[cfg(target_os = "linux")]
pub(crate) fn empty_calendar() -> libc::tm {
    libc::tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null(),
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn empty_calendar() -> libc::tm {
    // libc 0.2.190 generates safe field defaults, including a null zone pointer.
    libc::tm::default()
}

#[cfg(test)]
#[test]
fn empty_calendar_preserves_zero_fields_and_null_zone() {
    let calendar = empty_calendar();
    assert_eq!(
        [
            calendar.tm_sec,
            calendar.tm_min,
            calendar.tm_hour,
            calendar.tm_mday,
            calendar.tm_mon,
            calendar.tm_year,
            calendar.tm_wday,
            calendar.tm_yday,
            calendar.tm_isdst
        ],
        [0; 9]
    );
    assert_eq!(calendar.tm_gmtoff, 0);
    assert!(calendar.tm_zone.is_null());
}
