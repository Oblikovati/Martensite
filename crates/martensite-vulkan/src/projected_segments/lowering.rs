//! Ordinary segment preparation owns no driver, cache, or mutable source geometry.
use super::{
    DashBehavior, PreparedAxis, ProjectedFailure, ProjectedSegments, SegmentInterval, exact, failed,
};
use crate::{PrimitiveDraw, RenderVertex};
use num_traits::Zero;
use std::sync::Arc;
pub(super) fn unsupported(draw: &PrimitiveDraw) -> bool {
    // Preflight and later frame validation must apply exactly the same domain.
    unsupported_reason(draw).is_some()
}
pub(super) fn unsupported_reason(draw: &PrimitiveDraw) -> Option<&'static str> {
    geometry_refusal(draw)
        .or_else(|| surface_refusal(draw))
        .or_else(|| pipeline_refusal(draw))
}
fn state_reason(states: &[(bool, &'static str)]) -> Option<&'static str> {
    // Ordered tags diagnose state without exposing or copying source geometry.
    states
        .iter()
        .find_map(|(enabled, reason)| enabled.then_some(*reason))
}
fn geometry_refusal(draw: &PrimitiveDraw) -> Option<&'static str> {
    state_reason(&[
        (draw.line_style.is_none(), "independent-line=false"),
        (draw.clipping.is_some(), "world-clipping=true"),
        (draw.lighting.is_some(), "lighting=true"),
        (draw.textured_style.is_some(), "textured-style=true"),
        (draw.point_style.is_some(), "point-style=true"),
        (draw.mesh_edges.is_some(), "mesh-edges=true"),
    ])
}
fn surface_refusal(draw: &PrimitiveDraw) -> Option<&'static str> {
    state_reason(&[
        (draw.surface_texture.is_some(), "surface-texture=true"),
        (
            draw.surface_texture_unavailable.is_some(),
            "surface-texture-unavailable=true",
        ),
        (draw.surface_cutoff.is_some(), "surface-cutoff=true"),
        (draw.unlit_back_color.is_some(), "unlit-back-color=true"),
    ])
}
fn pipeline_refusal(draw: &PrimitiveDraw) -> Option<&'static str> {
    state_reason(&[
        (
            draw.capping_pass != crate::RenderCappingPass::None,
            "capping-pass=true",
        ),
        (draw.source_line_strip, "line-strip=true"),
        (draw.fixed_function_lighting, "fixed-function-lighting=true"),
        (draw.polygon_edge_style.is_some(), "polygon-edge-style=true"),
    ])
}

pub(super) fn prepare(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    limit: f64,
    identities: &[u64],
) -> Result<ProjectedSegments, ProjectedFailure> {
    validate_request(draw, extent, limit, identities)?;
    let mut draws = Vec::new();
    let mut intervals = Vec::new();
    for (ordinal, pair) in draw.indices.as_chunks::<2>().0.iter().enumerate() {
        if let Some((prepared, interval)) =
            related_pair(draw, pair, ordinal, identities[ordinal], extent, limit)?
        {
            draws.push(prepared);
            intervals.push(interval);
        }
    }
    Ok(completed(draws, intervals, limit, identities))
}
fn related_pair(
    source: &PrimitiveDraw,
    pair: &[u32],
    ordinal: usize,
    identity: u64,
    extent: [u32; 2],
    limit: f64,
) -> Result<Option<(PrimitiveDraw, SegmentInterval)>, ProjectedFailure> {
    prepare_pair(source, pair, ordinal, identity, extent, limit).map_err(|mut error| {
        error.source_identity = Some(identity);
        error
    })
}

fn completed(
    draws: Vec<PrimitiveDraw>,
    intervals: Vec<SegmentInterval>,
    limit: f64,
    identities: &[u64],
) -> ProjectedSegments {
    ProjectedSegments {
        draws: Arc::from(draws),
        intervals: Arc::from(intervals),
        behavior: DashBehavior::FiniteProjectiveV2,
        pixel_error_limit: Some(limit),
        source_identities: Arc::from(identities),
    }
}

fn validate_request(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    limit: f64,
    identities: &[u64],
) -> Result<(), ProjectedFailure> {
    validate_options(draw, extent, limit)?;
    let expected = draw.indices.len() / 2;
    if identities.len() != expected {
        let cause = format!(
            "source identities {}: expected one per original pair {expected}",
            identities.len()
        );
        return Err(failed(None, cause, false));
    }
    Ok(())
}
fn validate_options(
    draw: &PrimitiveDraw,
    extent: [u32; 2],
    limit: f64,
) -> Result<(), ProjectedFailure> {
    validate_supported_state(draw)?;
    if !extent.contains(&0) && limit.is_finite() && limit > 0. {
        return Ok(());
    }
    Err(failed(
        None,
        format!(
            "state/extent {extent:?}/pixel limit {limit:?}: expected ordinary unlit segments, nonzero extent and positive finite error cap"
        ),
        false,
    ))
}

fn validate_supported_state(draw: &PrimitiveDraw) -> Result<(), ProjectedFailure> {
    if let Some(reason) = unsupported_reason(draw) {
        let cause = format!(
            "unsupported draw state {reason}: expected ordinary unlit independent segments without clipping, textures, capping or polygon edges"
        );
        return Err(failed(None, cause, false));
    }
    Ok(())
}

