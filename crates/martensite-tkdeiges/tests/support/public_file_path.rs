//! Where a public IGES file of the pinned test data is.
use std::path::PathBuf;

/// A file in the dataset's folder or the one of the source tree's files.
/// Example: `public_file_path("BIGES.igs")`.
pub fn public_file_path(name: &str) -> PathBuf {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../external/martensite-test-data");
    let in_dataset = root.join("dataset-7.9.0/iges").join(name);
    if in_dataset.is_file() {
        in_dataset
    } else {
        root.join("occt-source-data/iges").join(name)
    }
}
