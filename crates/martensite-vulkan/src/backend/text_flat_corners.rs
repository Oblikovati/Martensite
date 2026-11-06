//! Triangle-local packing preserves semantic geometry while core flat varyings carry corners.
use crate::{PrimitiveDraw, RenderError, RenderTextInterpolationFailure as Failure};
pub(super) const STRIDE: usize = 112;
const BASE_STRIDE: usize = 40;

pub(super) struct FlatTextEncoding {
    pub(super) vertices: Vec<u8>,
    pub(super) indices: Vec<u32>,
    pub(super) sources: Vec<(usize, usize)>,
}

pub(super) fn admit(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if valid_count(draw.indices.len()) && valid_sources(draw) && draw.user_program.is_none() {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface { actual: format!(
        "flat text count={}, vertices={}, UVs={:?}, user_program={}; expected bounded u32 triangles and one UV per vertex without a user program",
        draw.indices.len(), draw.vertices.len(),
        draw.textured_style.as_ref().map(|style| style.uv.len()), draw.user_program.is_some(),
    ) }.into())
}

fn valid_count(count: usize) -> bool {
    count <= u32::MAX as usize && count.checked_mul(STRIDE).is_some()
}

fn valid_sources(draw: &PrimitiveDraw) -> bool {
    draw.textured_style
        .as_ref()
        .is_some_and(|style| style.uv.len() == draw.vertices.len())
        && draw
            .indices
            .iter()
            .all(|&index| (index as usize) < draw.vertices.len())
}

pub(super) fn encode(base: &[u8], indices: &[u32]) -> FlatTextEncoding {
    let mut vertices = Vec::with_capacity(indices.len() * STRIDE);
    for triangle in indices.as_chunks::<3>().0.iter() {
        let payload = corner_payload(base, triangle);
        for &index in triangle {
            vertices.extend_from_slice(vertex_record(base, index));
            vertices.extend_from_slice(&payload);
        }
    }
    FlatTextEncoding {
        vertices,
        indices: (0..indices.len() as u32).collect(),
        sources: source_pairs(indices),
    }
}

fn source_pairs(indices: &[u32]) -> Vec<(usize, usize)> {
    indices
        .iter()
        .map(|&index| (index as usize, index as usize))
        .collect()
}

fn corner_payload(base: &[u8], triangle: &[u32]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(STRIDE - BASE_STRIDE);
    for &index in triangle {
        payload.extend_from_slice(&vertex_record(base, index)[..16]);
    }
    for &index in triangle {
        payload.extend_from_slice(&vertex_record(base, index)[32..40]);
    }
    payload
}

fn vertex_record(base: &[u8], index: u32) -> &[u8] {
    let offset = index as usize * BASE_STRIDE;
    &base[offset..offset + BASE_STRIDE]
}

#[cfg(test)]
#[path = "text_flat_corners_tests.rs"]
mod tests;
