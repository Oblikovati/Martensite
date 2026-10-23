//! Restricts intermediate document bytes before a writer opens the staged file.
use std::{fs, io, os::unix::fs::DirBuilderExt, path::Path};

pub(super) fn create_private_directory(directory: &Path) -> io::Result<()> {
    fs::DirBuilder::new().mode(0o700).create(directory)
}
