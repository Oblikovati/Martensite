//! Shared output ownership is independent of a viewer or raster backend.
use std::{error::Error, path::PathBuf};

/// Resolve the existing invocation/PID directory; `capture_path("source.png")?`.
pub(crate) fn capture_path(basename: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = std::env::var_os("MARTENSITE_OPENGL_CASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("martensite-opengl-original-captures"));
    let directory = root.join(std::process::id().to_string());
    std::fs::create_dir_all(&directory)?;
    Ok(directory.join(basename))
}

#[cfg(test)]
mod controls {
    use super::*;

    #[test]
    fn capture_directory_and_basename_preserve_the_existing_join_contract() {
        let root = std::env::var_os("MARTENSITE_OPENGL_CASE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("martensite-opengl-original-captures"));
        let expected = root.join(std::process::id().to_string());
        for basename in ["owned.png", "nested/image.png", ""] {
            assert_eq!(capture_path(basename).unwrap(), expected.join(basename));
            assert!(expected.is_dir());
        }
    }
}
