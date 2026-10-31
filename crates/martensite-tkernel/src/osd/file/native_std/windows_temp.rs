//! Random names and atomic private ACLs for Windows temporary nodes (#334).
#![forbid(unsafe_code)]
use crate::osd::{Protection, SingleProtection, document_file};
use std::{fs::File, io, path::Path};

fn prefix(template: &[u8]) -> io::Result<String> {
    let leaf = template
        .rsplit(|byte| *byte == b'/' || *byte == b'\\')
        .next()
        .unwrap_or(template);
    let stem = leaf
        .strip_suffix(b"XXXXXX")
        .ok_or_else(|| io::Error::from_raw_os_error(87))?;
    String::from_utf8(stem.to_vec()).map_err(|_| io::Error::from_raw_os_error(87))
}

fn private_rights() -> Protection {
    Protection::with_values(
        SingleProtection::Rwxd,
        SingleProtection::Rwxd,
        SingleProtection::None,
        SingleProtection::None,
    )
}

pub(super) fn file(template: &[u8], root: &Path) -> io::Result<(File, Vec<u8>)> {
    let prefix = prefix(template)?;
    // The OSD caller owns cleanup. Disabling automatic cleanup avoids changing native attributes while
    // transferring the path; exclusive creation retries random-name collisions.
    let temporary = tempfile::Builder::new()
        .prefix(&prefix)
        .rand_bytes(12)
        .disable_cleanup(true)
        .make_in(root, |path| {
            document_file::exclusive_protected_file(path, &private_rights())
        })?;
    let (file, path) = temporary.into_parts();
    Ok((file, path.to_string_lossy().as_bytes().to_vec()))
}

pub(super) fn directory(template: &[u8], root: &Path) -> io::Result<Vec<u8>> {
    let prefix = prefix(template)?;
    let temporary = tempfile::Builder::new()
        .prefix(&prefix)
        .rand_bytes(12)
        .disable_cleanup(true)
        .make_in(root, |path| {
            document_file::create_protected_directory(path, &private_rights())
        })?;
    Ok(temporary
        .into_temp_path()
        .to_string_lossy()
        .as_bytes()
        .to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temporary_file_and_directory_are_unique_and_private() {
        let root = tempfile::tempdir().unwrap();
        let (first, first_name) = file(b"/tmp/CSFXXXXXX", root.path()).unwrap();
        let (second, second_name) = file(b"C:\\tmp\\CSFXXXXXX", root.path()).unwrap();
        let directory_name = directory(b"CSFXXXXXX", root.path()).unwrap();
        assert_ne!(first_name, second_name);
        for name in [&first_name, &second_name, &directory_name] {
            let name = std::str::from_utf8(name).unwrap();
            let path = Path::new(name);
            assert_eq!(path.parent(), Some(root.path()));
            assert!(
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("CSF")
            );
            assert_eq!(
                document_file::read_protection(path).unwrap().world(),
                SingleProtection::None
            );
        }
        drop((first, second));
    }

    #[test]
    fn invalid_templates_are_rejected_before_creating_nodes() {
        let root = tempfile::tempdir().unwrap();
        assert!(file(b"no-suffix", root.path()).is_err());
        assert!(directory(b"no-suffix", root.path()).is_err());
        assert!(std::fs::read_dir(root.path()).unwrap().next().is_none());
    }
}
