//! Resource output, directory creation and the process-wide debug trace.
#![allow(non_snake_case)]
use super::{
    fixtures::{console, system_with},
    support::FakeResourceSystem,
};
use crate::resource::ResourceManager;
use std::{path::PathBuf, sync::Arc};

#[test]
fn resource_debug_traces_every_key_value_and_saved_line() {
    const CHILD: &str = "MARTENSITE_RESOURCE_DEBUG_CHILD";
    if std::env::var_os(CHILD).is_none() {
        run_debug_trace_child(CHILD);
        return;
    }
    let system = Arc::new(FakeResourceSystem::default());
    let directory = PathBuf::from("defaults");
    system.write_resource(&directory, "App", "k", "v");
    system.set_environment("CSF_AppDefaults", directory);
    system.set_environment("ResourceDebug", PathBuf::from("1"));
    let mut manager = ResourceManager::new("App", false, system.host()).unwrap();
    manager.set_resource("s", b"x");
    system.set_environment("CSF_AppUserDefaults", PathBuf::from("out"));
    assert!(manager.save());
    assert_eq!(
        console(&system),
        "Key = 'k'\t Value = 'v'\nLine = 's:\tx\n'\n"
    );
    // The flag is process-wide and re-read by every name-based constructor.
    let quiet = system_with("Other", b"q: 1\n");
    let _ = ResourceManager::new("Other", false, quiet.host()).unwrap();
    assert!(console(&quiet).is_empty());
}

fn run_debug_trace_child(marker: &str) {
    let (_, module) = module_path!().split_once("::").unwrap();
    let case = format!("{module}::resource_debug_traces_every_key_value_and_saved_line");
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", &case])
        .env(marker, "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stdout.contains("1 passed"),
        "{case} exited with {}\n{stdout}\n{stderr}",
        output.status
    );
}

#[test]
fn save_without_variable_or_entries_follows_native_messages() {
    let system: Arc<FakeResourceSystem> = Arc::default();
    let manager = ResourceManager::new("App", true, system.host()).unwrap();
    system.console.lock().unwrap().clear();
    assert!(!manager.save());
    assert_eq!(
        console(&system),
        "Resource Manager Warning: environment variable \"CSF_AppUserDefaults\" not set.  Cannot save resources.\n"
    );
    system.set_environment("CSF_AppUserDefaults", PathBuf::from("out"));
    system.console.lock().unwrap().clear();
    assert!(manager.save());
    assert!(system.has_file(&PathBuf::from("out/App")));
    assert!(system.files.lock().unwrap()[&PathBuf::from("out/App")].is_empty());
    assert!(console(&system).is_empty());
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

#[test]
fn user_defaults_override_reference_but_save_only_explicit_overrides() {
    let system = system_with("App", b"key:default\n");
    let mut catalog = ResourceManager::new("App", false, system.host()).unwrap();
    system.write_resource(&PathBuf::from("user"), "App", "key", "user");
    system.set_environment("CSF_AppUserDefaults", PathBuf::from("user"));
    assert!(catalog.save());
    assert!(system.files.lock().unwrap()[&PathBuf::from("user/App")].is_empty());
    system.write_resource(&PathBuf::from("user"), "App", "key", "user");
    catalog = ResourceManager::new("App", false, system.host()).unwrap();
    assert_eq!(catalog.value("key").unwrap().as_bytes(), b"user");
    for (key, value) in [
        ("z", "\\path"),
        ("a", " leading"),
        ("m", "\ttab"),
        ("empty", ""),
    ] {
        catalog.set_resource(key, value.as_bytes());
    }
    system.set_environment("CSF_AppUserDefaults", PathBuf::from("new/nested"));
    assert!(catalog.save());
    assert_eq!(
        system.files.lock().unwrap()[&PathBuf::from("new/nested/App")],
        b"a:\t\\ leading\nempty:\t\nm:\t\\\ttab\nz:\t\\\\path\n"
    );
    assert!(system.has_file(&PathBuf::from("new/nested/App")));
}
