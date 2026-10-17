//! Filesystem acceptance differs; neither path may silently use lossy names (#123).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

#[test]
fn invalid_utf8_names_preserve_bytes_or_the_filesystem_rejection() {
    let scratch = PathScratch::new("invalid-utf8");
    let probe = scratch.bytes(b"probe-\xff");
    match std::fs::create_dir(OsStr::from_bytes(&probe)) {
        Ok(()) => {
            std::fs::remove_dir(OsStr::from_bytes(&probe)).unwrap();
            assert_accepted_raw_names(&scratch);
        }
        Err(error) => {
            assert_eq!(error.raw_os_error(), Some(libc::EILSEQ));
            assert_rejected_raw_name(&scratch, &probe);
        }
    }
}

fn assert_accepted_raw_names(scratch: &PathScratch) {
    let source = scratch.bytes(b"source-\xff");
    let target = scratch.bytes(b"target-\xfe");
    super::assert_rename_and_unlink_names(source, target);
    super::assert_directory_name(scratch.bytes(b"dir-\xff"));
}

fn assert_rejected_raw_name(scratch: &PathScratch, probe: &[u8]) {
    let lossy = String::from_utf8_lossy(probe).into_owned();
    std::fs::write(&lossy, b"untouched").unwrap();
    let valid = scratch.bytes(b"valid");
    let calls = NativePosixCalls;
    assert_eq!(calls.mkdir(probe, 0o700), Err(Errno(libc::EILSEQ)));
    let path = OsStr::from_bytes(probe);
    assert_os_rejection(calls.unlink(probe), std::fs::remove_file(path));
    assert_os_rejection(calls.rmdir(probe), std::fs::remove_dir(path));
    assert_os_rejection(
        calls.rename(probe, &valid),
        std::fs::rename(path, OsStr::from_bytes(&valid)),
    );
    assert_invalid_rename_destination(scratch, probe);
    assert_eq!(std::fs::read(lossy).unwrap(), b"untouched");
    assert!(!std::path::Path::new(OsStr::from_bytes(&valid)).exists());
}

fn assert_invalid_rename_destination(scratch: &PathScratch, destination: &[u8]) {
    let source = scratch.bytes(b"rename-source");
    let path = OsStr::from_bytes(&source);
    std::fs::write(path, b"source preserved").unwrap();
    assert_os_rejection(
        NativePosixCalls.rename(&source, destination),
        std::fs::rename(path, OsStr::from_bytes(destination)),
    );
    assert_eq!(std::fs::read(path).unwrap(), b"source preserved");
}

fn assert_os_rejection(actual: Result<(), Errno>, reference: std::io::Result<()>) {
    // Lookup and creation need not reject the same name with the same errno (#123).
    let expected = reference.expect_err("invalid name must not identify an existing entry");
    let code = expected
        .raw_os_error()
        .expect("native filesystem error code");
    assert_eq!(actual, Err(Errno(code)));
}
