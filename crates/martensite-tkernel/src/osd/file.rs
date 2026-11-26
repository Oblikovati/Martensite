//! File-system nodes: file nodes, files, directories,
//! and directory and file iterators.
//!
//! The types follow the POSIX semantics (the verified Linux profile, also
//! macOS). Every system call goes through [`PosixFileCalls`]: `NativePosixCalls` makes
//! the libc calls on Unix hosts; Windows uses native handles and whole-file locking
//! through `std::fs`, native local-time conversion and typed file-node ACLs.
//! Creation-time ACLs remain unfinished.
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

use calls::native_calls;
pub use calls::{
    AccessMode, CallResult, Errno, NodeStatus, NodeType, OpenFlags, PosixFileCalls, RecordLock,
    SeekOrigin,
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

#[cfg(test)]
mod nix_dir_guard_tests {
    /// Guard against enabling `nix`'s `dir` feature while `nix 0.31.3` `Dir::from_fd`
    /// leaks consumed file descriptors on failure (#126).
    #[test]
    fn tkernel_manifest_keeps_nix_dir_feature_disabled() {
        let manifest = include_str!("../../Cargo.toml");
        for line in manifest.lines() {
            if line.trim_start().starts_with("nix =") {
                assert!(
                    !line.contains("\"dir\""),
                    "nix `dir` feature must remain disabled until upstream fixes Dir::from_fd (#126): {line}"
                );
            }
        }
    }
}
