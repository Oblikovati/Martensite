//! Owned files exercise differing groups and a fixed Linux access ACL.
use super::super::{create_private_document_directory, preserve_document_access};
use super::*;
use std::{
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

struct DocumentAccessFixture {
    directory: PathBuf,
    destination: PathBuf,
    staged: PathBuf,
}
impl DocumentAccessFixture {
    fn new(name: &str) -> Self {
        let directory = fixture_directory(name);
        create_private_document_directory(&directory).unwrap();
        let destination = directory.join("document");
        let private = directory.join("private");
        create_private_document_directory(&private).unwrap();
        let staged = private.join("replacement");
        fs::write(&destination, b"original bytes").unwrap();
        fs::write(&staged, b"completed replacement").unwrap();
        Self {
            directory,
            destination,
            staged,
        }
    }
    fn publish(&self) -> io::Result<()> {
        preserve_document_access(&self.destination, &self.staged)?;
        fs::rename(&self.staged, &self.destination)
    }
}
impl Drop for DocumentAccessFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn fixture_directory(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "martensite-document-{}-{name}-{nonce}",
        std::process::id()
    ))
}

fn alternative_group(current_gid: u32) -> Option<u32> {
    let status = fs::read_to_string("/proc/self/status").unwrap();
    let groups = status.lines().find(|line| line.starts_with("Groups:"))?;
    groups
        .split_whitespace()
        .skip(1)
        .filter_map(|value| value.parse::<u32>().ok())
        .find(|group| *group != current_gid)
}

#[test]
fn published_document_preserves_a_different_supplementary_group() {
    let fixture = DocumentAccessFixture::new("group");
    let before = fs::metadata(&fixture.destination).unwrap();
    let Some(group) = alternative_group(before.gid()) else {
        eprintln!("no different supplementary group available; differing-group case excluded");
        return;
    };
    chown(&fixture.destination, None, Some(group)).unwrap();
    fs::set_permissions(&fixture.destination, fs::Permissions::from_mode(0o640)).unwrap();
    fixture.publish().unwrap();
    let after = fs::metadata(&fixture.destination).unwrap();
    assert_eq!(after.uid(), before.uid());
    assert_eq!(after.gid(), group);
    assert_eq!(after.mode() & 0o7777, 0o640);
    assert_eq!(
        fs::read(&fixture.destination).unwrap(),
        b"completed replacement"
    );
}

fn named_access_acl(owner: u32) -> Vec<u8> {
    let mut encoded = 2_u32.to_le_bytes().to_vec();
    for (tag, permission, id) in [
        (1_u16, 6_u16, u32::MAX),
        (2, 0, owner.wrapping_add(1)),
        (4, 4, u32::MAX),
        (16, 4, u32::MAX),
        (32, 0, u32::MAX),
    ] {
        encoded.extend(tag.to_le_bytes());
        encoded.extend(permission.to_le_bytes());
        encoded.extend(id.to_le_bytes());
    }
    encoded
}

#[test]
fn published_document_retains_named_access_acl_bytes() {
    let fixture = DocumentAccessFixture::new("acl");
    let expected = named_access_acl(fs::metadata(&fixture.destination).unwrap().uid());
    xattr::set(&fixture.destination, ACCESS_ATTRIBUTES[0], &expected).unwrap();
    fixture.publish().unwrap();
    assert_eq!(
        xattr::get(&fixture.destination, ACCESS_ATTRIBUTES[0]).unwrap(),
        Some(expected)
    );
    assert_eq!(
        fs::read(&fixture.destination).unwrap(),
        b"completed replacement"
    );
}

#[test]
fn inherited_staging_acl_is_removed_when_original_has_none() {
    let fixture = DocumentAccessFixture::new("inherited-acl");
    let inherited = named_access_acl(fs::metadata(&fixture.destination).unwrap().uid());
    // This ACL belongs to the private directory's default policy, not a custom writer.
    let stage_parent = fixture.staged.parent().unwrap();
    xattr::set(stage_parent, "system.posix_acl_default", &inherited).unwrap();
    xattr::set(&fixture.staged, ACCESS_ATTRIBUTES[0], &inherited).unwrap();
    fixture.publish().unwrap();
    assert_eq!(
        xattr::get(&fixture.destination, ACCESS_ATTRIBUTES[0]).unwrap(),
        None
    );
    assert_eq!(
        fs::read(&fixture.destination).unwrap(),
        b"completed replacement"
    );
}

