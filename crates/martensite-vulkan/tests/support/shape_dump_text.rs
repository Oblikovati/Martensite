//! Query the actual complete dump so source text predicates see every representation.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::IoStream;
use std::io::{Read, Seek, SeekFrom};

/// Preserve the full shape dump without synthesizing representation lines.
/// Example: `let text = complete_shape_dump_text(&edge)?`.
pub(crate) fn complete_shape_dump_text(
    shape: &Shape,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = IoStream::from_bytes(Vec::new());
    martensite_tkbrep::brep_tools::dump(shape, &mut stream)?;
    let mut device = stream
        .into_device()
        .ok_or("shape dump device absent; expected an attached in-memory stream")?;
    device.seek(SeekFrom::Start(0))?;
    let mut bytes = Vec::new();
    device.read_to_end(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}
