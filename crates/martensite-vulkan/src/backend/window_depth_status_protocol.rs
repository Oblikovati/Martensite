//! A fresh operation must observe every submitted input primitive before publication.
use crate::{RenderError, RenderWindowDepthFailure as Failure};
pub(super) const SCHEMA: u32 = 0x5744_5633;
pub(super) const DONE: u32 = 1;
pub(super) const EMPTY: u32 = 2;
#[cfg(test)]
pub(super) const KNOWN: u32 = 127;
pub(super) trait StatusReader {
    fn read_status(&self, size: usize) -> Result<Vec<u8>, RenderError>;
}

pub(super) fn initial_words(triangles: usize) -> Result<Vec<u32>, RenderError> {
    let count = u32::try_from(triangles).map_err(|_| capacity(triangles))?;
    let mut words = vec![0; word_count(triangles)?];
    words[0] = count;
    words[1] = SCHEMA;
    Ok(words)
}
fn word_count(triangles: usize) -> Result<usize, RenderError> {
    let count = triangles
        .checked_mul(2)
        .and_then(|n| n.checked_add(4))
        .ok_or_else(|| capacity(triangles))?;
    count.checked_mul(4).ok_or_else(|| capacity(triangles))?;
    Ok(count)
}
fn capacity(triangles: usize) -> RenderError {
    Failure::UnsupportedInterface{actual:format!("status triangle count={triangles}; expected u32 count and checked addressable header+two words per primitive")}.into()
}

pub(super) fn validate(
    reader: &impl StatusReader,
    indices: &[u32],
    expected: u64,
) -> Result<(), RenderError> {
    if !indices.len().is_multiple_of(3) {
        return Err(capacity(indices.len()));
    }
    let triangles = indices.len() / 3;
    let words = read_words(reader, triangles)?;
    require_header(&words, triangles, expected)?;
    let per_triangle = if triangles == 0 {
        0
    } else {
        expected / triangles as u64
    };
    for (triangle, record) in words[4..].as_chunks::<2>().0.iter().enumerate() {
        require_triangle(indices, triangle, *record, per_triangle)?;
    }
    Ok(())
}
fn read_words(reader: &impl StatusReader, triangles: usize) -> Result<Vec<u32>, RenderError> {
    let length = word_count(triangles)? * 4;
    let bytes = reader.read_status(length)?;
    if bytes.len() != length {
        return Err(Failure::GpuReadback {
            expected_bytes: length,
            observed_bytes: bytes.len(),
        }
        .into());
    }
    Ok(bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|w| u32::from_ne_bytes(*w))
        .collect())
}
fn require_header(words: &[u32], triangles: usize, expected: u64) -> Result<(), RenderError> {
    let divisible = (triangles == 0 && expected == 0)
        || (triangles != 0 && expected != 0 && expected.is_multiple_of(triangles as u64));
    if words[0] as usize == triangles
        && words[1] == SCHEMA
        && u64::from(words[2]) == expected
        && words[3] == 0
        && divisible
    {
        return Ok(());
    }
    Err(Failure::GpuCompletion {
        expected_triangles: triangles,
        expected_invocations: expected,
        observed_header: [words[0], words[1], words[2], words[3]],
    }
    .into())
}
fn require_triangle(
    indices: &[u32],
    triangle: usize,
    record: [u32; 2],
    expected: u64,
) -> Result<(), RenderError> {
    let source = &indices[triangle * 3..triangle * 3 + 3];
    let vertex_indices = [source[0], source[1], source[2]];
    if record[0] & DONE == 0 || record[0] & !(DONE | EMPTY) != 0 {
        return Err(Failure::GpuTriangle {
            triangle,
            vertex_indices,
            flags: record[0],
        }
        .into());
    }
    if u64::from(record[1]) != expected {
        return Err(Failure::GpuPrimitiveCount {
            triangle,
            vertex_indices,
            expected,
            observed: record[1],
        }
        .into());
    }
    Ok(())
}
#[cfg(test)]
#[path = "window_depth_status_protocol_tests.rs"]
mod tests;