#[test]
fn privileged_mode_is_refused_before_publication() {
    let fixture = DocumentAccessFixture::new("privileged");
    fs::set_permissions(&fixture.destination, fs::Permissions::from_mode(0o4640)).unwrap();
    let cause = fixture.publish().unwrap_err();
    assert_eq!(cause.kind(), io::ErrorKind::PermissionDenied);
    assert!(cause.to_string().contains("privileged mode"));
    assert_eq!(
        fs::metadata(&fixture.destination).unwrap().mode() & 0o7777,
        0o4640
    );
    assert_eq!(fs::read(&fixture.destination).unwrap(), b"original bytes");
}

#[test]
fn unrecognized_security_policy_is_refused_without_copying_privileges() {
    use std::ffi::OsStr;
    let unknown = validate_attribute_name(OsStr::new("system.nfs4_acl")).unwrap_err();
    assert_eq!(unknown.kind(), io::ErrorKind::PermissionDenied);
    assert!(unknown.to_string().contains("system.nfs4_acl"));
    for excluded in ["security.capability", "security.ima", "security.evm"] {
        assert!(validate_attribute_name(OsStr::new(excluded)).is_ok());
    }
}

#[test]
fn private_directory_is_exclusive_and_grants_only_owner_access() {
    let fixture = DocumentAccessFixture::new("private-directory");
    let mode = fs::metadata(&fixture.directory).unwrap().mode() & 0o777;
    assert_eq!(mode, 0o700);
    let failure = create_private_document_directory(&fixture.directory).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(fs::read(&fixture.destination).unwrap(), b"original bytes");
}

#[test]
fn absent_destination_receives_the_final_parent_default_acl() {
    let fixture = DocumentAccessFixture::new("absent-destination");
    fs::remove_file(&fixture.destination).unwrap();
    let inherited = named_access_acl(fs::metadata(&fixture.directory).unwrap().uid());
    xattr::set(&fixture.directory, "system.posix_acl_default", &inherited).unwrap();
    preserve_document_access(&fixture.destination, &fixture.staged).unwrap();
    assert_eq!(
        xattr::get(&fixture.staged, ACCESS_ATTRIBUTES[0]).unwrap(),
        Some(inherited)
    );
    assert_eq!(fs::metadata(&fixture.staged).unwrap().mode() & 0o777, 0o640);
    assert_eq!(fs::read(&fixture.staged).unwrap(), b"completed replacement");
    assert!(!fixture.destination.exists());
    assert_eq!(fs::read_dir(&fixture.directory).unwrap().count(), 1);
}

#[test]
fn staged_directory_and_symbolic_link_are_refused_before_policy_changes() {
    let fixture = DocumentAccessFixture::new("nonregular-output");
    fs::remove_file(&fixture.staged).unwrap();
    fs::create_dir(&fixture.staged).unwrap();
    let directory_error =
        preserve_document_access(&fixture.destination, &fixture.staged).unwrap_err();
    assert_eq!(directory_error.kind(), io::ErrorKind::InvalidInput);
    fs::remove_dir(&fixture.staged).unwrap();
    std::os::unix::fs::symlink(&fixture.destination, &fixture.staged).unwrap();
    let link_error = preserve_document_access(&fixture.destination, &fixture.staged).unwrap_err();
    assert_eq!(link_error.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(fs::read(&fixture.destination).unwrap(), b"original bytes");
}

#[test]
fn restrictive_writer_policy_is_not_widened_for_an_absent_destination() {
    let fixture = DocumentAccessFixture::new("restrictive-writer");
    fs::remove_file(&fixture.destination).unwrap();
    install_ordinary_creation_policy(&fixture);
    fs::set_permissions(&fixture.staged, fs::Permissions::from_mode(0o600)).unwrap();
    let failure = preserve_document_access(&fixture.destination, &fixture.staged).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::PermissionDenied);
    assert!(!fixture.destination.exists());
    assert_eq!(fs::metadata(&fixture.staged).unwrap().mode() & 0o777, 0o600);
    assert_eq!(fs::read(&fixture.staged).unwrap(), b"completed replacement");
}

