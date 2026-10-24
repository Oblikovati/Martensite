//! Preserves access decisions without copying content signatures or resource forks (#245).
use std::{
    fs, io,
    os::{
        macos::fs::MetadataExt as AppleMetadataExt,
        unix::fs::{MetadataExt, PermissionsExt, chown},
    },
    path::Path,
};

#[path = "macos/acl.rs"]
mod acl;
// The empty reference file has the same lifetime and exclusive creation policy on Unix.
#[path = "linux/new_access.rs"]
mod new_access;
use acl::ExtendedAcl;

const ACCESS_ATTRIBUTES: [&str; 1] = ["com.apple.quarantine"];

struct AccessSnapshot {
    metadata: fs::Metadata,
    acl: ExtendedAcl,
    attributes: [Option<Vec<u8>>; 1],
}

pub(super) fn create_private_directory(directory: &Path) -> io::Result<()> {
    super::unix::create_private_directory(directory)?;
    let result = restrict_directory(directory);
    if result.is_err() {
        let _ = fs::remove_dir(directory);
    }
    result
}

fn restrict_directory(directory: &Path) -> io::Result<()> {
    // chmod does not suppress macOS inherited ACL grants; no writer runs until
    // the empty ACL and owner-only mode have both been read back successfully.
    let empty = ExtendedAcl::empty()?;
    empty.apply(directory)?;
    fs::set_permissions(directory, fs::Permissions::from_mode(0o700))?;
    let actual = fs::metadata(directory)?;
    if actual.mode() & 0o7777 == 0o700
        && ExtendedAcl::read(directory)?.encoded()? == empty.encoded()?
    {
        return Ok(());
    }
    Err(policy_mismatch(directory))
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
    snapshot.copy_to(staged)?;
    snapshot.verify(staged)
}

impl AccessSnapshot {
    fn read(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        validate_policy(path, &metadata)?;
        validate_attributes(path)?;
        Ok(Self {
            metadata,
            acl: ExtendedAcl::read(path)?,
            attributes: [xattr::get_deref(path, ACCESS_ATTRIBUTES[0])?],
        })
    }

    fn copy_to(&self, staged: &Path) -> io::Result<()> {
        chown(staged, Some(self.metadata.uid()), Some(self.metadata.gid()))?;
        fs::set_permissions(staged, self.metadata.permissions())?;
        self.copy_attributes(staged)?;
        self.acl.apply(staged)
    }

    fn copy_attributes(&self, staged: &Path) -> io::Result<()> {
        for (name, expected) in ACCESS_ATTRIBUTES.iter().zip(&self.attributes) {
            if xattr::get_deref(staged, name)? == *expected {
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
        let same_owner = self.metadata.uid() == actual.metadata.uid()
            && self.metadata.gid() == actual.metadata.gid();
        let same_acl = self.acl.encoded()? == actual.acl.encoded()?;
        if same_owner
            && same_acl
            && self.metadata.mode() == actual.metadata.mode()
            && self.attributes == actual.attributes
        {
            return Ok(());
        }
        Err(policy_mismatch(staged))
    }
}

fn validate_policy(path: &Path, metadata: &fs::Metadata) -> io::Result<()> {
    // Immutable, append-only and restricted flags cannot be silently discarded.
    let restrictive_flags =
        libc::UF_IMMUTABLE | libc::UF_APPEND | libc::SF_IMMUTABLE | libc::SF_APPEND | 0x0018_0080; // SF_RESTRICTED, SF_NOUNLINK, UF_DATAVAULT in Apple's sys/stat.h.
    if metadata.mode() & 0o6000 != 0 || metadata.st_flags() & restrictive_flags != 0 {
        return Err(policy_mismatch(path));
    }
    Ok(())
}

fn validate_attributes(path: &Path) -> io::Result<()> {
    for name in xattr::list_deref(path)? {
        let name = name.to_string_lossy();
        if name.starts_with("com.apple.system.")
            || name.starts_with("com.apple.private.")
            || name == "com.apple.rootless"
            // TCC app consent is managed by the OS; byte equality does not
            // establish that its authority can be transferred to a new inode.
            || name == "com.apple.macl"
        {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!(
                    "document {} has unsupported access attribute {name}; expected a policy reproducible by public ACL and xattr APIs",
                    path.display(),
                ),
            ));
        }
    }
    Ok(())
}

fn policy_mismatch(path: &Path) -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!(
            "document {} has unsupported or changed access policy; expected identical owner, group, mode, ACL and access attributes without privileged or restrictive flags",
            path.display(),
        ),
    )
}

#[cfg(test)]
#[path = "macos/tests.rs"]
mod tests;
