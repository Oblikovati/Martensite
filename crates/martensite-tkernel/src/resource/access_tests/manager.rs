//! Constructor and query behavior through the private resource methods.
use super::{
    fixtures::{console, system_with},
    support::FakeResourceSystem,
};
use crate::{resource::ResourceManager, tcollection::AsciiString};
use std::{path::PathBuf, sync::Arc};

#[test]
fn directory_constructor_loads_both_files_into_the_reference_map() {
    let system = system_with("App", b"a: ref\nb: ref\n");
    system.write_resource(&PathBuf::from("user"), "App", "b", "user");
    let mut manager =
        ResourceManager::with_directories("App", "defaults", "user/", false, system.host())
            .unwrap();
    assert_eq!(manager.value("b").unwrap().as_bytes(), b"user");
    assert_eq!(manager.get_map(true).len(), 2);
    assert!(manager.get_map(false).is_empty());
    let empty = ResourceManager::with_directories("App", "", "", true, system.host()).unwrap();
    assert!(!empty.is_initialized());
    assert!(console(&system).ends_with(
        "aDefaultsDirectory is empty.\nResource Manager Warning: anUserDefaultsDirectory is empty.\n"
    ));
}

#[test]
fn empty_manager_lookups_assignments_and_maps() {
    let system: Arc<FakeResourceSystem> = Arc::default();
    let mut manager = ResourceManager::empty(system.host());
    assert!(!manager.is_initialized());
    manager.set_resource("text", b"abc\0ignored");
    manager.set_resource_integer("int", -7);
    manager.set_resource_real("real", 0.1);
    manager.set_resource_real("big", 1e300);
    assert_eq!(manager.value("text").unwrap().as_bytes(), b"abc");
    assert_eq!(manager.value("int").unwrap().as_bytes(), b"-7");
    assert_eq!(manager.value("real").unwrap().as_bytes(), b"0.1");
    assert_eq!(manager.value("big").unwrap().as_bytes(), b"1e+300");
    let found = manager.find_value(&AsciiString::new("int")).unwrap();
    assert_eq!(found.as_bytes(), b"-7");
    assert!(manager.find_value(&AsciiString::new("none")).is_none());
    manager
        .get_map(true)
        .bind(AsciiString::new("ref"), AsciiString::new("r"));
    assert!(manager.find("ref"));
    assert_eq!(manager.get_map(false).len(), 4);
    let missing = manager.value("none").unwrap_err();
    assert_eq!(missing.to_string(), "Resource_NoSuchResource: none");
}

#[test]
fn resource_path_follows_osd_path_rules() {
    let system: Arc<FakeResourceSystem> = Arc::default();
    let path = |directory: &str, user: bool| {
        let variable = if user {
            "CSF_AppUserDefaults"
        } else {
            "CSF_AppDefaults"
        };
        system.set_environment(variable, PathBuf::from(directory));
        let path = ResourceManager::resource_path_with("App", user, system.as_ref());
        String::from_utf8(path.as_bytes().to_vec()).unwrap()
    };
    assert_eq!(path("", false), "");
    assert_eq!(path("a/b", false), "a/b/App");
    assert_eq!(path("a/b/", true), "a/b/App");
    assert_eq!(path(".hidden", false), "App");
    assert_eq!(path("a/.hidden", true), "a/App");
    assert_eq!(path("a/b.c", false), "a/b.c/App");
    let native = ResourceManager::resource_path("MartensiteUnsetResourceName", false);
    assert!(native.is_empty());
}

#[test]
fn numeric_conversion_missing_resources_and_missing_save_path() {
    let system = system_with("App", b"int: -42extra\nreal: 1.25more\nbad: nothing\n");
    let mut catalog = ResourceManager::new("App", false, system.host()).unwrap();
    assert_eq!(catalog.integer("int").unwrap(), -42);
    assert_eq!(catalog.real("real").unwrap(), 1.25);
    assert_eq!(
        catalog.integer("bad").unwrap_err().exception_type(),
        "Standard_TypeMismatch"
    );
    assert_eq!(
        catalog.real("bad").unwrap_err().exception_type(),
        "Standard_TypeMismatch"
    );
    assert_eq!(
        catalog.value("missing").unwrap_err().exception_type(),
        "Resource_NoSuchResource"
    );
    assert!(!catalog.save());
    assert!(!system.has_file(&PathBuf::from("out/App")));
    catalog.set_resource_integer("int", 123);
    assert_eq!(catalog.integer("int").unwrap(), 123);
    let missing = Arc::new(FakeResourceSystem::default());
    assert!(
        !ResourceManager::new("missing", false, missing.host())
            .unwrap()
            .is_initialized()
    );
}
