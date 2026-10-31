//! Directories, with POSIX semantics.
use super::{
    FileNode,
    calls::{Errno, PosixFileCalls},
    native_calls,
};
use crate::{
    osd::{CadPath, OsdErrorAccess, Protection, WhoAmI},
    standard::Failure,
};
use std::sync::Arc;

const IAM: i32 = WhoAmI::Directory as i32;

/// A directory; `Directory::with_path(CadPath::new("out/")).build(&Protection::new())?`.
///
/// Dereferences to its [`FileNode`] for the inherited members.
#[derive(Clone, Debug)]
pub struct Directory {
    node: FileNode,
}

impl std::ops::Deref for Directory {
    type Target = FileNode;
    fn deref(&self) -> &FileNode {
        &self.node
    }
}
impl std::ops::DerefMut for Directory {
    fn deref_mut(&mut self) -> &mut FileNode {
        &mut self.node
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::empty()
    }
}

impl Directory {
    /// An empty path over the host calls; `Directory::empty()`.
    pub fn empty() -> Self {
        Self::with_calls(CadPath::default(), native_calls())
    }
    /// A directory at `path` over the host calls; `Directory::with_path(CadPath::new("out/"))`.
    pub fn with_path(path: CadPath) -> Self {
        Self::with_calls(path, native_calls())
    }
    /// A directory at `path` over injected calls; `Directory::with_calls(path, calls)`.
    pub fn with_calls(path: CadPath, calls: Arc<dyn PosixFileCalls>) -> Self {
        Self {
            node: FileNode::with_calls(path, calls),
        }
    }
    /// Builds the directory: `umask(0)` (process-wide, never restored) then `mkdir` with
    /// the protection's mode; a missing parent is built first from the path's last trek
    /// component (the extension is kept), and an existing node is not an
    /// error; `dir.build(&Protection::new())?`.
    ///
    /// A missing parent of a path without trek raises `Standard_NumericError` from
    /// the path's trek lookup.
    pub fn build(&mut self, protection: &Protection) -> Result<(), Failure> {
        let name: Vec<u8> = self.node.system_name();
        self.node.calls.umask(0);
        let mut status: Result<(), Errno> =
            self.node.calls.mkdir_with_protection(&name, protection);
        if status == Err(Errno(enoent())) {
            let parent: Directory = self.build_parent(protection)?;
            if parent.failed() {
                self.node.error = parent.node.error.clone();
                return Ok(());
            }
            status = self.node.calls.mkdir_with_protection(&name, protection);
        }
        if let Err(errno) = status.filter_existing() {
            let shown: &[u8] = &name[..name.len().min(2000)];
            let message: String = format!(
                "OSD_Directory::Build Directory \"{}\"",
                String::from_utf8_lossy(shown)
            );
            self.node.record(errno, IAM, &message);
        }
        Ok(())
    }
    fn build_parent(&self, protection: &Protection) -> Result<Directory, Failure> {
        let mut parent_path: CadPath = self.node.path.clone();
        parent_path.up_trek();
        let last: i32 = self.node.path.trek_length();
        parent_path.set_name(self.node.path.trek_value(last)?.as_bytes());
        let mut parent: Directory = Self::with_calls(parent_path, Arc::clone(&self.node.calls));
        parent.build(protection)?;
        Ok(parent)
    }
    /// `mkdtemp("/tmp/CSFXXXXXX")` (mode 0700), or an empty directory
    /// object when it fails; `Directory::build_temporary()`.
    ///
    /// The `unlink` of the new directory always fails (EISDIR) and is ignored.
    pub fn build_temporary() -> Self {
        Self::build_temporary_with(native_calls())
    }
    /// Builds a temporary directory over injected calls; `Directory::build_temporary_with(calls)`.
    pub fn build_temporary_with(calls: Arc<dyn PosixFileCalls>) -> Self {
        let Ok(name) = calls.mkdtemp(b"/tmp/CSFXXXXXX") else {
            return Self::with_calls(CadPath::default(), calls);
        };
        let _ = calls.unlink(&name);
        let mut directory: Self = Self::with_calls(CadPath::default(), calls);
        directory.set_path(CadPath::new(name));
        directory
    }
}

/// `if (aStatus == -1 && errno != EEXIST)`.
trait ExistingIsSuccess {
    fn filter_existing(self) -> Result<(), Errno>;
}
impl ExistingIsSuccess for Result<(), Errno> {
    fn filter_existing(self) -> Result<(), Errno> {
        match self {
            Err(Errno(code)) if code == eexist() => Ok(()),
            other => other,
        }
    }
}

#[cfg(unix)]
const fn enoent() -> i32 {
    libc::ENOENT
}
#[cfg(unix)]
const fn eexist() -> i32 {
    libc::EEXIST
}
#[cfg(not(unix))]
const fn enoent() -> i32 {
    // std reports ERROR_PATH_NOT_FOUND for a missing parent directory.
    3
}
#[cfg(not(unix))]
const fn eexist() -> i32 {
    // std reports ERROR_ALREADY_EXISTS for an existing directory.
    183
}
