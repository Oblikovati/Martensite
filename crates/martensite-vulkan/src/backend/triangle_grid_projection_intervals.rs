//! Per-vertex Q12 intervals for conservative logical-area admission.
use crate::{
    PrimitiveDraw, RenderViewportGridPrecision, triangle_grid_coverage::GridVertexInterval,
};

pub(super) fn projected_q12_intervals(
    draw: &PrimitiveDraw,
    indices: &[u32],
    viewport: [f32; 4],
) -> Option<[GridVertexInterval; 3]> {
    let vertices: [Option<GridVertexInterval>; 3] = std::array::from_fn(|corner| {
        let index = usize::try_from(*indices.get(corner)?).ok()?;
        projected_vertex_interval(draw, index, viewport)
    });
    Some([vertices[0]?, vertices[1]?, vertices[2]?])
}

fn projected_vertex_interval(
    draw: &PrimitiveDraw,
    vertex: usize,
    viewport: [f32; 4],
) -> Option<GridVertexInterval> {
    let clip = crate::window_depth_bounds::clip_transform_bounds(
        draw.clip_transform,
        *draw.vertices.get(vertex)?,
    )?;
    projected_clip_bounds(clip, viewport)
}

/// Project already bounded clip coordinates, including vertices created by clipping.
/// For example, `projected_clip_bounds(clipped_vertex, viewport)` preserves outward bounds.
pub(super) fn projected_clip_bounds(
    clip: [crate::window_depth_bounds::DepthShaderBound; 4],
    viewport: [f32; 4],
) -> Option<GridVertexInterval> {
    let one = bound(1.0)?;
    let half = bound(0.5)?;
    let x = projected_coordinate(clip[0], clip[3], viewport[0], viewport[2], one, half)?;
    let y = projected_coordinate(clip[1], clip[3], viewport[1], viewport[3], one, half)?;
    GridVertexInterval::new(snap_q12(x)?, snap_q12(y)?)
}

fn projected_coordinate(
    coordinate: crate::window_depth_bounds::DepthShaderBound,
    w: crate::window_depth_bounds::DepthShaderBound,
    origin: f32,
    extent: f32,
    one: crate::window_depth_bounds::DepthShaderBound,
    half: crate::window_depth_bounds::DepthShaderBound,
) -> Option<crate::window_depth_bounds::DepthShaderBound> {
    coordinate
        .divide(w)?
        .add(one)?
        .multiply(bound(extent)?)
        .and_then(|scaled| scaled.multiply(half))?
        .add(bound(origin)?)
}

fn snap_q12(bound: crate::window_depth_bounds::DepthShaderBound) -> Option<[i32; 2]> {
    let endpoints = bound.endpoints().map(q12_integer);
    Some([endpoints[0]?, endpoints[1]?])
}

fn q12_integer(value: f64) -> Option<i32> {
    let snapped = RenderViewportGridPrecision::Fractional12.snap(value).ok()?;
    let scaled = snapped * 4096.0;
    if !scaled.is_finite() || scaled.fract() != 0.0 || scaled.abs() > 1_073_741_823.0 {
        return None;
    }
    Some(scaled as i32)
}

fn bound(value: f32) -> Option<crate::window_depth_bounds::DepthShaderBound> {
    crate::window_depth_bounds::DepthShaderBound::source(value)
}

#[cfg(test)]
#[path = "triangle_grid_projection_intervals_tests.rs"]
mod tests;
