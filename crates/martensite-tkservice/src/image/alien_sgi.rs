//! SGI image files (".rgb", ".rgba", ".bw", ".sgi"), read without an external decoder.
//!
//! A file is a 512-byte big-endian header, then one plane per channel, each plane's rows
//! bottom row first. Rows are stored verbatim or run-length encoded; an encoded file lists
//! the start offset of every row after the header. Channels of one byte are read, with one
//! to four channels: gray, gray and alpha, red-green-blue, or red-green-blue-alpha. Files
//! with two-byte channels or a colour map are refused. Gray and alpha become a four-channel
//! image whose colour repeats the gray value.
use super::ImageFormat;
use super::alien_codec::DecodeFailure;
use super::alien_pixels::{DecodedRows, four_channel, reversed_colour};

/// The magic number 474 that starts every file.
const SIGNATURE: [u8; 2] = [0x01, 0xda];
const HEADER_BYTES: usize = 512;
/// Extensions of SGI files, compared without case.
const EXTENSIONS: [&[u8]; 4] = [b"rgb", b"rgba", b"bw", b"sgi"];

/// The header fields that select the layout of the pixel planes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SgiLayout {
    run_length: bool,
    width: usize,
    height: usize,
    channels: usize,
}

/// Whether `bytes` starts with the SGI magic number; `has_sgi_signature(&[1, 0xda])`.
pub(super) fn has_sgi_signature(bytes: &[u8]) -> bool {
    bytes.starts_with(&SIGNATURE)
}

/// Whether `extension` names an SGI file; `is_sgi_extension(b"RGB")`.
pub(super) fn is_sgi_extension(extension: &[u8]) -> bool {
    EXTENSIONS
        .iter()
        .any(|known| known.eq_ignore_ascii_case(extension))
}

/// The pixels of an SGI file, top row first: `Gray`, `Bgr`, or `Bgra` (`Bgr32` when every
/// pixel is opaque). Example: `decode_sgi(&file_bytes)?.format`.
pub(super) fn decode_sgi(bytes: &[u8]) -> Result<DecodedRows, DecodeFailure> {
    let layout = parse_header(bytes)?;
    let planes = (0..layout.channels)
        .map(|channel| read_plane(bytes, &layout, channel))
        .collect::<Result<Vec<_>, _>>()?;
    let (format, pixels) = interleave(&planes, &layout);
    Ok(DecodedRows {
        format,
        width: layout.width,
        height: layout.height,
        pixels,
    })
}

fn be_u16(bytes: &[u8], at: usize) -> usize {
    usize::from(u16::from_be_bytes([bytes[at], bytes[at + 1]]))
}

fn be_u32(bytes: &[u8], at: usize) -> Option<usize> {
    let word = bytes.get(at..at + 4)?;
    usize::try_from(u32::from_be_bytes([word[0], word[1], word[2], word[3]])).ok()
}

/// The layout of a supported file; a one-dimensional image has one row and a
/// two-dimensional image one channel, whatever the stored height and depth say.
fn parse_header(bytes: &[u8]) -> Result<SgiLayout, DecodeFailure> {
    if bytes.len() < HEADER_BYTES || !has_sgi_signature(bytes) {
        return Err(DecodeFailure::MissingOrInvalid);
    }
    let (storage, channel_bytes, dimension) = (bytes[2], bytes[3], be_u16(bytes, 4));
    if storage > 1 || !(1..=2).contains(&channel_bytes) || !(1..=3).contains(&dimension) {
        return Err(DecodeFailure::MissingOrInvalid);
    }
    let (width, stored_height, depth) = (be_u16(bytes, 6), be_u16(bytes, 8), be_u16(bytes, 10));
    let height = if dimension == 1 { 1 } else { stored_height };
    let channels = if dimension < 3 { 1 } else { depth };
    if width == 0 || height == 0 || channels == 0 {
        return Err(DecodeFailure::MissingOrInvalid);
    }
    let colour_map = be_u32(bytes, 104).unwrap_or(1);
    if channel_bytes != 1 || colour_map != 0 || channels > 4 {
        return Err(DecodeFailure::UnsupportedPixels);
    }
    Ok(SgiLayout {
        run_length: storage == 1,
        width,
        height,
        channels,
    })
}

/// The rows of one channel, bottom row first, `width` bytes each.
fn read_plane(bytes: &[u8], layout: &SgiLayout, channel: usize) -> Result<Vec<u8>, DecodeFailure> {
    let mut plane = Vec::with_capacity(layout.width * layout.height);
    for row in 0..layout.height {
        let index = channel * layout.height + row;
        if layout.run_length {
            // The start table lists every row of plane 0, then of plane 1, and so on.
            let start = be_u32(bytes, HEADER_BYTES + 4 * index);
            let start = start.ok_or(DecodeFailure::MissingOrInvalid)?;
            decode_run_row(bytes, start, layout.width, &mut plane)?;
        } else {
            let start = HEADER_BYTES + index * layout.width;
            let stored = bytes.get(start..start + layout.width);
            plane.extend_from_slice(stored.ok_or(DecodeFailure::MissingOrInvalid)?);
        }
    }
    Ok(plane)
}

/// Appends one run-length encoded row of `width` bytes starting at `start`. Each run begins
/// with a count byte: its high bit marks `count & 0x7f` literal bytes, otherwise the next
/// byte repeats `count` times; a zero count ends the row. A row that ends early, overflows
/// its width or leaves the file is invalid.
fn decode_run_row(
    bytes: &[u8],
    start: usize,
    width: usize,
    plane: &mut Vec<u8>,
) -> Result<(), DecodeFailure> {
    let (end, mut at) = (plane.len() + width, start);
    while plane.len() < end {
        let count_byte = *bytes.get(at).ok_or(DecodeFailure::MissingOrInvalid)?;
        let count = usize::from(count_byte & 0x7f);
        if count == 0 || plane.len() + count > end {
            return Err(DecodeFailure::MissingOrInvalid);
        }
        let literal = count_byte & 0x80 != 0;
        let stored = bytes.get(at + 1..at + 1 + if literal { count } else { 1 });
        let stored = stored.ok_or(DecodeFailure::MissingOrInvalid)?;
        if literal {
            plane.extend_from_slice(stored);
        } else {
            plane.resize(plane.len() + count, stored[0]);
        }
        at += 1 + stored.len();
    }
    Ok(())
}

/// The pixmap format and packed pixels of the channel planes, rows flipped to top first.
fn interleave(planes: &[Vec<u8>], layout: &SgiLayout) -> (ImageFormat, Vec<u8>) {
    // Gray and alpha repeat the gray value into red, green and blue.
    let sources: &[usize] = match planes.len() {
        1 => &[0],
        2 => &[0, 0, 0, 1],
        3 => &[0, 1, 2],
        _ => &[0, 1, 2, 3],
    };
    let stride = sources.len();
    let mut pixels = vec![0u8; layout.width * layout.height * stride];
    for (slot, &plane) in sources.iter().enumerate() {
        let top_first = planes[plane].chunks_exact(layout.width).rev().flatten();
        for (target, &value) in pixels.iter_mut().skip(slot).step_by(stride).zip(top_first) {
            *target = value;
        }
    }
    match sources.len() {
        1 => (ImageFormat::Gray, pixels),
        3 => (ImageFormat::Bgr, reversed_colour(&pixels, 3)),
        _ => four_channel(&pixels),
    }
}

#[cfg(test)]
#[path = "alien_sgi_tests.rs"]
pub(super) mod tests;
