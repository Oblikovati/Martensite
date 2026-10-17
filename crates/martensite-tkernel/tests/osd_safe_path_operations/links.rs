//! Symlink and overwrite fixtures shared by the native and safe implementations.
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use std::os::unix::{ffi::OsStrExt, fs::symlink};

#[test]
fn rename_replaces_destination_symlink_without_modifying_its_target() {
    let scratch = PathScratch::new("rename-link");
    let source = scratch.0.join("source");
    let target = scratch.0.join("target");
    let link = scratch.0.join("link");
    std::fs::write(&source, b"replacement").unwrap();
    std::fs::write(&target, b"original").unwrap();
    symlink(&target, &link).unwrap();
    NativePosixCalls
        .rename(source.as_os_str().as_bytes(), link.as_os_str().as_bytes())
        .unwrap();
    assert_eq!(std::fs::read(&link).unwrap(), b"replacement");
    assert_eq!(std::fs::read(&target).unwrap(), b"original");
    assert!(!std::fs::symlink_metadata(&link).unwrap().is_symlink());
    assert!(!source.exists());
}

#[test]
fn unlink_removes_dangling_symlink_without_following_it() {
    let scratch = PathScratch::new("unlink-link");
    let missing = scratch.0.join("missing");
    let link = scratch.0.join("link");
    symlink(&missing, &link).unwrap();
    NativePosixCalls
        .unlink(link.as_os_str().as_bytes())
        .unwrap();
    assert_eq!(
        std::fs::symlink_metadata(&link).unwrap_err().raw_os_error(),
        Some(libc::ENOENT)
    );
    assert!(!missing.exists());
}

#[test]
fn directory_operations_do_not_treat_symlink_as_directory_entry() {
    let scratch = PathScratch::new("directory-link");
    let directory = scratch.0.join("directory");
    let link = scratch.0.join("link");
    std::fs::create_dir(&directory).unwrap();
    symlink(&directory, &link).unwrap();
    let name = link.as_os_str().as_bytes();
    assert_eq!(
        NativePosixCalls.mkdir(name, 0o700),
        Err(Errno(libc::EEXIST))
    );
    assert_eq!(NativePosixCalls.rmdir(name), Err(Errno(libc::ENOTDIR)));
    assert!(directory.is_dir());
    assert!(std::fs::symlink_metadata(&link).unwrap().is_symlink());
}
