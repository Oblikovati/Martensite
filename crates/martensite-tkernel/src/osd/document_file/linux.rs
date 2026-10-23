//! Retains access policy without transplanting capabilities or content signatures (#245).
use std::{
    fs, io,
    os::unix::fs::{MetadataExt, chown},
    path::Path,
};

const ACCESS_ATTRIBUTES: [&str; 3] = [
    "system.posix_acl_access",
    "security.selinux",
    "security.SMACK64",
];

#[path = "linux/new_access.rs"]
mod new_access;

struct AccessSnapshot {
    metadata: fs::Metadata,
    attributes: [Option<Vec<u8>>; 3],
}

pub(super) fn create_private_directory(directory: &Path) -> io::Result<()> {
    super::unix::create_private_directory(directory)
}

pub(super) fn preserve_access(destination: &Path, staged: &Path) -> io::Result<()> {
    new_access::verify_default_staged_policy(staged)?;
    let snapshot = match AccessSnapshot::read(destination) {
        Ok(snapshot) => snapshot,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            new_access::snapshot_for_new_destination(destination)?
        }
        Err(error) => return Err(error),
    };
    reject_privileged_mode(destination, snapshot.metadata.mode())?;
    chown(
        staged,
        Some(snapshot.metadata.uid()),
        Some(snapshot.metadata.gid()),
    )?;
    snapshot.copy_attributes(staged)?;
    fs::set_permissions(staged, snapshot.metadata.permissions())?;
    snapshot.verify(staged)
}

impl AccessSnapshot {
    fn read(file: &Path) -> io::Result<Self> {
        for name in xattr::list(file)? {
            validate_attribute_name(&name)?;
        }
        Ok(Self {
            metadata: fs::metadata(file)?,
            attributes: [
                xattr::get(file, ACCESS_ATTRIBUTES[0])?,
                xattr::get(file, ACCESS_ATTRIBUTES[1])?,
                xattr::get(file, ACCESS_ATTRIBUTES[2])?,
            ],
        })
    }

    fn copy_attributes(&self, staged: &Path) -> io::Result<()> {
        for (name, expected) in ACCESS_ATTRIBUTES.iter().zip(&self.attributes) {
            let actual = xattr::get(staged, name)?;
            if actual == *expected {
                continue;
            }
            match expected {
                Some(value) => xattr::set(staged, name, value)?,
                None => xattr::remove(staged, name)?,
            }
        }
        Ok(())
    }

    fn verify(&self, staged: &Path) -> io::Result<()> {
        let actual = Self::read(staged)?;
        let ownership_matches = self.metadata.uid() == actual.metadata.uid()
            && self.metadata.gid() == actual.metadata.gid();
        let mode_matches = self.metadata.mode() == actual.metadata.mode();
        if ownership_matches && mode_matches && self.attributes == actual.attributes {
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!(
                "staged file {} changed access policy; expected original owner, group, mode and access attributes",
                staged.display(),
            ),
        ))
    }
}

fn reject_privileged_mode(destination: &Path, mode: u32) -> io::Result<()> {
    if mode & 0o6000 == 0 {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!(
            "destination {} has privileged mode {mode:o}; expected a document without set-user/group-ID bits",
            destination.display(),
        ),
    ))
}

#[cfg(test)]
#[path = "linux/tests.rs"]
mod tests;

fn validate_attribute_name(name: &std::ffi::OsStr) -> io::Result<()> {
    use std::os::unix::ffi::OsStrExt;
    let bytes = name.as_bytes();
    let access_namespace = bytes.starts_with(b"security.") || bytes.starts_with(b"system.");
    let known_access = ACCESS_ATTRIBUTES.iter().any(|allowed| name == *allowed);
    let content_bound = ["security.capability", "security.ima", "security.evm"]
        .iter()
        .any(|excluded| name == *excluded);
    if !access_namespace || known_access || content_bound {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!(
            "access attribute {name:?} is unsupported; expected a recognized access-control policy",
        ),
    ))
}
