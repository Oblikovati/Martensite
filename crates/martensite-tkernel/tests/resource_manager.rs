//! Complete resource manager regressions using isolated filesystem/environment fixtures.
#![allow(non_snake_case)]
use martensite_tkernel::resource::ResourceManager;
use std::{path::PathBuf, sync::Arc};
#[path = "resource_manager/support.rs"]
mod support;
use support::FakeResourceSystem;
#[test]
fn Resource_ManagerTest__OCC27849_PathsWithSpecialChars() {
    let system: Arc<FakeResourceSystem> = Arc::default();
    for relative in [
        "path",
        "path.with.dots",
        "path with spaces",
        "nested/dirs/path with spaces",
    ] {
        let directory: PathBuf = PathBuf::from("resource_manager_test").join(relative);
        system.write_resource(&directory, "TestResource", "test.resource", "ok");
        system.set_environment("CSF_TestResourceDefaults", directory);
        let resources = ResourceManager::new("TestResource", false, system.host()).unwrap();
        assert!(resources.find("test.resource"));
        if resources.find("test.resource") {
            assert_eq!(resources.value("test.resource").unwrap().as_bytes(), b"ok");
        }
    }
}
#[test]
fn Resource_ManagerTest__OCC181_SaveToExistingDirectory() {
    let system: Arc<FakeResourceSystem> = Arc::default();
    let source: PathBuf = PathBuf::from("resource_manager_test/source");
    let save: PathBuf = PathBuf::from("resource_manager_test/save_flat");
    system.write_resource(&source, "SaveCheck", "test.key", "test_value");
    system.add_directory(save.clone());
    system.set_environment("CSF_SaveCheckUserDefaults", source);
    let resources = ResourceManager::new("SaveCheck", false, system.host()).unwrap();
    system.set_environment("CSF_SaveCheckUserDefaults", save.clone());
    assert!(resources.save());
    assert!(system.has_file(&save.join("SaveCheck")));
}
#[test]
fn Resource_ManagerTest__OCC181_SaveToNestedNonExistentDirectory() {
    let system: Arc<FakeResourceSystem> = Arc::default();
    let source: PathBuf = PathBuf::from("resource_manager_test/source2");
    let save: PathBuf = PathBuf::from("resource_manager_test/nested/deep/dir");
    system.write_resource(&source, "SaveCheck", "test.key", "test_value");
    system.set_environment("CSF_SaveCheckUserDefaults", source);
    let resources = ResourceManager::new("SaveCheck", false, system.host()).unwrap();
    system.set_environment("CSF_SaveCheckUserDefaults", save.clone());
    assert!(resources.save());
    assert!(system.has_file(&save.join("SaveCheck")));
}
