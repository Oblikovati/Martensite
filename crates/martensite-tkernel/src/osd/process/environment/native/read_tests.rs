//! Process-owned environment fixtures avoid mutating the test runner's environment (#89).
use super::{EnvironmentSource, NativeEnvironment};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

const CHILD_TEST: &str =
    "osd::process::environment::native::read_tests::child_reads_owned_environment_bytes";

#[test]
fn child_process_preserves_raw_empty_missing_and_invalid_environment_reads() {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", CHILD_TEST, "--nocapture"])
        .env("MARTENSITE_NATIVE_ENV_CHILD", "1")
        .env(
            OsStr::from_bytes(b"MARTENSITE_NATIVE_\xff"),
            OsStr::from_bytes(b"raw\xfevalue"),
        )
        .env("MARTENSITE_NATIVE_EMPTY", "")
        .env_remove("MARTENSITE_NATIVE_MISSING")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
}

#[test]
fn child_reads_owned_environment_bytes() {
    if std::env::var_os("MARTENSITE_NATIVE_ENV_CHILD").is_none() {
        return;
    }
    let source = NativeEnvironment;
    assert_eq!(
        source.get(b"MARTENSITE_NATIVE_\xff"),
        Some(b"raw\xfevalue".to_vec())
    );
    assert_eq!(source.get(b"MARTENSITE_NATIVE_EMPTY"), Some(Vec::new()));
    assert_eq!(source.get(b"MARTENSITE_NATIVE_MISSING"), None);
    assert_eq!(source.get(b"MARTENSITE_NATIVE_EMPTY\0suffix"), None);
    assert_eq!(source.get(b""), None);
    assert_eq!(source.get(b"MARTENSITE_NATIVE_EMPTY="), None);
}
