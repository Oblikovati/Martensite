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
        ("1.0.0+rebuilt", true),
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
    let files = [changed_file("crates/a/Cargo.toml")];
    assert!(!package_changed(
        &current,
        Some(&previous),
        "crates/a",
        &files
    ));
    let files = [changed_file("crates/a/src/lib.rs")];
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

fn changed_file(path: &str) -> ChangedFile {
    ChangedFile {
        path: path.into(),
        deleted: false,
        untracked: false,
    }
}

#[test]
fn removed_package_cannot_reuse_an_older_published_version() {
    use crate::{context::tests::context_at, fake_runner::RecordingRunner};
    let root = tempfile::tempdir().unwrap();
    let runner = RecordingRunner::default()
        .with_stdout(
            "git rev-parse --show-toplevel",
            &root.path().display().to_string(),
        )
        .with_stdout("git show v1.0.0:Cargo.toml", "[workspace]")
        .with_stdout("git show v1.1.0:Cargo.toml", "[workspace]")
        .with_stdout(
            "git ls-tree -r --name-only -z v1.0.0",
            "crates/a/Cargo.toml\0",
        )
        .with_stdout(
            "git show v1.0.0:crates/a/Cargo.toml",
            "[package]\nname='a'\nversion='1.0.0'",
        );
    let tags = vec!["v1.0.0".into(), "v1.1.0".into()];
    let published = published_manifests(
        &context_at(&runner, root.path()),
        &tags,
        &Version::new(1, 2, 0),
    )
    .unwrap();
    let previous = published.get("a").unwrap();
    assert!(check_package_version(&manifest("1.0.0"), Some(previous), true).is_err());
    assert!(check_package_version(&manifest("1.0.1"), Some(previous), true).is_ok());
}
