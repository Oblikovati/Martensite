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
    let template = scratch.bytes(b"file-XXXXXX\0ignored");
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
    let template = scratch.bytes(b"directory-XXXXXX\0ignored");
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
        (Vec::new(), libc::EINVAL),
        (scratch.bytes(b"missing/XXXXXX"), libc::ENOENT),
    ] {
        assert_eq!(NativePosixCalls.mkstemp(&template), Err(Errno(errno)));
        assert_eq!(NativePosixCalls.mkdtemp(&template), Err(Errno(errno)));
    }
}

#[test]
fn native_temporary_raw_names_follow_filesystem_acceptance() {
    let scratch = PathScratch::new("temporary-raw");
    let template = scratch.bytes(b"raw-\xff-XXXXXX\0ignored");
    let probe = scratch.bytes(b"probe-\xff");
    match std::fs::create_dir(std::ffi::OsStr::from_bytes(&probe)) {
        Ok(()) => assert_raw_temporary_success(&template),
        Err(error) => {
            assert_eq!(error.raw_os_error(), Some(libc::EILSEQ));
            assert_eq!(
                NativePosixCalls.mkstemp(&template),
                Err(Errno(libc::EILSEQ))
            );
            assert_eq!(
                NativePosixCalls.mkdtemp(&template),
                Err(Errno(libc::EILSEQ))
            );
        }
    }
}

fn assert_raw_temporary_success(template: &[u8]) {
    let (fd, file) = NativePosixCalls.mkstemp(template).unwrap();
    NativePosixCalls.close(fd).unwrap();
    let directory = NativePosixCalls.mkdtemp(template).unwrap();
    assert_temporary_name(template, &file);
    assert_temporary_name(template, &directory);
    assert_ne!(file, directory);
}

#[cfg(target_os = "linux")]
#[test]
fn native_temporary_linux_requires_six_trailing_xs() {
    let scratch = PathScratch::new("temporary-template");
    let template = scratch.bytes(b"no-xs");
    assert_eq!(
        NativePosixCalls.mkstemp(&template),
        Err(Errno(libc::EINVAL))
    );
    assert_eq!(
        NativePosixCalls.mkdtemp(&template),
        Err(Errno(libc::EINVAL))
    );
}

#[cfg(target_os = "macos")]
#[test]
fn native_temporary_macos_accepts_zero_trailing_xs() {
    // Darwin's find_temp_path accepts zero Xs; preserve that contract (#140).
    let scratch = PathScratch::new("temporary-template");
    let file = scratch.bytes(b"no-xs-file");
    let (fd, created) = NativePosixCalls.mkstemp(&file).unwrap();
    NativePosixCalls.close(fd).unwrap();
    assert_eq!(created, file);
    let directory = scratch.bytes(b"no-xs-directory");
    assert_eq!(NativePosixCalls.mkdtemp(&directory).unwrap(), directory);
}
