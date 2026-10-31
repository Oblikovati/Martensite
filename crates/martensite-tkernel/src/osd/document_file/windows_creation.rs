//! Create with the requested DACL at the native call, without a post-creation exposure (#334).
use super::*;
use crate::osd::{OpenFlags, OpenMode, Protection};
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows_sys::Win32::Storage::FileSystem::*;

fn creation_descriptor(rights: &Protection) -> io::Result<LocalAllocation> {
    let owner = effective_token_sid(TokenOwner)?;
    let group = effective_token_sid(TokenPrimaryGroup)?;
    let (system, user, primary, world) = rights.values();
    let names = ["SY", owner.as_str(), group.as_str(), "WD"];
    let entries = names
        .into_iter()
        .zip([system, user, primary, world])
        .map(|(sid, right)| format!("(A;;0x{:x};;;{sid})", protection::to_access_mask(right)))
        .collect::<String>();
    parse_descriptor(&format!("O:{owner}G:{group}D:P{entries}"))
}

fn attributes(descriptor: &LocalAllocation) -> SECURITY_ATTRIBUTES {
    SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: descriptor.0,
        bInheritHandle: 0,
    }
}

pub(crate) fn create_protected_directory(path: &Path, rights: &Protection) -> io::Result<()> {
    let descriptor = creation_descriptor(rights)?;
    let attributes = attributes(&descriptor);
    let path = wide_path(path)?;
    // SAFETY: path and descriptor-backed attributes are initialized and live.
    // Windows applies the descriptor in the same operation that creates the directory.
    win32_success(unsafe { CreateDirectoryW(path.as_ptr(), &attributes) })
}

fn desired_access(flags: OpenFlags) -> u32 {
    let read = if flags.access != OpenMode::WriteOnly {
        FILE_GENERIC_READ
    } else {
        0
    };
    let write = if flags.access != OpenMode::ReadOnly || flags.truncate {
        FILE_GENERIC_WRITE
    } else {
        0
    };
    let access = read | write;
    if flags.append {
        access & !FILE_WRITE_DATA
    } else {
        access
    }
}

fn disposition(flags: OpenFlags) -> u32 {
    match (flags.create, flags.truncate) {
        (true, true) => CREATE_ALWAYS,
        (true, false) => OPEN_ALWAYS,
        (false, true) => TRUNCATE_EXISTING,
        (false, false) => OPEN_EXISTING,
    }
}

pub(crate) fn open_protected_file(
    path: &Path,
    flags: OpenFlags,
    rights: &Protection,
) -> io::Result<fs::File> {
    open_file_with_disposition(path, flags, rights, disposition(flags))
}

pub(crate) fn exclusive_protected_file(path: &Path, rights: &Protection) -> io::Result<fs::File> {
    let flags = OpenFlags {
        access: OpenMode::ReadWrite,
        create: true,
        truncate: false,
        append: false,
    };
    open_file_with_disposition(path, flags, rights, CREATE_NEW)
}

fn open_file_with_disposition(
    path: &Path,
    flags: OpenFlags,
    rights: &Protection,
    disposition: u32,
) -> io::Result<fs::File> {
    let descriptor = creation_descriptor(rights)?;
    let attributes = attributes(&descriptor);
    let path = wide_path(path)?;
    // SAFETY: all inputs are live; returned ownership is unique, non-inheritable,
    // and transferred to File only after checking the documented failure sentinel.
    let handle = unsafe {
        CreateFileW(
            path.as_ptr(),
            desired_access(flags),
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            &attributes,
            disposition,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: CreateFileW transferred this valid handle; no other owner exists.
    Ok(unsafe { fs::File::from_raw_handle(handle) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::osd::SingleProtection as Right;
    use std::io::Write;

    fn flags() -> OpenFlags {
        OpenFlags {
            access: OpenMode::ReadWrite,
            create: true,
            truncate: false,
            append: false,
        }
    }

    #[test]
    fn file_and_directory_are_created_with_explicit_world_rights() {
        let root = tempfile::tempdir().unwrap();
        let rights = Protection::with_values(Right::Rwxd, Right::Rwxd, Right::Rx, Right::None);
        let file = root.path().join("private-file");
        let directory = root.path().join("private-directory");
        drop(open_protected_file(&file, flags(), &rights).unwrap());
        create_protected_directory(&directory, &rights).unwrap();
        for path in [file, directory] {
            assert_eq!(
                protection::read_protection(&path).unwrap().world(),
                Right::None
            );
        }
    }

    #[test]
    fn public_file_build_passes_the_creation_policy_to_windows() {
        use crate::osd::{File, OsdErrorAccess};
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("public-file");
        let mut file = File::new(path.to_string_lossy().as_bytes());
        let rights = Protection::with_values(Right::Rwxd, Right::Rwxd, Right::Rx, Right::None);
        file.build(OpenMode::ReadWrite, &rights).unwrap();
        assert!(!file.failed());
        assert_eq!(file.protection().world(), Right::None);
        file.close().unwrap();
    }

    #[test]
    fn reopening_existing_file_does_not_replace_its_acl() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("existing");
        drop(open_protected_file(&path, flags(), &Protection::new()).unwrap());
        let before = protection::read_protection(&path).unwrap();
        let private = Protection::with_values(Right::Rwxd, Right::Rwxd, Right::None, Right::None);
        drop(open_protected_file(&path, flags(), &private).unwrap());
        assert_eq!(protection::read_protection(&path).unwrap(), before);
    }

    #[test]
    fn exclusive_creation_preserves_an_existing_file() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("collision");
        fs::write(&path, b"retained").unwrap();
        let error = exclusive_protected_file(&path, &Protection::new()).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read(path).unwrap(), b"retained");
    }

    #[test]
    fn append_handle_appends_without_a_position_race() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("append");
        let mut first = open_protected_file(&path, flags(), &Protection::new()).unwrap();
        first.write_all(b"first").unwrap();
        let mut append = flags();
        append.append = true;
        open_protected_file(&path, append, &Protection::new())
            .unwrap()
            .write_all(b"second")
            .unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"firstsecond");
    }

    #[test]
    fn dispositions_and_append_access_match_open_flags() {
        for (create, truncate, expected) in [
            (true, true, CREATE_ALWAYS),
            (true, false, OPEN_ALWAYS),
            (false, true, TRUNCATE_EXISTING),
            (false, false, OPEN_EXISTING),
        ] {
            let mut options = flags();
            options.create = create;
            options.truncate = truncate;
            assert_eq!(disposition(options), expected);
        }
        let mut append = flags();
        append.append = true;
        assert_eq!(desired_access(append) & FILE_WRITE_DATA, 0);
        assert_ne!(desired_access(append) & FILE_APPEND_DATA, 0);
    }
}
