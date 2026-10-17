//! Permission and access contracts pinned before safe-wrapper migration (#89).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use martensite_tkernel::osd::AccessMode;
use std::os::unix::fs::{PermissionsExt, symlink};

#[test]
fn access_preserves_all_mode_combinations_and_first_nul() {
    let scratch = PathScratch::new("access-modes");
    let path = scratch.0.join("payload-é");
    std::fs::write(&path, b"payload").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
    for bits in 0..8 {
        let mode = AccessMode {
            read: bits & 1 != 0,
            write: bits & 2 != 0,
            execute: bits & 4 != 0,
        };
        assert_eq!(
            NativePosixCalls.access(&scratch.bytes("payload-é\0ignored".as_bytes()), mode),
            Ok(())
        );
    }
}

#[test]
fn chmod_follows_links_and_preserves_first_nul() {
    let scratch = PathScratch::new("chmod-links");
    let path = scratch.0.join("payload");
    std::fs::write(&path, b"payload").unwrap();
    symlink("payload", scratch.0.join("link-é")).unwrap();
    for mode in [0o600, 0o751, 0o1700] {
        NativePosixCalls
            .chmod(&scratch.bytes("link-é\0ignored".as_bytes()), mode)
            .unwrap();
        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o7777,
            mode
        );
    }
}

#[test]
fn access_and_chmod_preserve_path_errors() {
    let scratch = PathScratch::new("permission-errors");
    std::fs::write(scratch.0.join("file"), b"payload").unwrap();
    symlink("missing", scratch.0.join("dangling")).unwrap();
    for (path, errno) in [
        (b"\0ignored".to_vec(), libc::ENOENT),
        (scratch.bytes(b"missing"), libc::ENOENT),
        (scratch.bytes(b"dangling"), libc::ENOENT),
        (scratch.bytes(b"file/child"), libc::ENOTDIR),
    ] {
        assert_permission_path_error(&path, errno);
    }
}

fn assert_permission_path_error(path: &[u8], errno: i32) {
    let exists = AccessMode {
        read: false,
        write: false,
        execute: false,
    };
    assert_eq!(NativePosixCalls.access(path, exists), Err(Errno(errno)));
    assert_eq!(NativePosixCalls.chmod(path, 0o700), Err(Errno(errno)));
}
