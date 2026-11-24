//! Complete resource manager regressions using isolated filesystem/environment fixtures.
#![allow(non_snake_case)]
use martensite_tkernel::resource::ResourceManager;
use std::{path::PathBuf, sync::Arc};
#[path = "resource_manager/support.rs"]
#[allow(dead_code)]
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
