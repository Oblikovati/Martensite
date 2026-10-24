use super::*;

fn manifest(version: &str) -> PackageManifest {
    PackageManifest::parse(
        &format!("[package]\nname='a'\nversion='{version}'"),
        &Table::new(),
    )
    .unwrap()
    .unwrap()
}

#[test]
fn mixed_versions_allow_changed_package_and_unchanged_dependency() {
    let previous = manifest("1.0.0");
    assert!(check_package_version(&manifest("1.0.1"), Some(&previous), true).is_ok());
    assert!(check_package_version(&previous, Some(&previous), false).is_ok());
    assert!(check_package_version(&manifest("0.1.0"), None, true).is_ok());
}

#[test]
fn published_versions_are_immutable_and_gratuitous_bumps_are_refused() {
    let previous = manifest("1.0.0");
    for (version, changed) in [
        ("1.0.0", true),
        ("0.9.0", true),
        ("1.0.1", false),
        ("0.9.0", false),
    ] {
        let error =
            check_package_version(&manifest(version), Some(&previous), changed).unwrap_err();
        assert!(error.to_string().contains("package \"a\" version 1.0.0 ->"));
    }
}

#[test]
fn only_own_source_or_substantive_manifest_changes_require_new_version() {
    let previous = manifest("1.0.0");
    let current = manifest("1.0.1");
    let files = [ChangedFile {
        path: "crates/a/Cargo.toml".into(),
        deleted: false,
        untracked: false,
    }];
    assert!(!package_changed(
        &current,
        Some(&previous),
        "crates/a",
        &files
    ));
    let files = [ChangedFile {
        path: "crates/a/src/lib.rs".into(),
        ..files[0].clone()
    }];
    assert!(package_changed(
        &current,
        Some(&previous),
        "crates/a",
        &files
    ));
    assert!(package_changed(&current, None, "crates/a", &[]));
}

#[test]
fn workspace_package_fields_are_part_of_owned_source_comparison() {
    let fields = workspace_fields("[workspace.package]\nedition='2024'").unwrap();
    assert_eq!(fields["edition"].as_str(), Some("2024"));
}
