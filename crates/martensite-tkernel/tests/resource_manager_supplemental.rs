//! Parser and persistence edge cases of resource managers.
use martensite_tkernel::resource::ResourceManager;
use std::{path::PathBuf, sync::Arc};
#[path = "resource_manager/support.rs"]
#[allow(dead_code)]
mod support;
use support::FakeResourceSystem;
fn fixture(bytes: &[u8]) -> (Arc<FakeResourceSystem>, ResourceManager) {
    let system: Arc<FakeResourceSystem> = Arc::default();
    system.set_environment("CSF_AppDefaults", PathBuf::from("defaults"));
    system
        .files
        .lock()
        .unwrap()
        .insert(PathBuf::from("defaults/App"), bytes.to_vec());
    let catalog = ResourceManager::new("App", false, system.host()).unwrap();
    (system, catalog)
}
#[test]
fn parser_preserves_native_whitespace_escape_and_duplicate_rules() {
    let (_,catalog)=fixture(b"!ignored: yes\n  !key : kept\n bad line\n:invalid\n x : old\nx: new \r\nspace: \\ leading\ntab:\\\tleading\nslash:\\\\path\nother:\\n\nempty: \t\nlast: eof");
    for (key, value) in [
        ("  !key", "absent"),
        ("!key", "kept"),
        ("x", "new "),
        ("space", " leading"),
        ("tab", "\tleading"),
        ("slash", "\\path"),
        ("other", "\\n"),
        ("empty", ""),
        ("last", "eof"),
    ] {
        if value == "absent" {
            assert!(!catalog.find(key));
        } else {
            assert_eq!(catalog.value(key).unwrap().as_bytes(), value.as_bytes());
        }
    }
    assert!(!catalog.find("!ignored"));
    assert!(catalog.is_initialized());
}

#[test]
fn clone_copies_resource_maps_before_independent_updates() {
    let (_, original) = fixture(b"key: original\n");
    let mut copied = original.clone();
    copied.set_resource("key", b"copy");
    copied.set_resource("new", b"copy-only");
    assert_eq!(original.value("key").unwrap().as_bytes(), b"original");
    assert!(!original.find("new"));
    assert_eq!(copied.value("key").unwrap().as_bytes(), b"copy");
    assert_eq!(copied.value("new").unwrap().as_bytes(), b"copy-only");
}
