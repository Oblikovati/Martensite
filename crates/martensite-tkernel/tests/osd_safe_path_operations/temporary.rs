//! Temporary-path contracts before replacing libc calls (#89).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use std::os::unix::{ffi::OsStrExt, fs::PermissionsExt};

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[test]
fn native_temporary_file_descriptor_is_inherited() {
    let scratch = PathScratch::new("temporary-inherit");
    let (fd, name) = NativePosixCalls.mkstemp(&scratch.bytes(b"XXXXXX")).unwrap();
    std::fs::write(std::ffi::OsStr::from_bytes(&name), b"inherited-temporary").unwrap();
    super::open::assert_descriptor_inherited(fd, b"inherited-temporary");
}

#[test]
fn native_temporary_file_preserves_raw_name_and_descriptor() {
    let scratch = PathScratch::new("temporary-file");
    let template = scratch.bytes(b"file-\xff-XXXXXX\0ignored");
    let (fd, name) = NativePosixCalls.mkstemp(&template).unwrap();
    assert_temporary_name(&template, &name);
    assert_eq!(NativePosixCalls.write(fd, b"payload").unwrap(), 7);
    NativePosixCalls.close(fd).unwrap();
    let path = std::ffi::OsStr::from_bytes(&name);
    assert_eq!(std::fs::read(path).unwrap(), b"payload");
    assert_eq!(
        std::fs::metadata(path).unwrap().permissions().mode() & 0o177,
        0
    );
}

#[test]
fn native_temporary_directory_preserves_raw_name_and_persistence() {
    let scratch = PathScratch::new("temporary-directory");
    let template = scratch.bytes(b"directory-\xff-XXXXXX\0ignored");
    let name = NativePosixCalls.mkdtemp(&template).unwrap();
    assert_temporary_name(&template, &name);
    let path = std::path::Path::new(std::ffi::OsStr::from_bytes(&name));
    assert!(path.is_dir());
    assert_eq!(
        std::fs::metadata(path).unwrap().permissions().mode() & 0o077,
        0
    );
    std::fs::write(path.join("child"), b"retained").unwrap();
    assert_eq!(std::fs::read(path.join("child")).unwrap(), b"retained");
}

fn assert_temporary_name(template: &[u8], name: &[u8]) {
    let end = template.iter().position(|&byte| byte == 0).unwrap();
    assert_eq!(name.len(), end);
    assert_eq!(&name[..end - 6], &template[..end - 6]);
    assert!(!name.contains(&0));
}

#[test]
fn native_temporary_paths_preserve_template_and_parent_errors() {
    let scratch = PathScratch::new("temporary-errors");
    for (template, errno) in [
        (scratch.bytes(b"invalid"), libc::EINVAL),
        (scratch.bytes(b"missing/XXXXXX"), libc::ENOENT),
    ] {
        assert_eq!(NativePosixCalls.mkstemp(&template), Err(Errno(errno)));
        assert_eq!(NativePosixCalls.mkdtemp(&template), Err(Errno(errno)));
    }
}