fn prepare_pair(
    source: &PrimitiveDraw,
    pair: &[u32],
    ordinal: usize,
    identity: u64,
    extent: [u32; 2],
    limit: f64,
) -> Result<Option<(PrimitiveDraw, SegmentInterval)>, ProjectedFailure> {
    let ends = pair_ends(source, pair, ordinal)?;
    let Some(parameters) = exact::clip(&ends[0], &ends[1], source.depth_clamp) else {
        return Ok(None);
    };
    let points = clipped_points(&ends, &parameters);
    let (upload, axis, error) =
        accepted_points(&points, extent, limit, ordinal, source.depth_clamp)?;
    Ok(Some((
        prepared_draw(source, upload, extent, axis, pair[0]),
        source_interval(ordinal, identity, parameters, error),
    )))
}
fn pair_ends(
    source: &PrimitiveDraw,
    pair: &[u32],
    ordinal: usize,
) -> Result<[exact::Point; 2], ProjectedFailure> {
    let vertices = pair_vertices(source, pair);
    validate_colors(&vertices, ordinal)?;
    Ok(vertices.map(|vertex| exact::transform(&vertex, &source.clip_transform)))
}

fn pair_vertices(source: &PrimitiveDraw, pair: &[u32]) -> [RenderVertex; 2] {
    let first = source.vertices[pair[0] as usize];
    let last = source.vertices[pair[1] as usize];
    [first, last]
}
fn clipped_points(
    ends: &[exact::Point; 2],
    parameters: &[exact::Rational; 2],
) -> [exact::Point; 2] {
    parameters
        .each_ref()
        .map(|parameter| exact::interpolate(&ends[0], &ends[1], parameter))
}

fn prepared_draw(
    source: &PrimitiveDraw,
    upload: [[f32; 4]; 2],
    extent: [u32; 2],
    axis: u32,
    first_index: u32,
) -> PrimitiveDraw {
    let color = source.vertices[first_index as usize].color;
    let vertices = upload
        .map(|point| RenderVertex::homogeneous(point, color))
        .to_vec();
    let mut prepared = source.with_replacement_geometry(vertices, vec![0, 1]);
    prepared.clip_transform = crate::identity_clip();
    prepared.projected_dash = Some(PreparedAxis { extent, axis });
    prepared
}
fn source_interval(
    ordinal: usize,
    identity: u64,
    parameters: [exact::Rational; 2],
    error: f64,
) -> SegmentInterval {
    SegmentInterval {
        source_segment: ordinal,
        source_identity: identity,
        exact_parameters: parameters.map(|v| v.to_string()),
        pixel_error_bound: error,
    }
}

fn validate_colors(vertices: &[RenderVertex; 2], ordinal: usize) -> Result<(), ProjectedFailure> {
    if vertices[0].color != vertices[1].color {
        return Err(failed(
            Some(ordinal),
            format!(
                "colors {:?}: expected equal endpoint colors in finite ordinary-segment v2",
                vertices.map(|v| v.color)
            ),
            false,
        ));
    }
    Ok(())
}
fn accepted_points(
    points: &[exact::Point; 2],
    extent: [u32; 2],
    limit: f64,
    ordinal: usize,
    clamp: bool,
) -> Result<([[f32; 4]; 2], u32, f64), ProjectedFailure> {
    validate_weights(points, ordinal)?;
    let axis = projected_axis(points, extent, ordinal)?;
    let upload = exact::upload(points).ok_or_else(|| {
        failed(
            Some(ordinal),
            format!("homogeneous endpoints {points:?}: expected representable positive weights"),
            true,
        )
    })?;
    validate_upload(&upload, clamp, ordinal)?;
    let bound = accepted_error(points, &upload, extent, limit, ordinal)?;
    Ok((upload, axis, bound))
}
fn validate_weights(points: &[exact::Point; 2], ordinal: usize) -> Result<(), ProjectedFailure> {
    if points.iter().all(|p| p[3] > exact::Rational::zero()) {
        return Ok(());
    }
    Err(failed(
        Some(ordinal),
        format!(
            "clipped weights {:?}: expected positive finite projective endpoints",
            points.each_ref().map(|p| &p[3])
        ),
        false,
    ))
}
fn projected_axis(
    points: &[exact::Point; 2],
    extent: [u32; 2],
    ordinal: usize,
) -> Result<u32, ProjectedFailure> {
    exact::axis(&points[0], &points[1], extent).ok_or_else(|| {
        failed(
            Some(ordinal),
            "zero projected direction: expected nondegenerate projected segment".into(),
            false,
        )
    })
}
fn accepted_error(
    points: &[exact::Point; 2],
    upload: &[[f32; 4]; 2],
    extent: [u32; 2],
    limit: f64,
    ordinal: usize,
) -> Result<f64, ProjectedFailure> {
    let error = exact::pixel_error(points, upload, extent);
    let bound = error_bound(&error, ordinal)?;
    if error > exact::Rational::from_float(limit).expect("validated finite limit") {
        return Err(failed(
            Some(ordinal),
            format!("pixel error bound {bound:?}: expected at most {limit:?}"),
            true,
        ));
    }
    Ok(bound)
}

fn error_bound(error: &exact::Rational, ordinal: usize) -> Result<f64, ProjectedFailure> {
    exact::upward(error).ok_or_else(|| {
        failed(
            Some(ordinal),
            "pixel error: expected finite certified upper bound".into(),
            true,
        )
    })
}

fn validate_upload(
    upload: &[[f32; 4]; 2],
    clamp: bool,
    ordinal: usize,
) -> Result<(), ProjectedFailure> {
    if upload
        .iter()
        .any(|p| (0..if clamp { 2 } else { 3 }).any(|axis| p[axis].abs() > p[3]))
    {
        return Err(failed(
            Some(ordinal),
            format!(
                "rounded endpoints {upload:?}: expected points inside enabled homogeneous clip planes"
            ),
            true,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod state_copy_tests;
