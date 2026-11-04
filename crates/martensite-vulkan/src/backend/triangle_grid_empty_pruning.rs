//! Only sample-empty source refusals get private zero-area carriers; no covered input is waived.
use super::*;

pub(super) fn private_geometry(
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    bits: u32,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    let clips = projected_clips(draw);
    private_geometry_with(draw, "V3", |draw, indices, ordinal| {
        empty_refusal(draw, indices, &clips, viewport, bits, ordinal)
    })
}

pub(super) fn private_native_grid_v4_geometry(
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
) -> Result<Option<PrimitiveDraw>, RenderError> {
    let clips = projected_clips(draw);
    private_geometry_with(draw, "V4", |draw, indices, ordinal| {
        native_grid_v4_empty(draw, indices, &clips, viewport, ordinal)
    })
}

fn private_geometry_with(
    draw: &PrimitiveDraw,
    version: &str,
    mut should_replace: impl FnMut(&PrimitiveDraw, &[u32], usize) -> Result<bool, RenderError>,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    let mut replacement = None;
    for (ordinal, indices) in draw.indices.as_chunks::<3>().0.iter().enumerate() {
        if should_replace(draw, indices, ordinal)? {
            let encoded = replacement.get_or_insert_with(|| draw.indices.clone());
            encoded[ordinal * 3..ordinal * 3 + 3].fill(indices[0]);
        }
    }
    if replacement.is_some() {
        require_private_carrier_pass(draw, version)?;
    }
    Ok(replacement.map(|indices| draw.with_replacement_geometry(draw.vertices.clone(), indices)))
}

fn native_grid_v4_empty(
    draw: &PrimitiveDraw,
    indices: &[u32],
    projected: &[[f32; 4]],
    viewport: [f32; 4],
    ordinal: usize,
) -> Result<bool, RenderError> {
    // Whole frames contain collapsed triangles (poles, repeated nodes) and vertices beyond the
    // viewport inside the guard band. Neither can leave V4 unable to draw the frame: an empty grid
    // triangle becomes a carrier and any other positive-w triangle keeps its nearest-even Q12
    // rounding. Only a vertex the projection cannot place stays a refusal.
    let (clips, _) = triangle_inputs(draw, indices, projected);
    if clips
        .iter()
        .any(|c| !c.iter().all(|v| v.is_finite()) || c[3] <= 0.0)
    {
        return Err(super::invalid_triangle(ordinal, clips));
    }
    let intervals = super::super::triangle_grid_projection_intervals::projected_q12_intervals(
        draw, indices, viewport,
    );
    if intervals.is_some_and(|points| {
        sample::q12_interval_vertices_empty(points) == sample::Q12EmptyCertificate::Empty
    }) {
        return Ok(true);
    }
    let grid = super::grid_triangle(clips, viewport)
        .ok_or_else(|| super::invalid_triangle(ordinal, clips))?;
    Ok(grid.signed_area() == 0)
}

fn empty_refusal(
    draw: &PrimitiveDraw,
    indices: &[u32],
    projected: &[[f32; 4]],
    viewport: [f32; 4],
    bits: u32,
    ordinal: usize,
) -> Result<bool, RenderError> {
    let (clips, errors) = triangle_inputs(draw, indices, projected);
    match admit_triangle(clips, errors, viewport, bits, ordinal) {
        Ok(()) => Ok(false),
        Err(error) if empty_grid_refusal(&error, clips, errors, viewport) => Ok(true),
        Err(error) => Err(error),
    }
}

fn empty_grid_refusal(
    error: &RenderError,
    clips: [[f32; 4]; 3],
    errors: [Option<[f64; 4]>; 3],
    viewport: [f32; 4],
) -> bool {
    matches!(
        error.triangle_coverage_failure(),
        Some(Failure::RasterGridDegenerate { .. })
    ) && sample_box_empty(clips, errors, viewport)
}

#[path = "triangle_grid_empty_sample.rs"]
mod sample;

fn sample_box_empty(
    clips: [[f32; 4]; 3],
    errors: [Option<[f64; 4]>; 3],
    viewport: [f32; 4],
) -> bool {
    let observations: Option<Vec<_>> = clips
        .into_iter()
        .zip(errors)
        .map(|(clip, error)| observed_window(clip, error, viewport))
        .collect();
    let Some(observations) = observations else {
        return false;
    };
    let points = std::array::from_fn(|i| observations[i].0);
    let radii = std::array::from_fn(|i| observations[i].1);
    sample::sample_box_empty(points, radii)
}

fn observed_window(
    clip: [f32; 4],
    error: Option<[f64; 4]>,
    viewport: [f32; 4],
) -> Option<([f64; 2], [f64; 2])> {
    let point = super::super::phong_interpolation_math::window_point(clip, viewport)?;
    let error = error?;
    let radii = [
        window_error(clip, error, viewport, 0)?,
        window_error(clip, error, viewport, 1)?,
    ];
    Some((point.map(f64::from), radii))
}

fn require_private_carrier_pass(draw: &PrimitiveDraw, version: &str) -> Result<(), RenderError> {
    if !draw.quad_corners.is_empty()
        || draw.fixed_grid_point
        || draw
            .lighting
            .as_ref()
            .is_some_and(|lighting| lighting.fixed.is_some())
    {
        return unsupported(format!(
            "{version} empty carrier downstream geometry rewriting"
        ));
    }
    if draw.annotation || draw.stencil_rect || draw.text_shadow_bias || draw.text_logic_xor {
        return unsupported(format!(
            "{version} empty carrier annotation/stencil/text auxiliary pass"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "triangle_grid_empty_pruning_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "triangle_grid_empty_v4_tests.rs"]
mod v4_tests;

#[cfg(test)]
#[path = "triangle_grid_empty_exact_tests.rs"]
mod exact_tests;
