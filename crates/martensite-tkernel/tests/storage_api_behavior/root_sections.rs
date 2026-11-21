//! Root and type section behavior against a scripted driver.
use super::scripted_driver::ScriptedDriver;
use crate::storage::{OpenMode, RootData, StorageError, StreamException, TypeData};

#[test]
fn root_and_type_reads_bind_every_entry() {
    let roots = RootData::default();
    let mut driver = ScriptedDriver::opened(OpenMode::Read);
    driver.roots = vec![("doc", 1, "PDoc"), ("aux", 2, "PAux")];
    assert!(roots.read(&mut driver).unwrap());
    assert_eq!(roots.number_of_roots(), 2);
    assert_eq!(roots.find(&"aux".into()).get().unwrap().reference(), 2);
    let types = TypeData::default();
    driver.types = vec![(1, "PDoc"), (2, "PAux")];
    assert!(types.read(&mut driver).unwrap());
    assert_eq!(types.type_number(&"PAux".into()).unwrap(), 2);
}

#[test]
fn a_type_mismatch_in_an_entry_stops_the_section() {
    let roots = RootData::default();
    let mut driver = ScriptedDriver::opened(OpenMode::Read);
    driver.roots = vec![("doc", 1, "PDoc")];
    driver.fail_at = Some(("ReadRoot", StreamException::TypeMismatch));
    assert!(!roots.read(&mut driver).unwrap());
    assert_eq!(roots.error_status(), StorageError::TypeMismatch);
    assert_eq!(roots.error_status_extension().as_bytes(), b"ReadRoot");
    let types = TypeData::default();
    driver.fail_at = Some(("TypeSectionSize", StreamException::TypeMismatch));
    assert!(
        types.read(&mut driver).is_err(),
        "section sizes are read outside the try"
    );
    let mut writer = ScriptedDriver::opened(OpenMode::Write);
    assert!(!types.read(&mut writer).unwrap());
    assert_eq!(types.error_status(), StorageError::ModeError);
    roots.clear_error_status();
    assert_eq!(roots.error_status(), StorageError::Ok);
}
