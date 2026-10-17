//! Field initialization for the native record-lock layouts verified in #89.

#[cfg(any(
    target_os = "macos",
    all(
        target_os = "linux",
        target_env = "gnu",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
))]
pub(super) fn empty_lock_record() -> libc::flock {
    libc::flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    }
}

#[cfg(not(any(
    target_os = "macos",
    all(
        target_os = "linux",
        target_env = "gnu",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
)))]
pub(super) fn empty_lock_record() -> libc::flock {
    // Other libc layouts can contain private padding; retain their existing
    // initialization until those targets receive their own migration evidence.
    // SAFETY: an all-zero flock is a valid value of this plain C struct.
    unsafe { std::mem::zeroed() }
}

#[cfg(test)]
#[test]
fn lock_record_preserves_zero_fields() {
    let record = empty_lock_record();
    assert_eq!([record.l_type, record.l_whence], [0; 2]);
    assert_eq!([record.l_start, record.l_len], [0; 2]);
    assert_eq!(record.l_pid, 0);
}
