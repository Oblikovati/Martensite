//! Native filesystem cases: run on macOS; cross-checking does not execute them.
use super::*;
use std::{
    os::unix::fs::symlink,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

struct AppleAccessFixture(PathBuf);

impl AppleAccessFixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "martensite-apple-access-{}-{nonce}",
            std::process::id()
        ));
        create_private_directory(&path).unwrap();
        Self(path)
    }

    fn file(&self, name: &str) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, name.as_bytes()).unwrap();
        path
    }

    fn grant_inheritance(&self) {
        set_acl(
            &self.0,
            "everyone allow read,readattr,readextattr,readsecurity,file_inherit,directory_inherit",
        );
    }
}

impl Drop for AppleAccessFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn set_acl(path: &Path, entry: &str) {
    assert!(
        Command::new("/bin/chmod")
            .arg("+a")
            .arg(entry)
            .arg(path)
            .status()
            .unwrap()
            .success()
    );
}

#[test]
fn inherited_grants_are_removed_before_stage_is_returned() {
    let fixture = AppleAccessFixture::new();
    fixture.grant_inheritance();
    let stage = fixture.0.join("private-stage");
    create_private_directory(&stage).unwrap();
    assert_eq!(fs::metadata(&stage).unwrap().mode() & 0o7777, 0o700);
    assert_eq!(
        ExtendedAcl::read(&stage).unwrap().encoded().unwrap(),
        ExtendedAcl::empty().unwrap().encoded().unwrap()
    );
    assert_eq!(fs::read_dir(&stage).unwrap().count(), 0);
    assert_eq!(
        create_private_directory(&stage).unwrap_err().kind(),
        io::ErrorKind::AlreadyExists
    );
}

#[test]
fn replacement_retains_deny_acl_owner_group_and_mode() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("new");
    fs::set_permissions(&destination, fs::Permissions::from_mode(0o640)).unwrap();
    set_acl(&destination, "everyone deny execute");
    let expected = AccessSnapshot::read(&destination).unwrap();
    // Recreated output must retain its ordinary writer policy before destination restoration.
    fs::remove_file(&staged).unwrap();
    fs::write(&staged, b"new").unwrap();
    let ordinary = fixture.file("ordinary-control");
    AccessSnapshot::read(&ordinary)
        .unwrap()
        .verify(&staged)
        .unwrap();
    preserve_access(&destination, &staged).unwrap();
    fs::rename(staged, &destination).unwrap();
    expected.verify(&destination).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"new");
}

#[test]
fn absent_target_obtains_final_parent_acl_and_group() {
    let fixture = AppleAccessFixture::new();
    fixture.grant_inheritance();
    let stage_directory = fixture.0.join("stage");
    create_private_directory(&stage_directory).unwrap();
    let staged = stage_directory.join("new");
    fs::write(&staged, b"private model").unwrap();
    let reference = fixture.file("ordinary-reference");
    let expected = AccessSnapshot::read(&reference).unwrap();
    preserve_access(&fixture.0.join("absent"), &staged).unwrap();
    expected.verify(&staged).unwrap();
    assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 2);
}

#[test]
fn privileged_destination_is_rejected_without_changing_bytes_or_policy() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("new");
    fs::set_permissions(&destination, fs::Permissions::from_mode(0o4640)).unwrap();
    let before = fs::metadata(&destination).unwrap();
    let error = preserve_access(&destination, &staged).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    assert_eq!(fs::read(&destination).unwrap(), b"original");
    assert_eq!(fs::metadata(destination).unwrap().mode(), before.mode());
}

#[test]
fn unicode_parent_symlink_uses_resolved_parent_policy() {
    let fixture = AppleAccessFixture::new();
    let parent = fixture.0.join("模型-é");
    fs::create_dir(&parent).unwrap();
    let alias = fixture.0.join("alias");
    symlink(&parent, &alias).unwrap();
    let reference = parent.join("reference");
    fs::write(&reference, b"").unwrap();
    let expected = AccessSnapshot::read(&reference).unwrap();
    let staged = fixture.file("replacement");
    preserve_access(&alias.join("模型"), &staged).unwrap();
    expected.verify(&staged).unwrap();
}

#[test]
fn access_attributes_are_copied_without_content_attributes() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("replacement");
    xattr::set(
        &destination,
        "com.apple.quarantine",
        b"0081;12345678;Martensite;",
    )
    .unwrap();
    xattr::set(&destination, "com.apple.ResourceFork", b"old content").unwrap();
    preserve_access(&destination, &staged).unwrap();
    assert_eq!(
        xattr::get(&destination, "com.apple.quarantine").unwrap(),
        xattr::get(&staged, "com.apple.quarantine").unwrap()
    );
    assert_eq!(xattr::get(&staged, "com.apple.ResourceFork").unwrap(), None);
}

#[test]
fn missing_stage_preserves_native_error_and_original_document() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let expected = AccessSnapshot::read(&destination).unwrap();
    let error = preserve_access(&destination, &fixture.0.join("missing")).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(error.raw_os_error().is_some());
    expected.verify(&destination).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"original");
}

