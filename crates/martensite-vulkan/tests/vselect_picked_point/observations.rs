//! Optional test evidence consumes the already owned points and never queries selection again.
use crate::picked_point_text::source_point_text;
use std::io::{self, Write};
use std::path::Path;

/// Owned I/O boundary for temporary test artifacts; `writer.write_record(path, bytes)`.
pub(crate) trait PointObservationWriter {
    /// Write one new observation or return the actual I/O error; `writer.write_record(path, b"[]")`.
    fn write_record(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
}

/// Stateless filesystem output for the existing capture directory, not a published kernel type.
pub(crate) struct FilesystemPointObservationWriter;

impl PointObservationWriter for FilesystemPointObservationWriter {
    fn write_record(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        let folder = path.parent().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("observation {path:?}; expected a parent folder"),
            )
        })?;
        std::fs::create_dir_all(folder)?;
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        output.write_all(bytes)
    }
}

/// Preserve actual ranked results only when requested; `record_point_observation(None, 1, (0,0), &[], writer)`.
pub(crate) fn record_point_observation(
    folder: Option<&Path>,
    mode: i32,
    pixel: (i32, i32),
    points: &[[f64; 3]],
    writer: &impl PointObservationWriter,
) -> io::Result<()> {
    let Some(folder) = folder else {
        return Ok(());
    };
    let path = folder.join(format!("bug29938-mode-{mode}.points.json"));
    let bytes = point_observation_bytes(mode, pixel, points);
    writer.write_record(&path, bytes.as_bytes()).map_err(|cause| {
        io::Error::new(cause.kind(), format!("picked-point observation {path:?}: {cause}; expected one new writable JSON record"))
    })
}

fn point_observation_bytes(mode: i32, pixel: (i32, i32), points: &[[f64; 3]]) -> String {
    let cursor = [pixel.0, pixel.1];
    let text: Vec<String> = points
        .iter()
        .map(|point| source_point_text(*point))
        .collect();
    let bits: Vec<[u64; 3]> = points.iter().map(|point| point.map(f64::to_bits)).collect();
    format!(
        "{{\"source_mode\":{mode},\"cursor\":{cursor:?},\"point_text\":{text:?},\"rust_point_bits_uncompared\":{bits:?}}}\n"
    )
}

#[cfg(test)]
#[path = "observation_tests.rs"]
mod tests;
