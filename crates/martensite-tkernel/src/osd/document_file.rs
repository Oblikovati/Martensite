//! Access-policy boundary for staged document files; publication belongs to the caller.
use std::{io, path::Path};

#[cfg(target_os = "linux")]
#[path = "document_file/linux.rs"]
mod backend;
#[cfg(target_vendor = "apple")]
#[path = "document_file/macos.rs"]
mod backend;
#[cfg(windows)]
#[path = "document_file/windows.rs"]
mod backend;
#[cfg(unix)]
#[path = "document_file/unix.rs"]
mod unix;

/// Create an exclusive staging directory whose access policy protects document bytes.
/// The directory must not exist; Unix platforms grant access only to its owner.
///
/// ```no_run
/// use martensite_tkernel::osd::create_private_document_directory;
/// let stage = std::env::temp_dir().join("document-save-stage");
/// create_private_document_directory(&stage)?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn create_private_document_directory(directory: &Path) -> io::Result<()> {
    backend::create_private_directory(directory)
}

/// Preserve and verify an existing destination's access policy on a staged file.
/// A failure leaves the destination untouched; the caller owns syncing and publication.
/// New destinations receive the final parent directory's inherited file policy.
/// The staged output must be a regular file; symbolic links are refused.
///
/// ```no_run
/// use martensite_tkernel::osd::preserve_document_access;
/// use std::path::Path;
/// preserve_document_access(Path::new("model"), Path::new("stage/model"))?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn preserve_document_access(destination: &Path, staged: &Path) -> io::Result<()> {
    let metadata = std::fs::symlink_metadata(staged)?;
    if !metadata.file_type().is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "staged document {} is not regular; expected a regular file",
                staged.display(),
            ),
        ));
    }
    backend::preserve_access(destination, staged)
}
