//! File-system nodes: file nodes, files, directories,
//! and directory and file iterators.
//!
//! The types follow the POSIX semantics (the verified Linux profile, also
//! macOS). Every system call goes through [`PosixFileCalls`]: `NativePosixCalls` makes
//! the libc calls on Unix hosts; on Windows it emulates them over `std::fs`, because the
//! Win32 semantics (handles, ACL protections, `LockFileEx`) are not implemented.
mod calls;
mod directory;
mod directory_stream;
mod file_io;
mod file_node;
mod iterators;
#[cfg(not(unix))]
mod native_std;
#[cfg(unix)]
mod native_unix;
mod osd_file;
mod stdio;

pub use calls::{
    AccessMode, CallResult, Errno, NodeStatus, NodeType, OpenFlags, PosixFileCalls, RecordLock,
    SeekOrigin, native_calls,
};
pub use directory::Directory;
pub use directory_stream::DirectoryEntries;
pub use file_node::FileNode;
pub use iterators::{DirectoryIterator, FileIterator, mask_matches};
#[cfg(not(unix))]
pub use native_std::NativePosixCalls;
#[cfg(unix)]
pub use native_unix::NativePosixCalls;
pub use osd_file::File;
