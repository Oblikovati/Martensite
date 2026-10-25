//! Native paths and archive member names have distinct separator contracts (#290).
use super::files;
use std::{fs, path::Path};

struct NativePayloadFixture {
    directory: tempfile::TempDir,
}

impl NativePayloadFixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let vendor = directory.path().join("vendor").join("dependency");
        fs::create_dir_all(&vendor).unwrap();
        fs::write(vendor.join("payload.rs"), b"sealed native payload").unwrap();
        Self { directory }
    }
}

#[test]
fn native_joined_components_produce_exact_portable_members_and_payloads() {
    let native = Path::new("vendor").join("dependency").join("payload.rs");
    assert_eq!(
        files::native_archive_path(&native).unwrap(),
        "vendor/dependency/payload.rs"
    );
    let fixture = NativePayloadFixture::new();
    let entries = files::entries(fixture.directory.path()).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].archive_path, "vendor/dependency/payload.rs");
    assert_eq!(
        fs::read(&entries[0].source).unwrap(),
        b"sealed native payload"
    );
}

#[test]
fn native_serialization_refuses_nonrelative_components_and_drive_names() {
    for path in ["", "..", "../escape", "/absolute", "vendor/C:payload"] {
        assert!(
            files::native_archive_path(Path::new(path)).is_err(),
            "{path}"
        );
    }
}

#[test]
fn raw_archive_members_still_reject_backslashes_and_drive_paths() {
    for path in [
        r"vendor\payload.rs",
        r"..\escape",
        r"C:\escape",
        "vendor/C:payload",
    ] {
        assert!(files::relative(Path::new(path)).is_err(), "{path}");
    }
}

#[cfg(unix)]
#[test]
fn native_unix_names_refuse_literal_backslashes_and_nonunicode_bytes() {
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
    assert!(files::native_archive_path(Path::new(r"vendor\payload.rs")).is_err());
    let nonunicode = Path::new(OsStr::from_bytes(b"payload\xff.rs"));
    assert!(files::native_archive_path(nonunicode).is_err());
}
