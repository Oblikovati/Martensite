//! Native record behavior pinned before replacing initialization (#89).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use martensite_tkernel::osd::RecordLock;
use std::os::fd::AsRawFd;

#[test]
fn native_record_lock_accepts_read_write_and_unlock() {
    let scratch = PathScratch::new("record-lock");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(scratch.0.join("locked"))
        .unwrap();
    for lock in [RecordLock::Read, RecordLock::Write, RecordLock::Unlock] {
        assert_eq!(NativePosixCalls.record_lock(file.as_raw_fd(), lock), Ok(()));
    }
}

#[test]
fn native_record_lock_invalid_descriptor_reports_ebadf() {
    for lock in [RecordLock::Read, RecordLock::Write, RecordLock::Unlock] {
        assert_eq!(
            NativePosixCalls.record_lock(-1, lock),
            Err(Errno(libc::EBADF))
        );
    }
}

const CALENDAR_CHILD: &str = "MARTENSITE_FILE_CALENDAR_CHILD";

#[test]
fn native_file_calendar_uses_utc_epoch() {
    if std::env::var_os(CALENDAR_CHILD).is_some() {
        assert_utc_calendar_fields();
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "osd_safe_path_operations::records::native_file_calendar_uses_utc_epoch",
        ])
        .env(CALENDAR_CHILD, "1")
        .env("TZ", "UTC0")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "calendar subprocess failed: {output:?}"
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
}

fn assert_utc_calendar_fields() {
    let clock = NativePosixCalls.local_time(0).unwrap();
    assert_eq!([clock.year, clock.month, clock.day], [1970, 1, 1]);
    assert_eq!(
        [clock.hour, clock.minute, clock.second, clock.millisecond],
        [0; 4]
    );
    let later = NativePosixCalls.local_time(86_461).unwrap();
    assert_eq!([later.year, later.month, later.day], [1970, 1, 2]);
    assert_eq!([later.hour, later.minute, later.second], [0, 1, 1]);
}
