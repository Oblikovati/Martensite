//! Explicit file protections share the existing owned security-descriptor boundary (#334).
use super::*;
use crate::osd::{Protection, SingleProtection};
use windows_sys::Win32::Security::Authorization::{
    BuildTrusteeWithSidW, GetEffectiveRightsFromAclW, TRUSTEE_W,
};
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, FILE_EXECUTE, FILE_GENERIC_EXECUTE, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_READ_DATA, FILE_WRITE_DATA,
};

fn principals(descriptor: &LocalAllocation) -> io::Result<[String; 4]> {
    let owner = descriptor_text(descriptor, OWNER_SECURITY_INFORMATION)?;
    let group = descriptor_text(descriptor, GROUP_SECURITY_INFORMATION)?;
    Ok([
        "SY".into(),
        owner
            .strip_prefix("O:")
            .ok_or_else(|| io::Error::from_raw_os_error(1336))?
            .into(),
        group
            .strip_prefix("G:")
            .ok_or_else(|| io::Error::from_raw_os_error(1336))?
            .into(),
        "WD".into(),
    ])
}

pub(crate) fn read_protection(path: &Path) -> io::Result<Protection> {
    let descriptor = read_section(
        path,
        OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
    )?;
    let names = principals(&descriptor)?;
    let mut rights = [SingleProtection::None; 4];
    for (slot, name) in rights.iter_mut().zip(names) {
        *slot = principal_rights(&descriptor, &name)?;
    }
    Ok(Protection::with_values(
        rights[0], rights[1], rights[2], rights[3],
    ))
}

fn principal_rights(descriptor: &LocalAllocation, principal: &str) -> io::Result<SingleProtection> {
    let mut acl = null_mut();
    let (mut present, mut defaulted) = (0, 0);
    // SAFETY: the Windows-returned descriptor owns the ACL; every output slot is live.
    win32_success(unsafe {
        GetSecurityDescriptorDacl(descriptor.0, &mut present, &mut acl, &mut defaulted)
    })?;
    if present == 0 || acl.is_null() {
        return Ok(SingleProtection::Rwxd);
    }
    let sid_descriptor = parse_descriptor(&format!("O:{principal}"))?;
    let mut sid = null_mut();
    // SAFETY: the parsed descriptor is valid and owns its owner SID throughout this call.
    win32_success(unsafe {
        GetSecurityDescriptorOwner(sid_descriptor.0, &mut sid, &mut defaulted)
    })?;
    let mask = effective_mask(acl, sid)?;
    Ok(from_access_mask(mask))
}

fn effective_mask(acl: *mut ACL, sid: PSID) -> io::Result<u32> {
    let mut trustee = TRUSTEE_W::default();
    let mut mask = 0;
    // SAFETY: callers pass an ACL and SID borrowed from live Windows-validated
    // descriptor allocations. Neither API takes ownership or retains the pointers.
    let status = unsafe {
        BuildTrusteeWithSidW(&mut trustee, sid);
        GetEffectiveRightsFromAclW(acl, &trustee, &mut mask)
    };
    if status != 0 {
        return Err(io::Error::from_raw_os_error(status as i32));
    }
    Ok(mask)
}

fn from_access_mask(mask: u32) -> SingleProtection {
    let mut bits = 0usize;
    for (index, right) in [FILE_READ_DATA, FILE_WRITE_DATA, FILE_EXECUTE, DELETE]
        .into_iter()
        .enumerate()
    {
        if mask & right != 0 {
            bits |= 1 << index;
        }
    }
    SingleProtection::ALL[bits]
}

pub(super) fn to_access_mask(rights: SingleProtection) -> u32 {
    let mut mask = 0;
    for (index, right) in [
        FILE_GENERIC_READ,
        FILE_GENERIC_WRITE,
        FILE_GENERIC_EXECUTE,
        DELETE,
    ]
    .into_iter()
    .enumerate()
    {
        if rights as usize & (1 << index) != 0 {
            mask |= right;
        }
    }
    mask
}

pub(crate) fn write_protection(path: &Path, rights: &Protection) -> io::Result<()> {
    let previous = read_section(
        path,
        OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION,
    )?;
    let (system, user, group, world) = rights.values();
    let entries = principals(&previous)?
        .into_iter()
        .zip([system, user, group, world])
        .map(|(sid, right)| format!("(A;;0x{:x};;;{sid})", to_access_mask(right)))
        .collect::<String>();
    let descriptor = parse_descriptor(&format!("D:P{entries}"))?;
    apply_section(
        path,
        DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
        &descriptor,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_protection_masks_round_trip_without_losing_delete() {
        for &rights in SingleProtection::ALL {
            assert_eq!(from_access_mask(to_access_mask(rights)), rights);
        }
    }

    #[test]
    fn native_acl_preserves_separate_owner_and_world_rights() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let desired = Protection::with_values(
            SingleProtection::Rwxd,
            SingleProtection::Rwxd,
            SingleProtection::Rx,
            SingleProtection::R,
        );
        write_protection(file.path(), &desired).unwrap();
        let observed = read_protection(file.path()).unwrap();
        assert_eq!(observed.user(), SingleProtection::Rwxd);
        assert_eq!(observed.world(), SingleProtection::R);
    }

    #[test]
    fn missing_path_is_reported_as_an_error() {
        let dir = tempfile::tempdir().unwrap();
        assert!(read_protection(&dir.path().join("missing")).is_err());
        assert!(write_protection(&dir.path().join("missing"), &Protection::new()).is_err());
    }
}
