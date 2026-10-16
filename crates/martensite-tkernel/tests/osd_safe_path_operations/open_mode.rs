//! Mode-width baseline before extending the safe open wrapper to macOS (#89).
use super::{NativePosixCalls, PathScratch, PosixFileCalls};
use martensite_tkernel::osd::{OpenFlags, OpenMode};
use std::os::unix::{ffi::OsStrExt, fs::PermissionsExt};

#[test]
fn native_open_creation_ignores_mode_bits_above_u16() {
    let scratch = PathScratch::new("open-mode-width");
    let low = create_mode(&scratch, b"low", 0o600);
    let high = create_mode(&scratch, b"high", 0xffff_0000 | 0o600);
    assert_eq!(low, high);
    assert_eq!(low & !0o600, 0);
}

fn create_mode(scratch: &PathScratch, name: &[u8], mode: u32) -> u32 {
    let flags = OpenFlags {
        access: OpenMode::ReadWrite,
        create: true,
        truncate: false,
        append: false,
    };
    let path = scratch.bytes(name);
    let fd = NativePosixCalls.open(&path, flags, mode).unwrap();
    NativePosixCalls.close(fd).unwrap();
    std::fs::metadata(std::ffi::OsStr::from_bytes(&path))
        .unwrap()
        .permissions()
        .mode()
        & 0o7777
}