#[test]
fn new_document_refuses_custom_writer_mode_without_publishing() {
    let fixture = AppleAccessFixture::new();
    let staged = fixture.file("replacement");
    let ordinary = fs::metadata(&staged).unwrap().mode() & 0o777;
    fs::set_permissions(&staged, fs::Permissions::from_mode(ordinary ^ 0o040)).unwrap();
    let destination = fixture.0.join("absent");
    assert_eq!(
        preserve_access(&destination, &staged).unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert!(!destination.exists());
    assert_eq!(fs::read(&staged).unwrap(), b"replacement");
}

#[test]
fn new_document_refuses_custom_writer_acl_without_publishing() {
    let fixture = AppleAccessFixture::new();
    let staged = fixture.file("replacement");
    set_acl(&staged, "everyone deny execute");
    let destination = fixture.0.join("absent");
    assert_eq!(
        preserve_access(&destination, &staged).unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert!(!destination.exists());
}

#[test]
fn denied_stage_policy_keeps_original_document_unchanged() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("replacement");
    let expected = AccessSnapshot::read(&destination).unwrap();
    set_stage_immutable(&staged, true);
    let result = preserve_access(&destination, &staged);
    set_stage_immutable(&staged, false);
    let error = result.unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    let reason = error.to_string();
    assert!(reason.contains("unsupported or changed access policy"));
    assert!(reason.contains(staged.to_str().unwrap()));
    expected.verify(&destination).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"original");
}

#[test]
fn target_alias_reads_referent_access_attributes() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    xattr::set(
        &destination,
        "com.apple.quarantine",
        b"0081;12345678;Martensite;",
    )
    .unwrap();
    let alias = fixture.0.join("alias");
    symlink(&destination, &alias).unwrap();
    let staged = fixture.file("replacement");
    preserve_access(&alias, &staged).unwrap();
    AccessSnapshot::read(&destination)
        .unwrap()
        .verify(&staged)
        .unwrap();
    assert!(
        fs::symlink_metadata(alias)
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn replacement_preserves_alternative_supplementary_group_when_available() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("replacement");
    let gid = fs::metadata(&destination).unwrap().gid();
    let groups = Command::new("/usr/bin/id").arg("-G").output().unwrap();
    assert!(groups.status.success());
    let alternative = String::from_utf8(groups.stdout)
        .unwrap()
        .split_whitespace()
        .filter_map(|group| group.parse::<u32>().ok())
        .find(|group| *group != gid);
    let Some(group) = alternative else {
        eprintln!(
            "different supplementary-group case excluded: only the primary group is available"
        );
        return;
    };
    chown(&destination, None, Some(group)).unwrap();
    preserve_access(&destination, &staged).unwrap();
    assert_eq!(fs::metadata(staged).unwrap().gid(), group);
}

#[test]
fn absent_acl_normalizes_to_empty_and_missing_file_retains_enoent() {
    let fixture = AppleAccessFixture::new();
    let ordinary = fixture.file("ordinary");
    assert_eq!(
        ExtendedAcl::read(&ordinary).unwrap().encoded().unwrap(),
        ExtendedAcl::empty().unwrap().encoded().unwrap()
    );
    let result = ExtendedAcl::read(&fixture.0.join("missing"));
    assert_eq!(result.err().unwrap().raw_os_error(), Some(libc::ENOENT));
}

#[test]
fn unknown_private_access_attribute_fails_before_publication() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("replacement");
    xattr::set(
        &destination,
        "com.apple.private.martensite-test-policy",
        b"unknown",
    )
    .unwrap();
    let error = preserve_access(&destination, &staged).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::Unsupported);
    assert_eq!(fs::read(destination).unwrap(), b"original");
}

#[test]
fn existing_destination_does_not_widen_restrictive_writer_policy() {
    if isolated_default_policy_case() {
        return;
    }
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("replacement");
    assert_eq!(fs::metadata(&staged).unwrap().mode() & 0o777, 0o644);
    fs::set_permissions(&destination, fs::Permissions::from_mode(0o644)).unwrap();
    fs::set_permissions(&staged, fs::Permissions::from_mode(0o600)).unwrap();
    let original = AccessSnapshot::read(&destination).unwrap();
    let writer = AccessSnapshot::read(&staged).unwrap();
    let error = preserve_access(&destination, &staged).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    original.verify(&destination).unwrap();
    writer.verify(&staged).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"original");
    assert_eq!(fs::read(staged).unwrap(), b"replacement");
}

fn isolated_default_policy_case() -> bool {
    const CHILD: &str = "MARTENSITE_EXISTING_WRITER_POLICY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        return false;
    }
    let executable = std::env::current_exe().unwrap();
    let qualified = concat!(
        module_path!(),
        "::existing_destination_does_not_widen_restrictive_writer_policy"
    );
    let (_, test) = qualified.split_once("::").unwrap();
    let mut isolated = Command::new("/bin/sh");
    let script = r#"umask 022; exec "$1" --exact "$2""#;
    isolated.args(["-c", script, "document-policy"]);
    isolated.arg(executable).arg(test).env(CHILD, "1");
    assert!(isolated.status().unwrap().success());
    true
}

#[test]
fn existing_destination_refuses_custom_writer_acl_without_changes() {
    let fixture = AppleAccessFixture::new();
    let destination = fixture.file("original");
    let staged = fixture.file("replacement");
    set_acl(&staged, "everyone deny execute");
    let original = AccessSnapshot::read(&destination).unwrap();
    let writer = AccessSnapshot::read(&staged).unwrap();
    let error = preserve_access(&destination, &staged).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    original.verify(&destination).unwrap();
    writer.verify(&staged).unwrap();
    assert_eq!(fs::read(destination).unwrap(), b"original");
    assert_eq!(fs::read(staged).unwrap(), b"replacement");
}

fn set_stage_immutable(staged: &Path, immutable: bool) {
    let flag = if immutable { "uchg" } else { "nouchg" };
    let status = Command::new("/usr/bin/chflags")
        .arg(flag)
        .arg(staged)
        .status()
        .unwrap();
    assert!(
        status.success(),
        "chflags {flag} failed for {}",
        staged.display()
    );
}
