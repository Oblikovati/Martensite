//! Publishes a driver's completed file before attaching its final-path metadata (#245).
use crate::{
    cdm::CdmDocument,
    pcdm::{PcdmStorageDriver, PcdmStoreStatus},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    tcollection::ExtendedString,
};
use std::{
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(target_os = "linux")]
mod access_metadata;

struct StagedStore {
    directory: PathBuf,
    destination: PathBuf,
    file: PathBuf,
}

pub(super) fn write_document(
    writer: &dyn PcdmStorageDriver,
    document: &Handle<dyn CdmDocument>,
    destination: &ExtendedString,
    range: &ProgressRange<'_>,
) -> Result<PcdmStoreStatus, Failure> {
    let stage = StagedStore::new(Path::new(&destination.to_utf8()))
        .map_err(|error| storage_failure(destination, error, "prepare"))?;
    let staged_name = ExtendedString::new(&stage.file.to_string_lossy());
    writer.write_file(document, &staged_name, range)?;
    let status = writer.get_store_status();
    if status == PcdmStoreStatus::Ok {
        stage
            .commit()
            .map_err(|error| storage_failure(destination, error, "publish"))?;
    }
    Ok(status)
}

fn storage_failure(destination: &ExtendedString, error: io::Error, phase: &str) -> Failure {
    let exception = match (phase, error.kind()) {
        ("publish", io::ErrorKind::NotFound) => "DocumentStore_MissingOutput",
        (_, io::ErrorKind::PermissionDenied) => "DocumentStore_PermissionDenied",
        (_, io::ErrorKind::NotFound) => "DocumentStore_DestinationNotFound",
        (_, io::ErrorKind::InvalidInput) => "DocumentStore_InvalidDestination",
        _ => "DocumentStore_PublicationFailure",
    };
    Failure::with_message(exception, &format!(
        "DocumentStore::{phase} {}: expected a completed writable file; cause_kind={:?}; cause_os_code={:?}; cause={error}",
        destination.to_utf8(), error.kind(), error.raw_os_error(),
    )).with_exception_parent("Standard_Failure")
}

impl StagedStore {
    fn new(destination: &Path) -> io::Result<Self> {
        let destination = resolved_destination(destination)?;
        ensure_destination_writable(&destination)?;
        let parent = destination.parent().unwrap_or_else(|| Path::new("."));
        let name = destination
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "expected a file name"))?;
        let directory = staging_directory(parent)?;
        let file = directory.join(name);
        Ok(Self {
            directory,
            destination,
            file,
        })
    }

    fn commit(&self) -> io::Result<()> {
        let staged_file = fs::OpenOptions::new().write(true).open(&self.file)?;
        #[cfg(target_os = "linux")]
        access_metadata::preserve_access(&self.destination, &self.file)?;
        #[cfg(not(target_os = "linux"))]
        if let Ok(metadata) = fs::metadata(&self.destination) {
            fs::set_permissions(&self.file, metadata.permissions())?;
        }
        staged_file.sync_all()?;
        // The sibling directory keeps rename on the same filesystem. No failed driver
        // can publish partial bytes, and document metadata retains the requested path.
        fs::rename(&self.file, &self.destination)
    }
}

impl Drop for StagedStore {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn resolved_destination(destination: &Path) -> io::Result<PathBuf> {
    let mut resolved = destination.to_owned();
    for _ in 0..=40 {
        match fs::symlink_metadata(&resolved) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                let link = fs::read_link(&resolved)?;
                resolved = resolved
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join(link);
            }
            Ok(_) => return Ok(resolved),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(resolved),
            Err(error) => return Err(error),
        }
    }
    Err(excessive_symlinks(destination))
}

fn excessive_symlinks(destination: &Path) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "symbolic link chain at {} exceeded 40 links; expected a finite destination",
            destination.display(),
        ),
    )
}

fn ensure_destination_writable(destination: &Path) -> io::Result<()> {
    match fs::metadata(destination) {
        Ok(metadata) if metadata.permissions().readonly() => Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!(
                "destination {} is read-only; expected write access",
                destination.display()
            ),
        )),
        Ok(_) => fs::OpenOptions::new()
            .write(true)
            .open(destination)
            .map(|_| ()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn staging_directory(parent: &Path) -> io::Result<PathBuf> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
    let nonce = timestamp.map_err(io::Error::other)?.as_nanos();
    for attempt in 0..100 {
        let directory = parent.join(format!(
            ".martensite-save-{}-{nonce}-{attempt}",
            std::process::id()
        ));
        match private_directory(&directory) {
            Ok(()) => return Ok(directory),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "expected an unused staging directory",
    ))
}

fn private_directory(directory: &Path) -> io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(directory)
}

#[cfg(test)]
mod tests;