#[test]
fn destination_alias_is_refused_without_dropping_referent_acl() {
    let fixture = DocumentAccessFixture::new("destination-alias");
    let expected = named_access_acl(fs::metadata(&fixture.destination).unwrap().uid());
    xattr::set(&fixture.destination, ACCESS_ATTRIBUTES[0], &expected).unwrap();
    let alias = fixture.directory.join("alias");
    std::os::unix::fs::symlink(&fixture.destination, &alias).unwrap();
    let before = AccessSnapshot::read(&fixture.staged).unwrap();
    let failure = preserve_document_access(&alias, &fixture.staged).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::InvalidInput);
    before.verify(&fixture.staged).unwrap();
    assert_eq!(
        xattr::get(&fixture.destination, ACCESS_ATTRIBUTES[0]).unwrap(),
        Some(expected)
    );
    assert_eq!(fs::read(&fixture.destination).unwrap(), b"original bytes");
    assert_eq!(fs::read_link(alias).unwrap(), fixture.destination);
}

fn install_ordinary_creation_policy(fixture: &DocumentAccessFixture) {
    install_creation_policy(fixture, 4);
}

fn install_creation_policy(fixture: &DocumentAccessFixture, public_read: u16) {
    let mut acl = 2_u32.to_le_bytes().to_vec();
    for (tag, permission) in [(1_u16, 6_u16), (4, public_read), (32, public_read)] {
        acl.extend(tag.to_le_bytes());
        acl.extend(permission.to_le_bytes());
        acl.extend(u32::MAX.to_le_bytes());
    }
    xattr::set(&fixture.directory, "system.posix_acl_default", &acl).unwrap();
    let stage_parent = fixture.staged.parent().unwrap();
    xattr::set(stage_parent, "system.posix_acl_default", &acl).unwrap();
    let control = stage_parent.join("ordinary-control");
    fs::write(&control, b"").unwrap();
    let expected = 0o600 | u32::from(public_read) << 3 | u32::from(public_read);
    assert_eq!(fs::metadata(&control).unwrap().mode() & 0o777, expected);
    fs::remove_file(control).unwrap();
}

#[test]
fn existing_destination_does_not_widen_restrictive_writer_policy() {
    let fixture = DocumentAccessFixture::new("existing-restrictive-writer");
    install_ordinary_creation_policy(&fixture);
    fs::set_permissions(&fixture.destination, fs::Permissions::from_mode(0o644)).unwrap();
    fs::set_permissions(&fixture.staged, fs::Permissions::from_mode(0o600)).unwrap();
    let original = AccessSnapshot::read(&fixture.destination).unwrap();
    let writer_policy = AccessSnapshot::read(&fixture.staged).unwrap();
    let failure = preserve_document_access(&fixture.destination, &fixture.staged).unwrap_err();
    assert_eq!(failure.kind(), io::ErrorKind::PermissionDenied);
    original.verify(&fixture.destination).unwrap();
    writer_policy.verify(&fixture.staged).unwrap();
    assert_eq!(fs::read(&fixture.destination).unwrap(), b"original bytes");
    assert_eq!(fs::read(&fixture.staged).unwrap(), b"completed replacement");
}

#[test]
fn ordinary_private_default_is_compatible_with_existing_public_mode() {
    let fixture = DocumentAccessFixture::new("ordinary-private-default");
    install_creation_policy(&fixture, 0);
    fs::set_permissions(&fixture.destination, fs::Permissions::from_mode(0o644)).unwrap();
    fs::set_permissions(&fixture.staged, fs::Permissions::from_mode(0o600)).unwrap();
    let original = AccessSnapshot::read(&fixture.destination).unwrap();
    preserve_document_access(&fixture.destination, &fixture.staged).unwrap();
    original.verify(&fixture.destination).unwrap();
    original.verify(&fixture.staged).unwrap();
    assert_eq!(fs::read(&fixture.destination).unwrap(), b"original bytes");
    assert_eq!(fs::read(&fixture.staged).unwrap(), b"completed replacement");
}
