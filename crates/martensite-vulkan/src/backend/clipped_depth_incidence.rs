//! Bounded GPU failure observations use the existing fenced completion allocation.
use crate::{RenderError, RenderWindowDepthFailure as Failure};
pub(super) const STRIDE: usize = 64;
const TRIANGLE_LIMIT: usize = 32768;
const RECORD_LIMIT: usize = 32;
const MAGIC: u32 = 0x4349_4431;
fn refused(actual: impl std::fmt::Debug) -> RenderError {
    Failure::UnsupportedInterface {
        actual: format!(
            "V4 incidence observation {actual:?}; expected bounded complete same-generation records"
        ),
    }
    .into()
}
pub(super) fn byte_count(triangles: usize) -> Result<usize, RenderError> {
    if triangles > TRIANGLE_LIMIT {
        return Err(refused((triangles, TRIANGLE_LIMIT)));
    }
    Ok(triangles * STRIDE * 4)
}
pub(super) fn initial(triangles: usize) -> Result<Vec<u8>, RenderError> {
    let mut words = vec![0u32; byte_count(triangles)? / 4];
    for record in words.chunks_exact_mut(STRIDE) {
        record[0] = u32::MAX;
    }
    Ok(words.into_iter().flat_map(u32::to_ne_bytes).collect())
}
fn record_words(bytes: &[u8], triangles: usize) -> Result<Vec<[u32; STRIDE]>, RenderError> {
    if triangles > TRIANGLE_LIMIT || bytes.len() != triangles * STRIDE * 4 {
        return Err(refused((triangles, bytes.len())));
    }
    let words: Vec<_> = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|w| u32::from_ne_bytes(*w))
        .collect();
    Ok(words.as_chunks::<STRIDE>().0.to_vec())
}
fn selected(records: &[[u32; STRIDE]]) -> Vec<usize> {
    records
        .iter()
        .enumerate()
        .filter_map(|(i, row)| (row[0] != u32::MAX).then_some(i))
        .collect()
}
fn require_record(record: &[u32; STRIDE], invocation_count: u32) -> Result<(), RenderError> {
    if record[0] >= 0x8000_0000 || record[1] != MAGIC || invocation_count != 1 {
        return Err(refused((record[0], record[1], invocation_count)));
    }
    if record[62..] != [0, 0] {
        return Err(refused(record[62..].to_vec()));
    }
    Ok(())
}
fn record_json(triangle: usize, indices: &[u32], record: &[u32; STRIDE]) -> String {
    format!(
        "{{\"primitive\":{triangle},\"source_indices\":{:?},\"sample_key\":{},\"raw_gl_clip_f32_bits\":{:?},\"actual_vs_clip_distance_f32_bits\":{:?},\"reference_distance_f32_bits\":{:?}}}",
        &indices[triangle * 3..triangle * 3 + 3],
        record[0],
        &record[2..14],
        &record[14..38],
        &record[38..62]
    )
}
pub(super) fn report(
    draw: usize,
    bytes: &[u8],
    prefix: usize,
    indices: &[u32],
) -> Result<String, RenderError> {
    require_prefix(bytes, prefix, indices.len())?;
    let triangles = indices.len() / 3;
    let records = record_words(&bytes[prefix..], triangles)?;
    let failed = selected(&records);
    let rows = checked_rows(bytes, indices, &records, &failed)?;
    Ok(report_json(draw, prefix_words(bytes), failed.len(), rows))
}
fn prefix_words(bytes: &[u8]) -> [u32; 4] {
    std::array::from_fn(|i| u32::from_ne_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap()))
}
fn report_json(draw: usize, header: [u32; 4], failed: usize, rows: Vec<String>) -> String {
    format!(
        "{{\"kind\":\"clipped-depth-failure-incidences\",\"draw\":{draw},\"status_header\":{header:?},\"failed_primitives\":{failed},\"record_limit\":{RECORD_LIMIT},\"sample_key_contract\":\"((((y*1024+x)*16+sy)*16+sx)*8+sampleID); offset=(sx,sy)/16\",\"records\":[{}]}}",
        rows.join(",")
    )
}

fn checked_rows(
    bytes: &[u8],
    indices: &[u32],
    records: &[[u32; STRIDE]],
    failed: &[usize],
) -> Result<Vec<String>, RenderError> {
    let mut rows = Vec::new();
    for &triangle in failed.iter().take(RECORD_LIMIT) {
        let offset = 16 + triangle * 8 + 4;
        let count = u32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap());
        require_record(&records[triangle], count)?;
        rows.push(record_json(triangle, indices, &records[triangle]));
    }
    Ok(rows)
}
fn require_prefix(bytes: &[u8], prefix: usize, index_count: usize) -> Result<(), RenderError> {
    let triangles = index_count / 3;
    if index_count % 3 != 0
        || triangles > TRIANGLE_LIMIT
        || prefix != 16 + triangles * 8
        || bytes.len() < prefix
    {
        return Err(refused((index_count, prefix, bytes.len())));
    }
    require_header(prefix_words(bytes), triangles)
}
fn require_header(header: [u32; 4], triangles: usize) -> Result<(), RenderError> {
    if header[0] as usize != triangles
        || header[1] != super::window_depth_status_protocol::SCHEMA
        || header[2] as usize != triangles
    {
        return Err(refused(header));
    }
    Ok(())
}
fn without_tail(bytes: &[u8], prefix: usize, tail: &[u8]) -> Result<Vec<u8>, RenderError> {
    let end = prefix
        .checked_add(tail.len())
        .ok_or_else(|| refused(prefix))?;
    let observed = bytes.get(prefix..end).ok_or_else(|| refused(bytes.len()))?;
    super::clipped_depth_uniforms::require_tail(tail, observed)?;
    let mut selected = bytes[..prefix].to_vec();
    selected.extend_from_slice(&bytes[end..]);
    Ok(selected)
}
fn add_constants(mut report: String, tail: &[u8]) -> String {
    let constants: Vec<_> = tail
        .as_chunks::<4>()
        .0
        .iter()
        .map(|w| u32::from_ne_bytes(*w))
        .collect();
    report.pop();
    report.push_str(&format!(",\"immutable_reference_words\":{constants:?}}}"));
    report
}
pub(super) fn checked_report(
    draw: usize,
    bytes: &[u8],
    prefix: usize,
    tail: &[u8],
    indices: &[u32],
) -> Result<String, RenderError> {
    let selected = without_tail(bytes, prefix, tail)?;
    Ok(add_constants(
        report(draw, &selected, prefix, indices)?,
        tail,
    ))
}
pub(super) fn unavailable(draw: usize, error: &RenderError) {
    let hex: String = error
        .to_string()
        .bytes()
        .map(|b| format!("{b:02x}"))
        .collect();
    eprintln!(
        "{{\"kind\":\"clipped-depth-incidence-unavailable\",\"draw\":{draw},\"error_utf8_hex\":\"{hex}\"}}"
    );
}

#[cfg(test)]
#[path = "clipped_depth_incidence_tests.rs"]
mod tests;
