//! Native path compatibility fixtures for the safe system-call migration (#89).
#![cfg(unix)]
use martensite_tkernel::osd::{Errno, NativePosixCalls, PosixFileCalls};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

struct PathScratch(PathBuf);
impl PathScratch {
    fn new(label: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "martensite-path-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
    fn bytes(&self, name: &[u8]) -> Vec<u8> {
        self.0
            .join(std::ffi::OsStr::from_bytes(name))
            .as_os_str()
            .as_bytes()
            .to_vec()
    }
}
impl Drop for PathScratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn native_rename_and_unlink_preserve_raw_names_and_nul_truncation() {
    let scratch = PathScratch::new("raw");
    let source = scratch.bytes(b"source-\xff");
    let target = scratch.bytes(b"target-\xfe");
    std::fs::write(std::ffi::OsStr::from_bytes(&source), b"payload").unwrap();
    let mut terminated_source = source.clone();
    terminated_source.extend_from_slice(b"\0ignored");
    let mut terminated_target = target.clone();
    terminated_target.extend_from_slice(b"\0ignored");
    NativePosixCalls
        .rename(&terminated_source, &terminated_target)
        .unwrap();
    assert_eq!(
        std::fs::read(std::ffi::OsStr::from_bytes(&target)).unwrap(),
        b"payload"
    );
    assert!(!std::path::Path::new(std::ffi::OsStr::from_bytes(&source)).exists());
    NativePosixCalls.unlink(&terminated_target).unwrap();
    assert_eq!(NativePosixCalls.unlink(&target), Err(Errno(libc::ENOENT)));
}

#[test]
fn native_directory_calls_preserve_mode_and_nonempty_error() {
    let scratch = PathScratch::new("directory");
    let directory = scratch.bytes(b"dir-\xff");
    let mut terminated = directory.clone();
    terminated.extend_from_slice(b"\0ignored");
    NativePosixCalls.mkdir(&terminated, 0o700).unwrap();
    let path = std::path::Path::new(std::ffi::OsStr::from_bytes(&directory));
    assert_eq!(
        std::fs::metadata(path).unwrap().permissions().mode() & 0o077,
        0
    );
    assert_eq!(
        NativePosixCalls.mkdir(&terminated, 0o700),
        Err(Errno(libc::EEXIST))
    );
    std::fs::write(path.join("child"), b"value").unwrap();
    assert_eq!(
        NativePosixCalls.rmdir(&terminated),
        Err(Errno(libc::ENOTEMPTY))
    );
    std::fs::remove_file(path.join("child")).unwrap();
    NativePosixCalls.rmdir(&terminated).unwrap();
    assert!(!path.exists());
}

#[test]
fn native_empty_paths_report_enoent_after_nul_truncation() {
    let calls = NativePosixCalls;
    assert_eq!(calls.rename(b"", b""), Err(Errno(libc::ENOENT)));
    assert_eq!(calls.unlink(b"\0ignored"), Err(Errno(libc::ENOENT)));
    assert_eq!(calls.rmdir(b""), Err(Errno(libc::ENOENT)));
    assert_eq!(calls.mkdir(b"\0ignored", 0o700), Err(Errno(libc::ENOENT)));
}
