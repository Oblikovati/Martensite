//! Native status behavior before changing record initialization (#89).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use martensite_tkernel::osd::NodeType;
use std::os::{fd::AsRawFd, unix::fs::MetadataExt};

#[test]
fn path_and_descriptor_status_preserve_native_fields() {
    let scratch = PathScratch::new("status");
    let path = scratch.0.join("payload");
    std::fs::write(&path, b"payload").unwrap();
    let file = std::fs::File::open(&path).unwrap();
    let expected = file.metadata().unwrap();
    let actual = NativePosixCalls
        .stat(&scratch.bytes(b"payload\0ignored"))
        .unwrap();
    assert_eq!(actual, NativePosixCalls.fstat(file.as_raw_fd()).unwrap());
    assert_eq!(actual.node_type, NodeType::Regular);
    assert_eq!(actual.size, 7);
    assert_eq!(actual.permissions, expected.mode() & 0o7777);
    assert_eq!(actual.change_time, expected.ctime());
    assert_eq!(actual.block_size, expected.blksize());
}

#[test]
fn descriptor_status_survives_unlink() {
    let scratch = PathScratch::new("status-unlink");
    let path = scratch.0.join("payload");
    std::fs::write(&path, b"retained").unwrap();
    let file = std::fs::File::open(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    let actual = NativePosixCalls.fstat(file.as_raw_fd()).unwrap();
    assert_eq!(actual.node_type, NodeType::Regular);
    assert_eq!(actual.size, 8);
    assert_eq!(
        NativePosixCalls.stat(&scratch.bytes(b"payload")),
        Err(Errno(libc::ENOENT))
    );
}

#[test]
fn native_status_reports_empty_path_and_invalid_descriptor() {
    assert_eq!(
        NativePosixCalls.stat(b"\0ignored"),
        Err(Errno(libc::ENOENT))
    );
    assert_eq!(NativePosixCalls.fstat(-1), Err(Errno(libc::EBADF)));
}
