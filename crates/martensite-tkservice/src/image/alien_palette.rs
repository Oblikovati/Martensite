//! Indexed (palette) PNG and BMP files: their palette decides between a gray and a colour
//! pixmap, which the decoded RGB values alone cannot tell.
//!
//! An indexed image loads as `Gray` when its palette is a gray ramp: black then white for one
//! bit per pixel, else every listed entry `i` equal to `(i, i, i)`. Any other palette loads as
//! `Bgr`, without the palette's transparency.

/// How an indexed file's pixels load; `None` from `indexed_kind` for a direct-colour file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum IndexedKind {
    /// The index is the gray level.
    GrayRamp,
    /// The palette colours load as opaque colour.
    Colour,
}

/// The kind of an indexed PNG or BMP file, `None` for any other file.
///
/// Example: `indexed_kind(&std::fs::read("a.png")?) == Some(IndexedKind::Colour)`.
pub(super) fn indexed_kind(bytes: &[u8]) -> Option<IndexedKind> {
    let (depth, palette) = png_palette(bytes).or_else(|| bmp_palette(bytes))?;
    Some(if is_gray_ramp(depth, &palette) {
        IndexedKind::GrayRamp
    } else {
        IndexedKind::Colour
    })
}

fn is_gray_ramp(depth: u8, palette: &[[u8; 3]]) -> bool {
    if depth == 1 {
        return palette.first() == Some(&[0; 3]) && palette.get(1) == Some(&[u8::MAX; 3]);
    }
    !palette.is_empty()
        && palette
            .iter()
            .enumerate()
            .all(|(i, entry)| entry.iter().all(|&c| usize::from(c) == i))
}

/// Bit depth and PLTE entries of an indexed PNG (colour type 3).
fn png_palette(bytes: &[u8]) -> Option<(u8, Vec<[u8; 3]>)> {
    const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    if !bytes.starts_with(SIGNATURE) || bytes.get(25) != Some(&3) {
        return None;
    }
    let depth = *bytes.get(24)?;
    let mut offset = SIGNATURE.len();
    while let Some(header) = bytes.get(offset..offset + 8) {
        let length = u32::from_be_bytes([header[0], header[1], header[2], header[3]]) as usize;
        let body = bytes.get(offset + 8..offset + 8 + length)?;
        if &header[4..8] == b"PLTE" {
            return Some((depth, body.as_chunks::<3>().0.to_vec()));
        }
        offset += 12 + length;
    }
    None
}

/// Bit depth and colour table (blue-green-red entries, turned to RGB) of a BMP with at most
/// eight bits per pixel and a Windows information header.
fn bmp_palette(bytes: &[u8]) -> Option<(u8, Vec<[u8; 3]>)> {
    let read_u32 = |at: usize| Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?));
    if !bytes.starts_with(b"BM") {
        return None;
    }
    let header_size = read_u32(14)? as usize;
    let depth = u16::from_le_bytes(bytes.get(28..30)?.try_into().ok()?);
    if header_size < 40 || depth > 8 {
        return None;
    }
    let count = match read_u32(46)? {
        0 => 1usize << depth,
        used => used as usize,
    };
    let start = 14 + header_size;
    let table = bytes.get(start..start + 4 * count)?;
    Some((
        depth as u8,
        table
            .as_chunks::<4>()
            .0
            .iter()
            .map(|c| [c[2], c[1], c[0]])
            .collect(),
    ))
}

#[cfg(test)]
#[path = "alien_palette_tests.rs"]
mod tests;
