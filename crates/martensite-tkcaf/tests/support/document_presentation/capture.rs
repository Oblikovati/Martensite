//! The sole result frame uses the existing native viewer and an explicit artifact destination.
use super::{DocumentScene, unsupported};
use martensite_tkernel::standard::Failure;
use std::path::PathBuf;
impl DocumentScene {
    /// Execute the actual single-view result capture; `scene.result_view("bug21707")?`.
    pub(crate) fn result_view(&mut self, image: &str) -> Result<(), Failure> {
        self.validate_views()?;
        self.viewer.make_view(1, "AXON", 728, 450, 400, 400)?;
        assert_eq!(self.viewer.zoom(1), 1., "smallview: expected default zoom");
        self.only("result")?;
        self.fit_and_title()?;
        println!("{{\"fit_zoom\":{}}}", self.viewer.zoom(1));
        self.save_frame(image)
    }

    fn save_frame(&mut self, image: &str) -> Result<(), Failure> {
        let path = self.capture_path(image)?;
        self.save_window(&path)?;
        let bytes = std::fs::read(&path).map_err(|error| capture_failure(&path, &error))?;
        validate_frame(&bytes)?;
        if self.capture_directory.is_none() {
            std::fs::remove_file(&path).map_err(|error| capture_failure(&path, &error))?;
        }
        Ok(())
    }

    fn save_window(&mut self, path: &std::path::Path) -> Result<(), Failure> {
        self.viewer.repaint_view(1)?;
        let name = path
            .to_str()
            .ok_or_else(|| unsupported(&format!("path={path:?}"), "Unicode capture path"))?;
        let saved = self.viewer.save_view(1, name).map_err(|cause| {
            unsupported(
                &format!("save path={path:?}, cause={cause}"),
                "existing drawable native view1",
            )
        })?;
        if !saved {
            return Err(unsupported(
                &format!("save path={path:?}"),
                "successful native frame save",
            ));
        }
        Ok(())
    }

    fn capture_path(&self, image: &str) -> Result<PathBuf, Failure> {
        if let Some(directory) = &self.capture_directory {
            std::fs::create_dir_all(directory)
                .map_err(|error| capture_failure(directory, &error))?;
            return Ok(directory.join(format!("{image}.ppm")));
        }
        Ok(std::env::temp_dir().join(format!(
            "martensite-document-view-{}-{image}.ppm",
            std::process::id()
        )))
    }
}
fn capture_failure(path: &std::path::Path, cause: &std::io::Error) -> Failure {
    unsupported(
        &format!("capture path={path:?}, cause={cause}"),
        "readable and writable native capture destination",
    )
}
fn validate_frame(bytes: &[u8]) -> Result<(), Failure> {
    if bytes.len() != 480015 || !bytes.starts_with(b"P6\n400 400\n255\n") {
        return Err(unsupported(
            &format!(
                "frame bytes={}, header={:?}",
                bytes.len(),
                &bytes[..bytes.len().min(15)]
            ),
            "400x400 RGB P6 with480015bytes",
        ));
    }
    Ok(())
}

/// Check malformed-frame and injected I/O messages once; `assert_capture_refusals()`.
#[cfg(test)]
pub(super) fn assert_capture_refusals() {
    let empty = validate_frame(&[]).unwrap_err();
    assert!(empty.message().contains("frame bytes=0"));
    assert!(empty.message().contains("expected 400x400 RGB P6"));
    let malformed = validate_frame(&vec![0; 480015]).unwrap_err();
    assert!(malformed.message().contains("header=[0,"));
    let cause = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "fixture refusal");
    let denied = capture_failure(std::path::Path::new("denied-frame"), &cause);
    assert!(denied.message().contains("denied-frame"));
    assert!(denied.message().contains("cause=fixture refusal"));
    assert!(denied.message().contains("expected readable and writable"));
}
