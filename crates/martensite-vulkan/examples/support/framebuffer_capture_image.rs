//! Completed framebuffer publication uses owned image storage and atomic file replacement.
#![allow(dead_code)]
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::image::{ImageAlienPixMap, ImageFormat, ImagePixMap};
use martensite_vulkan::FrameCapture;
use std::{io, path::Path};
/// Inject publication I/O; e.g. publish_capture(prefix, frame, &FileFrameImageStore).
pub trait FrameImageStore {
    fn write(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
    fn write_image(&self, path: &Path, image: &ImagePixMap) -> io::Result<()>;
    fn rename(&self, temporary: &Path, target: &Path) -> io::Result<()>;
}
pub struct FileFrameImageStore;
impl FrameImageStore for FileFrameImageStore {
    fn write(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        std::fs::write(path, bytes)
    }
    fn write_image(&self, path: &Path, image: &ImagePixMap) -> io::Result<()> {
        let output = ImageAlienPixMap::default();
        if !output.init_copy(image) {
            return Err(io::Error::other("expected owned image copy"));
        }
        let name = path
            .to_str()
            .ok_or_else(|| io::Error::other(format!("path={path:?}; expected UTF8")))?;
        let saved = output
            .save(&AsciiString::from(name))
            .map_err(io::Error::other)?;
        saved
            .then_some(())
            .ok_or_else(|| io::Error::other(format!("image={path:?}; expected saved PPM")))
    }
    fn rename(&self, temporary: &Path, target: &Path) -> io::Result<()> {
        std::fs::rename(temporary, target)
    }
}
/// Publish color/depth and an actual image after rendering; e.g. publish_capture(Path::new("frame"),&frame,&FileFrameImageStore).
pub fn publish_capture(
    prefix: &Path,
    frame: &FrameCapture,
    store: &impl FrameImageStore,
) -> io::Result<()> {
    let image = owned_capture_image(frame)?;
    publish_bytes(&capture_artifact_path(prefix, "rgba"), &frame.rgba, store)?;
    let depth: Vec<_> = frame
        .depth
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    publish_bytes(&capture_artifact_path(prefix, "depth"), &depth, store)?;
    let temporary = capture_artifact_path(prefix, "tmp.ppm");
    store.write_image(&temporary, &image)?;
    store.rename(&temporary, &capture_artifact_path(prefix, "ppm"))
}
fn capture_artifact_path(prefix: &Path, suffix: &str) -> std::path::PathBuf {
    let mut name = prefix.as_os_str().to_os_string();
    name.push(format!(".{suffix}"));
    name.into()
}

fn owned_capture_image(frame: &FrameCapture) -> io::Result<ImagePixMap> {
    validate_capture_extent(frame)?;
    let image = ImagePixMap::default();
    if !image.init_wrapper(
        ImageFormat::Rgba,
        Some(frame.rgba.clone()),
        frame.width as usize,
        frame.height as usize,
        0,
    ) {
        return Err(io::Error::other("expected owned RGBA ImagePixMap"));
    }
    image.set_top_down(true);
    Ok(image)
}
fn validate_capture_extent(frame: &FrameCapture) -> io::Result<()> {
    let expected = (frame.width as usize)
        .checked_mul(frame.height as usize)
        .and_then(|pixels| pixels.checked_mul(4));
    let Some(bytes) = expected else {
        return Err(invalid_capture_shape(frame, expected));
    };
    if frame.rgba.len() != bytes || frame.depth.len() != bytes / 4 || bytes == 0 {
        return Err(invalid_capture_shape(frame, expected));
    }
    Ok(())
}
fn invalid_capture_shape(frame: &FrameCapture, expected: Option<usize>) -> io::Error {
    let shape = match expected {
        Some(bytes) => format!(
            "{bytes} RGBA bytes and {} depth values for positive extent",
            bytes / 4
        ),
        None => "positive extent with representable usize RGBA byte length".into(),
    };
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "RGBA bytes={}, depth values={}, extent={}x{}; expected {shape}",
            frame.rgba.len(),
            frame.depth.len(),
            frame.width,
            frame.height
        ),
    )
}

fn publish_bytes(path: &Path, bytes: &[u8], store: &impl FrameImageStore) -> io::Result<()> {
    // Oblikovati viewport_screenshot.go publishes only after completed render via temporary→rename.
    let temporary = path.with_extension("tmp");
    store.write(&temporary, bytes)?;
    store.rename(&temporary, path)
}
#[cfg(test)]
#[path = "framebuffer_capture_image_tests.rs"]
mod tests;
