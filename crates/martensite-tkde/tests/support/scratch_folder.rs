//! Paths for the file checks. A fresh temporary folder proves a name is free on every platform,
//! where a fixed path such as a Unix root folder only assumes it.
use std::path::{Path, PathBuf};
use tempfile::{Builder, TempDir};

/// A temporary folder that goes when this does.
pub struct ScratchFolder {
    root: TempDir,
}

impl ScratchFolder {
    /// A new empty folder; `ScratchFolder::new()`.
    pub fn new() -> Self {
        let root = Builder::new()
            .prefix("tkde-")
            .tempdir()
            .unwrap_or_else(|cause| panic!("a temporary folder: {cause}"));
        Self { root }
    }

    /// `name` inside folders that do not exist, so neither reading nor creating it can work;
    /// `folder.missing("file.stp")`.
    pub fn missing(&self, name: &str) -> String {
        let path = self.root.path().join("no").join("such").join(name);
        assert!(!path.exists(), "{path:?}: expected a missing path");
        text_of(&path)
    }

    /// `name` directly in the folder, where it can be created; `folder.present("conf.txt")`.
    pub fn file(&self, name: &str) -> PathBuf {
        self.root.path().join(name)
    }
}

/// The path as the text the checks take; `text_of(&folder.file("a.txt"))`.
pub fn text_of(path: &Path) -> String {
    path.to_str()
        .unwrap_or_else(|| panic!("{path:?}: expected a UTF-8 path"))
        .to_owned()
}
