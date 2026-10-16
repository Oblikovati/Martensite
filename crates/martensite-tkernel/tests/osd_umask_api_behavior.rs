//! Isolated process-mask contracts before the safe-wrapper migration (#89).
#![cfg(unix)]
use martensite_tkernel::osd::{NativePosixCalls, PosixFileCalls};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

struct MaskScratch(PathBuf);
impl MaskScratch {
    fn new() -> Self {
        let suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("martensite-mask-{}-{suffix}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        Self(root)
    }
}
impl Drop for MaskScratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn umask_controls_created_modes_and_returns_previous_mask() {
    let calls = NativePosixCalls;
    let original = calls.umask(0o077);
    let scratch = MaskScratch::new();
    let mut previous = 0o077;
    for (mask, file_mode, directory_mode) in [
        (0o022, 0o644, 0o755),
        (0o002, 0o664, 0o775),
        (0o077, 0o600, 0o700),
    ] {
        assert_eq!(calls.umask(mask), previous);
        assert_created_modes(&scratch, file_mode, directory_mode);
        previous = mask;
    }
    assert_eq!(calls.umask(u32::MAX), previous);
    assert_eq!(calls.umask(original), 0o777);
}

fn assert_created_modes(scratch: &MaskScratch, file_mode: u32, directory_mode: u32) {
    let file = scratch.0.join("file");
    let directory = scratch.0.join("directory");
    std::fs::write(&file, b"payload").unwrap();
    std::fs::create_dir(&directory).unwrap();
    assert_eq!(
        std::fs::metadata(&file).unwrap().permissions().mode() & 0o7777,
        file_mode
    );
    assert_eq!(
        std::fs::metadata(&directory).unwrap().permissions().mode() & 0o7777,
        directory_mode
    );
    std::fs::remove_file(file).unwrap();
    std::fs::remove_dir(directory).unwrap();
}
