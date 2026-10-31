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
