//! The C `errno` value and its `strerror` wording, for glibc's `%m` directive.

/// The calling thread's C `errno`; `current_errno()` is 0 when no call has failed.
pub(crate) fn current_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// `strerror(code)` as the C runtime words it; `os_error_text(12) == "Cannot allocate memory"`.
#[cfg(not(windows))]
pub(crate) fn os_error_text(code: i32) -> String {
    let text = std::io::Error::from_raw_os_error(code).to_string();
    let suffix = format!(" (os error {code})");
    text.strip_suffix(&suffix).unwrap_or(&text).to_owned()
}

/// `strerror(code)` as the C runtime words it; `os_error_text(12) == "Not enough space"`.
#[cfg(windows)]
#[allow(unsafe_code)]
pub(crate) fn os_error_text(code: i32) -> String {
    unsafe extern "C" {
        fn strerror_s(buffer: *mut libc::c_char, size: usize, code: i32) -> libc::c_int;
    }
    let mut buffer = [0_u8; 128];
    // SAFETY: `buffer` is a writable 128-byte array and strerror_s writes at most `buffer.len()` bytes including the NUL. The documented POSIX allocation model uses CRT errno values, not Win32 codes
    // (runner 38133190067). Copy into owned storage so parallel errors cannot alias.
    let status = unsafe { strerror_s(buffer.as_mut_ptr().cast(), buffer.len(), code) };
    assert_eq!(status, 0, "CRT errno {code}: expected strerror_s success");
    let end = buffer
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(buffer.len());
    String::from_utf8_lossy(&buffer[..end]).into_owned()
}

#[cfg(all(test, target_os = "linux", target_env = "gnu"))]
mod tests {
    #[test]
    fn glibc_wording_covers_success_and_unknown_codes() {
        assert_eq!(super::os_error_text(0), "Success");
        assert_eq!(super::os_error_text(2), "No such file or directory");
        assert_eq!(super::os_error_text(9999), "Unknown error 9999");
    }
}
