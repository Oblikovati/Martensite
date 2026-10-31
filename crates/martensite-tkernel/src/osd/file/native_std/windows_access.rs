//! Ask Windows to check the effective token's requested file rights (#334).
#![forbid(unsafe_code)]
use super::AccessMode;
use std::{fs, io, os::windows::fs::OpenOptionsExt, path::Path};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_BACKUP_SEMANTICS, FILE_GENERIC_EXECUTE, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
};

pub(super) fn check(path: &Path, mode: AccessMode) -> io::Result<()> {
    if !(mode.read || mode.write || mode.execute) {
        return fs::metadata(path).map(|_| ());
    }
    let mask = (u32::from(mode.read) * FILE_GENERIC_READ)
        | (u32::from(mode.write) * FILE_GENERIC_WRITE)
        | (u32::from(mode.execute) * FILE_GENERIC_EXECUTE);
    // Open only: do not create or truncate. BACKUP_SEMANTICS enables directory
    // handles and does not bypass ACLs without explicit backup privileges.
    fs::OpenOptions::new()
        .access_mode(mask)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
        .map(drop)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::osd::{Protection, SingleProtection as Right, document_file};

    #[test]
    fn native_access_checks_honor_the_acl_instead_of_readonly_attributes() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("access");
        // SYSTEM may also be the effective token on a hosted runner (#344).
        // Every granted principal must omit execute for the denial to be meaningful.
        let rights = Protection::with_values(Right::Rwd, Right::Rwd, Right::None, Right::None);
        drop(document_file::exclusive_protected_file(&path, &rights).unwrap());
        assert!(
            check(
                &path,
                AccessMode {
                    read: true,
                    ..Default::default()
                }
            )
            .is_ok()
        );
        assert!(
            check(
                &path,
                AccessMode {
                    write: true,
                    ..Default::default()
                }
            )
            .is_ok()
        );
        assert!(
            check(
                &path,
                AccessMode {
                    execute: true,
                    ..Default::default()
                }
            )
            .is_err()
        );
        assert!(check(&path, AccessMode::default()).is_ok());
        assert!(check(&root.path().join("missing"), AccessMode::default()).is_err());
    }
}
