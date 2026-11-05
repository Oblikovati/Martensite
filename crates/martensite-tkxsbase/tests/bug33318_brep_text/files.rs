//! Preserve full original fixture bytes and the physical text-file lifecycle.
use std::{
    fs,
    path::{Path, PathBuf},
};
const ORIGINAL: &[u8] =
    include_bytes!("../../../../external/martensite-test-data/dataset-7.9.0/brep/bug28620.brep");

pub(super) struct CaseBrepFiles {
    root: PathBuf,
    input: PathBuf,
    output: PathBuf,
}

impl CaseBrepFiles {
    /// Use a fresh child-owned directory and unconverted fixture; `CaseBrepFiles::new()`.
    pub(super) fn new() -> Self {
        let root = std::env::temp_dir().join(format!("martensite-bug33318-{}", std::process::id()));
        fs::create_dir(&root)
            .unwrap_or_else(|error| panic!("directory {root:?}: expected unused path: {error}"));
        let files = Self {
            input: root.join("bug28620.brep"),
            output: root.join("tmpfile1.brep"),
            root,
        };
        fs::write(&files.input, ORIGINAL).unwrap();
        files
    }

    /// Supply the original physical reader input; `files.input_path()`.
    pub(super) fn input_path(&self) -> &Path {
        &self.input
    }
    /// Supply the original output basename; `files.output_path()`.
    pub(super) fn output_path(&self) -> &Path {
        &self.output
    }
    /// Read the original as text before import; `files.read()`.
    pub(super) fn read(&self) -> String {
        fs::read_to_string(&self.input).unwrap()
    }
    /// Read the physical exported file after write; `files.read_written()`.
    pub(super) fn read_written(&self) -> String {
        fs::read_to_string(&self.output).unwrap()
    }
    /// Keep actual files only when evidence capture was requested; `files.capture_written()`.
    pub(super) fn capture_written(&self) {
        let Some(root) = std::env::var_os("MARTENSITE_BUG33318_ARTIFACT_DIR") else {
            return;
        };
        let root = PathBuf::from(root);
        fs::create_dir_all(&root).unwrap();
        fs::copy(&self.input, root.join("original.brep")).unwrap();
        fs::copy(&self.output, root.join("written.brep")).unwrap();
    }
    /// Preserve original forced output removal after its predicate; `files.remove_written()`.
    pub(super) fn remove_written(&self) {
        fs::remove_file(&self.output).unwrap();
    }
}
impl Drop for CaseBrepFiles {
    fn drop(&mut self) {
        // Only a directory successfully created by this child is ever owned here.
        let _ = fs::remove_dir_all(&self.root);
    }
}
