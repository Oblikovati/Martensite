//! Resource manager constructors, traces, lookups, assignments and paths.
use super::{
    fixtures::{console, system_with},
    support::FakeResourceSystem,
};
use martensite_tkernel::resource::ResourceManager;
use std::{path::PathBuf, sync::Arc};

#[test]
fn verbose_named_constructor_reports_loads_and_missing_variables() {
    let _serial = super::serial();
    let system = system_with("App", b"a: 1\nno colon\n");
    let manager = ResourceManager::new("App", true, system.host()).unwrap();
    assert!(manager.is_initialized());
    assert_eq!(
        console(&system),
        "Resource Manager: Syntax error at line 2 in file : App\n\
         Resource Manager: Reference file \"App\" loaded\n\
         Resource Manager Warning: Environment variable \"CSF_AppUserDefaults\" not set.\n"
    );
}
#[test]
fn resource_verbose_variable_forces_verbose_and_missing_file_is_reported() {
    let _serial = super::serial();
    let system: Arc<FakeResourceSystem> = Arc::default();
    system.set_environment("CSF_ResourceVerbose", PathBuf::from("1"));
    system.set_environment("CSF_AppDefaults", PathBuf::from("nowhere"));
    let manager = ResourceManager::new("App", false, system.host()).unwrap();
    assert!(!manager.is_initialized());
    assert!(console(&system).starts_with(
        "Resource Manager Warning: Cannot read file \"App\". File not found or permission denied.\n"
    ));
}
#[test]
fn fgets_chunks_hide_bytes_after_nul_and_reject_a_leading_nul() {
    let _serial = super::serial();
    let system = system_with(
        "App",
        b"a\0x\nnext: line\nlong.key.name : long value text\n",
    );
    let manager = ResourceManager::new("App", false, system.host()).unwrap();
    assert!(!manager.find("next"));
    assert_eq!(manager.value("anext").unwrap().as_bytes(), b"line");
    assert_eq!(
        manager.value("long.key.name").unwrap().as_bytes(),
        b"long value text"
    );
    let failing = system_with("Bad", b"\0start\n");
    let error = ResourceManager::new("Bad", false, failing.host())
        .err()
        .unwrap();
    assert_eq!(error.exception_type(), "Standard_OutOfRange");
}
