//! Owned files exercise differing groups and a fixed Linux access ACL.
use super::*;
use crate::cdf::staged_store::{StagedStore, tests::TemporaryDestination};
use std::os::unix::fs::PermissionsExt;

fn prepared_store() -> (TemporaryDestination, StagedStore) {
    let destination = TemporaryDestination::new();
    fs::write(&destination.file, b"original bytes").unwrap();
    let stage = StagedStore::new(&destination.file).unwrap();
    fs::write(&stage.file, b"completed replacement").unwrap();
    (destination, stage)
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
    let (destination, stage) = prepared_store();
    let before = fs::metadata(&destination.file).unwrap();
    let Some(group) = alternative_group(before.gid()) else {
        eprintln!("no different supplementary group available; differing-group case excluded");
        return;
    };
    chown(&destination.file, None, Some(group)).unwrap();
    fs::set_permissions(&destination.file, fs::Permissions::from_mode(0o640)).unwrap();
    stage.commit().unwrap();
    let after = fs::metadata(&destination.file).unwrap();
    assert_eq!(after.uid(), before.uid());
    assert_eq!(after.gid(), group);
    assert_eq!(after.mode() & 0o7777, 0o640);
    assert_eq!(
        fs::read(&destination.file).unwrap(),
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
    let (destination, stage) = prepared_store();
    let expected = named_access_acl(fs::metadata(&destination.file).unwrap().uid());
    xattr::set(&destination.file, ACCESS_ATTRIBUTES[0], &expected).unwrap();
    stage.commit().unwrap();
    assert_eq!(
        xattr::get(&destination.file, ACCESS_ATTRIBUTES[0]).unwrap(),
        Some(expected)
    );
    assert_eq!(
        fs::read(&destination.file).unwrap(),
        b"completed replacement"
    );
}

#[test]
fn inherited_staging_acl_is_removed_when_original_has_none() {
    let (destination, stage) = prepared_store();
    let inherited = named_access_acl(fs::metadata(&destination.file).unwrap().uid());
    xattr::set(&stage.file, ACCESS_ATTRIBUTES[0], &inherited).unwrap();
    stage.commit().unwrap();
    assert_eq!(
        xattr::get(&destination.file, ACCESS_ATTRIBUTES[0]).unwrap(),
        None
    );
    assert_eq!(
        fs::read(&destination.file).unwrap(),
        b"completed replacement"
    );
}

#[test]
fn privileged_mode_is_refused_before_publication() {
    let (destination, stage) = prepared_store();
    fs::set_permissions(&destination.file, fs::Permissions::from_mode(0o4640)).unwrap();
    let cause = stage.commit().unwrap_err();
    assert_eq!(cause.kind(), io::ErrorKind::PermissionDenied);
    assert!(cause.to_string().contains("privileged mode"));
    assert_eq!(
        fs::metadata(&destination.file).unwrap().mode() & 0o7777,
        0o4640
    );
    assert_eq!(fs::read(&destination.file).unwrap(), b"original bytes");
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
