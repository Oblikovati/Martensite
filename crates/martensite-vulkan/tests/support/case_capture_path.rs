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
support_unit_tests! {
    #[path = "case_capture_path_controls.rs"]
    mod controls;
}
