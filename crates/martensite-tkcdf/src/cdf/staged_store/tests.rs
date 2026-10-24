//! Publication classification uses actual filesystem failures and independently expected types.
use super::{StagedStore, ensure_destination_writable, staging_directory, storage_failure};
use martensite_tkernel::tcollection::ExtendedString;
use std::{fs, io, path::PathBuf};

pub(super) struct TemporaryDestination {
    pub(super) directory: PathBuf,
    pub(super) file: PathBuf,
}
impl TemporaryDestination {
    pub(super) fn new() -> Self {
        let directory = staging_directory(&std::env::temp_dir()).unwrap();
        let file = directory.join("document");
        Self { directory, file }
    }
    fn name(&self) -> ExtendedString {
        ExtendedString::new(&self.file.to_string_lossy())
    }
}
impl Drop for TemporaryDestination {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

#[test]
fn missing_writer_output_retains_publication_class_and_device_cause() {
    let destination = TemporaryDestination::new();
    let stage = StagedStore::new(&destination.file).unwrap();
    let cause = stage.commit().unwrap_err();
    assert_eq!(cause.kind(), io::ErrorKind::NotFound);
    let failure = storage_failure(&destination.name(), cause, "publish");
    assert_eq!(failure.exception_type(), "DocumentStore_MissingOutput");
    assert!(failure.is_kind("Standard_Failure"));
    assert!(failure.message().contains("DocumentStore::publish"));
    assert!(failure.message().contains("cause_kind=NotFound"));
    assert!(failure.message().contains("cause_os_code=Some("));
    assert!(
        failure
            .message()
            .contains(&destination.file.to_string_lossy().to_string())
    );
    assert!(!destination.file.exists());
}

#[test]
fn read_only_destination_retains_permission_class_and_preserves_contents() {
    let destination = TemporaryDestination::new();
    fs::write(&destination.file, b"owned sentinel").unwrap();
    let original = fs::metadata(&destination.file).unwrap().permissions();
    let mut restricted = original.clone();
    restricted.set_readonly(true);
    fs::set_permissions(&destination.file, restricted).unwrap();
    let cause = ensure_destination_writable(&destination.file).unwrap_err();
    fs::set_permissions(&destination.file, original).unwrap();
    assert_eq!(cause.kind(), io::ErrorKind::PermissionDenied);
    let failure = storage_failure(&destination.name(), cause, "prepare");
    assert_eq!(failure.exception_type(), "DocumentStore_PermissionDenied");
    assert!(failure.message().contains("cause_kind=PermissionDenied"));
    assert!(
        failure
            .message()
            .contains("read-only; expected write access")
    );
    assert_eq!(fs::read(&destination.file).unwrap(), b"owned sentinel");
}

#[cfg(unix)]
#[test]
fn cyclic_final_symlink_is_refused_with_its_destination() {
    let destination = TemporaryDestination::new();
    std::os::unix::fs::symlink("document", &destination.file).unwrap();
    let failure = super::resolved_destination(&destination.file).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::InvalidInput);
    assert!(failure.to_string().contains("exceeded 40 links"));
    assert!(
        failure
            .to_string()
            .contains(&destination.file.to_string_lossy().to_string())
    );
}

#[cfg(windows)]
#[test]
fn named_stream_refusal_preserves_original_and_cleans_staging() {
    let destination = TemporaryDestination::new();
    fs::write(&destination.file, b"owned original").unwrap();
    let mut stream = destination.file.as_os_str().to_os_string();
    stream.push(":Zone.Identifier");
    let stream = PathBuf::from(stream);
    fs::write(&stream, b"[ZoneTransfer]\r\nZoneId=3\r\n").unwrap();
    let stage = StagedStore::new(&destination.file).unwrap();
    fs::write(&stage.file, b"completed replacement").unwrap();
    let directory = stage.directory.clone();
    let failure = storage_failure(&destination.name(), stage.commit().unwrap_err(), "publish");
    assert_eq!(failure.exception_type(), "DocumentStore_PermissionDenied");
    assert!(failure.message().contains("Zone.Identifier"));
    drop(stage);
    assert!(!directory.exists());
    assert_eq!(fs::read(&destination.file).unwrap(), b"owned original");
    assert_eq!(fs::read(stream).unwrap(), b"[ZoneTransfer]\r\nZoneId=3\r\n");
}
