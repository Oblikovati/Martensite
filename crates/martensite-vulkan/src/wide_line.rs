//! Aliased wide lines beyond the device line-width range, drawn by the native GL rule.
//!
//! Native OpenGL draws any width (llvmpipe and radeonsi agree byte for byte), while RADV limits
//! lineWidthRange to [0, 8]. Both native drivers (the recorded wide_lines capture, Mesa
//! `lp_setup_line`) shift an aliased segment of integer width w by -(w - 1) / 2 in the minor
//! direction (OpenGL 4.6, 14.5.2.1) and cover a run of w pixels in each of the columns (rows for
//! y-major segments) chosen by the diamond-exit tests of the original endpoints. The Vulkan
//! driver draws a one-pixel line w times through viewports shifted by -(w - 1) / 2 + k, after
//! classifying each segment by its window-space major axis and moving its endpoints to its
//! native columns (`crate::wide_line_columns`).
use crate::{PrimitiveDraw, wide_line_columns::SegmentColumns};

/// How a wide draw is replayed: its segments as one-pixel lines, x-major ones first.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WideLineReplay {
    /// The replayed vertices: each segment's endpoints moved to its native columns, or the
    /// original vertices when per-vertex attributes forbid moving them.
    pub vertices: Vec<crate::RenderVertex>,
    /// Segment indices reordered so that the first `x_major` indices are x-major segments.
    pub indices: Vec<u32>,
    pub x_major: usize,
    /// The GL integer width: the number of one-pixel replicas per segment.
    pub replicas: u32,
}

impl WideLineReplay {
    /// Classify the segments of `draw` on a framebuffer of `extent` pixels for `width`.
    /// Example: `WideLineReplay::new(&draw, [64, 64], 9.)`.
    pub fn new(draw: &PrimitiveDraw, extent: [u32; 2], width: f32) -> Self {
        let replicas = gl_line_width(width);
        let offsets = replica_offsets(replicas).collect::<Vec<_>>();
        let (vertices, segments) = replay_segments(draw, extent, &offsets);
        let (mut x_major, y_major): (Vec<_>, Vec<_>) =
            segments.into_iter().partition(|segment| segment.1);
        let count = x_major.len() * 2;
        x_major.extend(y_major);
        Self {
            vertices,
            indices: x_major.into_iter().flat_map(|segment| segment.0).collect(),
            x_major: count,
            replicas,
        }
    }

    /// The window-space pixel offsets (towards GL up for x-major, right for y-major) of each
    /// replica: GL shifts the segment by -(w - 1) / 2 before widening, so an even width uses
    /// half-pixel offsets. Example: width 12 gives -5.5, -4.5, ..., 5.5.
    pub fn offsets(&self) -> impl Iterator<Item = f32> + '_ {
        replica_offsets(self.replicas)
    }
}

fn replica_offsets(replicas: u32) -> impl Iterator<Item = f32> {
    let base = -((replicas - 1) as f32) / 2.;
    (0..replicas).map(move |k| base + k as f32)
}

type ReplaySegment = ([u32; 2], bool);

/// The replay vertices and each segment's indices with its window major axis.
fn replay_segments(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    offsets: &[f32],
) -> (Vec<crate::RenderVertex>, Vec<ReplaySegment>) {
    let pairs = draw.indices.as_chunks::<2>().0;
    let classified = pairs
        .iter()
        .map(|&pair| (pair, is_x_major(draw, pair, extent)));
    if !endpoints_movable(draw) {
        return (draw.vertices.clone(), classified.collect());
    }
    let mut vertices = Vec::with_capacity(pairs.len() * 2);
    let mut segments = Vec::with_capacity(pairs.len());
    for (pair, x_major) in classified {
        let ends =
            match crate::wide_line_columns::segment_columns(draw, pair, extent, (x_major, offsets))
            {
                SegmentColumns::Empty => continue,
                SegmentColumns::Unchanged => pair.map(|index| draw.vertices[index as usize]),
                SegmentColumns::Moved(ends) => ends,
            };
        let first = vertices.len() as u32;
        vertices.extend(ends);
        segments.push(([first, first + 1], x_major));
    }
    (vertices, segments)
}

/// Endpoints can move along a segment only when no per-vertex attribute besides the colour
/// (lighting normals, flat-source maps, projected dashes) refers to the original vertices.
fn endpoints_movable(draw: &PrimitiveDraw) -> bool {
    draw.lighting
        .as_ref()
        .is_none_or(|lighting| lighting.normals.is_empty())
        && draw.native_flat_sources.is_empty()
        && draw.projected_dash.is_none()
}

/// GL rounds the aliased width to the nearest integer; zero acts as one.
fn gl_line_width(width: f32) -> u32 {
    (width.round() as u32).max(1)
}

/// GL: a segment is x-major when |dx| >= |dy| in window coordinates.
fn is_x_major(draw: &PrimitiveDraw, [a, b]: [u32; 2], extent: [u32; 2]) -> bool {
    let (from, to) = visible_part(clip_point(draw, a), clip_point(draw, b));
    let delta = |axis: usize| {
        let ndc = to[axis] / to[3] - from[axis] / from[3];
        (ndc * extent[axis] as f32 / 2.).abs()
    };
    delta(0) >= delta(1)
}

/// Clip-space position of one vertex: `clip_transform` is column-major, as in the shaders.
fn clip_point(draw: &PrimitiveDraw, index: u32) -> [f32; 4] {
    let vertex = &draw.vertices[index as usize];
    let [x, y, z] = vertex.position;
    let source = [x, y, z, vertex.weight];
    let m = &draw.clip_transform;
    std::array::from_fn(|row| (0..4).map(|col| m[col * 4 + row] * source[col]).sum())
}

/// The part of a segment in front of the GL near plane (z >= -w), whose window slope GL uses.
fn visible_part(from: [f32; 4], to: [f32; 4]) -> ([f32; 4], [f32; 4]) {
    let distance = |p: [f32; 4]| p[2] + p[3];
    let (d0, d1) = (distance(from), distance(to));
    if (d0 >= 0. && d1 >= 0.) || (d0 < 0. && d1 < 0.) {
        return (from, to);
    }
    let t = d0 / (d0 - d1);
    let cut: [f32; 4] = std::array::from_fn(|i| from[i] + t * (to[i] - from[i]));
    if d0 < 0. { (cut, to) } else { (from, cut) }
}

#[cfg(test)]
#[path = "wide_line_tests.rs"]
mod tests;
