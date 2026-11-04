//! Clip far line segments to a guard band before submission.
//!
//! Desktop OpenGL on Mesa clips lines to the view volume in the draw module before llvmpipe
//! sets them up (`st_atom_rasterizer.c`: `point_line_tri_clip` only for GLES). lavapipe sets
//! `point_line_tri_clip`, so `draw_pipe_clip.c` (`clip_line_guard_xy`) hands unclipped lines to
//! `lp_setup_line.c`, which drops or misfills a line whose window bounding box spans 2^15 px or
//! more (measured on Mesa 26.1.6: a vertical line spanning 32,000 px draws, 33,000 px draws
//! nothing, 36,000 px fills stray pixels). The infinite isolines of bug129_1 and the far lines
//! of bug218 cross that limit. Such segments are clipped here, in homogeneous coordinates, to
//! a band far outside the viewport, so the visible part keeps its rasterization; segments that
//! stay below the limit are submitted unchanged.
use crate::{PrimitiveDraw, PrimitiveTopology, RenderVertex};
use std::sync::Arc;

/// Window span from which a segment is clipped; half the measured llvmpipe limit.
const SAFE_LINE_SPAN: f32 = 16384.;

/// The submitted geometry of a clipped draw, for renderers whose driver clips lines natively.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GuardBandSource {
    pub(crate) vertices: Vec<RenderVertex>,
    pub(crate) indices: Vec<u32>,
}

/// Clip every eligible line draw whose segments reach `SAFE_LINE_SPAN`.
/// Example: `clip_far_lines(&mut frame_draws, [width, height])`.
pub(crate) fn clip_far_lines(draws: &mut [PrimitiveDraw], extent: [u32; 2]) {
    let band = GuardBand::new(extent);
    for draw in draws.iter_mut().filter(|draw| clippable(draw)) {
        if band.needs_clipping(draw) {
            *draw = band.clipped(draw);
        }
    }
}

/// Lines whose per-vertex state is only position, weight and colour; side arrays indexed by
/// vertex, prepared dashes and wide-line corners keep their own geometry.
pub(crate) fn clippable(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Lines
        && draw.line_style.is_some()
        && draw.projected_dash.is_none()
        && draw.quad_corners.is_empty()
        && draw.native_flat_sources.is_empty()
        && draw.lighting.is_none()
        && draw.surface_texture.is_none()
        && draw.textured_style.is_none()
}

/// The band |x| <= scale_x * w, |y| <= scale_y * w in clip space.
struct GuardBand {
    extent: [f32; 2],
    scale: [f32; 2],
}

impl GuardBand {
    /// A band `SAFE_LINE_SPAN` px wide around the viewport (the viewport itself when larger).
    fn new(extent: [u32; 2]) -> Self {
        let extent = extent.map(|v| v.max(1) as f32);
        let scale = extent.map(|size| (SAFE_LINE_SPAN / size).max(1.));
        Self { extent, scale }
    }
    fn needs_clipping(&self, draw: &PrimitiveDraw) -> bool {
        draw.indices.as_chunks::<2>().0.iter().any(|&[a, b]| {
            let ends = [a, b].map(|i| clip_position(draw, &draw.vertices[i as usize]));
            self.window_span(ends) >= SAFE_LINE_SPAN
        })
    }
    /// The larger window extent of a segment; infinite when an end is not in front (w <= 0).
    fn window_span(&self, ends: [[f32; 4]; 2]) -> f32 {
        if ends
            .iter()
            .any(|clip| clip[3] <= 0. || !clip.iter().all(|v| v.is_finite()))
        {
            return f32::INFINITY;
        }
        let window = |clip: [f32; 4], axis: usize| clip[axis] / clip[3] * self.extent[axis] / 2.;
        (0..2)
            .map(|axis| (window(ends[0], axis) - window(ends[1], axis)).abs())
            .fold(0., f32::max)
    }
    fn clipped(&self, draw: &PrimitiveDraw) -> PrimitiveDraw {
        let mut vertices = Vec::new();
        for &[a, b] in draw.indices.as_chunks::<2>().0 {
            let ends = [a, b].map(|i| draw.vertices[i as usize]);
            if let Some(range) = self.visible_range(draw, ends) {
                vertices.extend(range.map(|t| interpolate(ends, t)));
            }
        }
        let indices = (0..vertices.len() as u32).collect();
        let mut clipped = draw.with_replacement_geometry(vertices, indices);
        clipped.guard_band_source = Some(retained_source(draw));
        clipped
    }
    /// Liang-Barsky parameter range of a segment inside the band, `None` when outside.
    fn visible_range(&self, draw: &PrimitiveDraw, ends: [RenderVertex; 2]) -> Option<[f32; 2]> {
        let clip = ends.map(|vertex| clip_position(draw, &vertex));
        let mut range = [0f32, 1.];
        for (axis, sign) in [(0, 1.), (0, -1.), (1, 1.), (1, -1.)] {
            let distance = clip.map(|c| self.scale[axis] * c[3] - sign * c[axis]);
            range = narrow(range, distance)?;
        }
        Some(range)
    }
}

/// A frame replayed through preparation again keeps the first, truly submitted geometry.
fn retained_source(draw: &PrimitiveDraw) -> Arc<GuardBandSource> {
    draw.guard_band_source.clone().unwrap_or_else(|| {
        Arc::new(GuardBandSource {
            vertices: draw.vertices.clone(),
            indices: draw.indices.clone(),
        })
    })
}

/// Keep the part of `range` where the linear `distance` is non-negative.
fn narrow([low, high]: [f32; 2], [d0, d1]: [f32; 2]) -> Option<[f32; 2]> {
    if d0 < 0. && d1 < 0. {
        return None;
    }
    if d0 >= 0. && d1 >= 0. {
        return Some([low, high]);
    }
    let t = d0 / (d0 - d1);
    let range = if d0 < 0. {
        [low.max(t), high]
    } else {
        [low, high.min(t)]
    };
    (range[0] <= range[1]).then_some(range)
}

/// Homogeneous interpolation of object position, weight and colour.
fn interpolate([a, b]: [RenderVertex; 2], t: f32) -> RenderVertex {
    let mix = |x: f32, y: f32| x + (y - x) * t;
    RenderVertex {
        position: std::array::from_fn(|i| mix(a.position[i], b.position[i])),
        weight: mix(a.weight, b.weight),
        color: std::array::from_fn(|i| mix(a.color[i], b.color[i])),
    }
}

/// `clip_transform * (position, weight)`, the column-major vertex-shader product.
fn clip_position(draw: &PrimitiveDraw, vertex: &RenderVertex) -> [f32; 4] {
    let m = &draw.clip_transform;
    let v = [
        vertex.position[0],
        vertex.position[1],
        vertex.position[2],
        vertex.weight,
    ];
    std::array::from_fn(|row| (0..4).map(|column| m[column * 4 + row] * v[column]).sum())
}

#[cfg(test)]
#[path = "line_guard_band_tests.rs"]
mod tests;
