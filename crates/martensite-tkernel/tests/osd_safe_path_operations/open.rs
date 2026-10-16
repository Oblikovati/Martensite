//! Owned fixtures for the raw descriptor API before safe open migration (#89).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use martensite_tkernel::osd::{OpenFlags, OpenMode, SeekOrigin};

const READ_ONLY: OpenFlags = OpenFlags {
    access: OpenMode::ReadOnly,
    create: false,
    truncate: false,
    append: false,
};
const CREATE_READ_WRITE: OpenFlags = OpenFlags {
    access: OpenMode::ReadWrite,
    create: true,
    ..READ_ONLY
};

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[test]
fn native_open_descriptor_remains_inherited_by_executed_child() {
    let scratch = PathScratch::new("open-inherit");
    std::fs::write(scratch.0.join("payload"), b"inherited-open-fixture").unwrap();
    let fd = NativePosixCalls
        .open(&scratch.bytes(b"payload"), READ_ONLY, 0)
        .unwrap();
    let output = std::process::Command::new("/bin/cat")
        .arg(format!("/dev/fd/{fd}"))
        .output()
        .unwrap();
    NativePosixCalls.close(fd).unwrap();
    assert!(output.status.success(), "{:?}", output.stderr);
    assert_eq!(output.stdout, b"inherited-open-fixture");
}

#[test]
fn native_open_preserves_read_only_and_nul_truncation() {
    let scratch = PathScratch::new("open-read");
    std::fs::write(scratch.0.join("payload"), b"abc").unwrap();
    let fd = NativePosixCalls
        .open(&scratch.bytes(b"payload\0ignored"), READ_ONLY, 0)
        .unwrap();
    let mut bytes = [0; 3];
    assert_eq!(NativePosixCalls.read(fd, &mut bytes).unwrap(), 3);
    assert_eq!(&bytes, b"abc");
    assert_eq!(NativePosixCalls.write(fd, b"x"), Err(Errno(libc::EBADF)));
    NativePosixCalls.close(fd).unwrap();
}

#[test]
fn native_open_append_overrides_seek_position() {
    let scratch = PathScratch::new("open-append");
    std::fs::write(scratch.0.join("payload"), b"abc").unwrap();
    let flags = OpenFlags {
        access: OpenMode::WriteOnly,
        append: true,
        ..READ_ONLY
    };
    let fd = NativePosixCalls
        .open(&scratch.bytes(b"payload"), flags, 0)
        .unwrap();
    assert_eq!(NativePosixCalls.lseek(fd, 0, SeekOrigin::Start).unwrap(), 0);
    assert_eq!(NativePosixCalls.write(fd, b"d").unwrap(), 1);
    NativePosixCalls.close(fd).unwrap();
    assert_eq!(std::fs::read(scratch.0.join("payload")).unwrap(), b"abcd");
}

#[test]
fn native_open_missing_and_empty_paths_preserve_errno() {
    let scratch = PathScratch::new("open-errors");
    assert_eq!(
        NativePosixCalls.open(&scratch.bytes(b"missing"), READ_ONLY, 0),
        Err(Errno(libc::ENOENT))
    );
    assert_eq!(
        NativePosixCalls.open(b"\0ignored", READ_ONLY, 0),
        Err(Errno(libc::ENOENT))
    );
}

#[test]
fn native_open_creation_retains_descriptor_and_read_write_access() {
    let scratch = PathScratch::new("open-create");
    let fd = NativePosixCalls
        .open(&scratch.bytes(b"new"), CREATE_READ_WRITE, 0o600)
        .unwrap();
    assert_eq!(NativePosixCalls.write(fd, b"abc").unwrap(), 3);
    assert_eq!(NativePosixCalls.lseek(fd, 0, SeekOrigin::Start).unwrap(), 0);
    let mut bytes = [0; 3];
    assert_eq!(NativePosixCalls.read(fd, &mut bytes).unwrap(), 3);
    assert_eq!(&bytes, b"abc");
    NativePosixCalls.close(fd).unwrap();
    assert_eq!(std::fs::read(scratch.0.join("new")).unwrap(), b"abc");
}

#[test]
fn native_open_create_preserves_existing_contents_and_truncate_removes_them() {
    let scratch = PathScratch::new("open-truncate");
    std::fs::write(scratch.0.join("payload"), b"abc").unwrap();
    let path = scratch.bytes(b"payload");
    let fd = NativePosixCalls
        .open(&path, CREATE_READ_WRITE, 0o600)
        .unwrap();
    NativePosixCalls.close(fd).unwrap();
    assert_eq!(std::fs::read(scratch.0.join("payload")).unwrap(), b"abc");
    let flags = OpenFlags {
        truncate: true,
        ..CREATE_READ_WRITE
    };
    let fd = NativePosixCalls.open(&path, flags, 0o600).unwrap();
    NativePosixCalls.close(fd).unwrap();
    assert!(std::fs::read(scratch.0.join("payload")).unwrap().is_empty());
}
