//! Path-status contracts before replacing the native stat call (#89).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use martensite_tkernel::osd::NodeType;
use std::os::unix::{ffi::OsStrExt, fs::MetadataExt, fs::symlink, net::UnixListener};

#[test]
fn path_status_distinguishes_directory_socket_and_followed_link() {
    let scratch = PathScratch::new("status-kinds");
    std::fs::create_dir(scratch.0.join("directory")).unwrap();
    std::fs::write(scratch.0.join("file"), b"payload").unwrap();
    symlink("file", scratch.0.join("link")).unwrap();
    let _socket = UnixListener::bind(scratch.0.join("socket")).unwrap();
    for (name, kind) in [
        (b"directory".as_slice(), NodeType::Directory),
        (b"socket".as_slice(), NodeType::Socket),
        (b"link".as_slice(), NodeType::Regular),
    ] {
        assert_status_fields(&scratch, name, kind);
    }
}

fn assert_status_fields(scratch: &PathScratch, name: &[u8], kind: NodeType) {
    let bytes = scratch.bytes(name);
    let expected = std::fs::metadata(std::ffi::OsStr::from_bytes(&bytes)).unwrap();
    let actual = NativePosixCalls.stat(&bytes).unwrap();
    assert_eq!(actual.node_type, kind);
    assert_eq!(actual.permissions, expected.mode() & 0o7777);
    assert_eq!(actual.size, expected.size());
    assert_eq!(actual.change_time, expected.ctime());
    assert_eq!(actual.block_size, expected.blksize());
}

#[test]
fn path_status_preserves_dangling_link_and_nondirectory_errors() {
    let scratch = PathScratch::new("status-errors");
    std::fs::write(scratch.0.join("file"), b"payload").unwrap();
    symlink("missing", scratch.0.join("dangling")).unwrap();
    symlink("loop", scratch.0.join("loop")).unwrap();
    for (name, errno) in [
        (b"dangling".as_slice(), libc::ENOENT),
        (b"file/child".as_slice(), libc::ENOTDIR),
        (b"loop".as_slice(), libc::ELOOP),
    ] {
        assert_eq!(
            NativePosixCalls.stat(&scratch.bytes(name)),
            Err(Errno(errno))
        );
    }
}
