//! An empty reference file obtains the final parent's actual inheritance policy.
use super::AccessSnapshot;
use std::{
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

struct InheritedAccessTemplate {
    path: PathBuf,
    _file: fs::File,
}

pub(super) fn snapshot_for_new_destination(
    destination: &Path,
    staged: &Path,
) -> io::Result<AccessSnapshot> {
    verify_default_staged_policy(staged)?;
    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let template = InheritedAccessTemplate::new(parent)?;
    AccessSnapshot::read(&template.path)
}

fn verify_default_staged_policy(staged: &Path) -> io::Result<()> {
    let parent = staged
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let template = InheritedAccessTemplate::new(parent)?;
    AccessSnapshot::read(&template.path)?.verify(staged)
}

impl InheritedAccessTemplate {
    fn new(parent: &Path) -> io::Result<Self> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
        let nonce = timestamp.map_err(io::Error::other)?.as_nanos();
        for attempt in 0..100 {
            let path = template_path(parent, nonce, attempt);
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(file) => return Ok(Self { path, _file: file }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "expected an unused access-template file",
        ))
    }
}

impl Drop for InheritedAccessTemplate {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn template_path(parent: &Path, nonce: u128, attempt: u32) -> PathBuf {
    parent.join(format!(
        ".martensite-access-{}-{nonce}-{attempt}",
        std::process::id()
    ))
}
