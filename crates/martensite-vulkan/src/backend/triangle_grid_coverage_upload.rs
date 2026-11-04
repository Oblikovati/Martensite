use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError, RenderPhongInterpolationBehavior, RenderShading,
    RenderTriangleCoverageBehavior as Behavior, RenderTriangleCoverageFailure as Failure,
};
use ash::vk;

#[path = "triangle_grid_empty_pruning.rs"]
mod empty_pruning;

#[cfg(test)]
pub(super) fn admit(
    behavior: Behavior,
    interpolation: RenderPhongInterpolationBehavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    samples: vk::SampleCountFlags,
    subpixel_bits: u32,
) -> Result<(), RenderError> {
    prepare(
        behavior,
        interpolation,
        draw,
        viewport,
        samples,
        subpixel_bits,
        false,
        false,
    )
    .map(|_| ())
}

#[allow(clippy::too_many_arguments)] // Each argument is a distinct Vulkan or behaviour input.
pub(super) fn prepare(
    behavior: Behavior,
    interpolation: RenderPhongInterpolationBehavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    samples: vk::SampleCountFlags,
    subpixel_bits: u32,
    allow_world_clipping: bool,
    allow_unclipped_v3: bool,
) -> Result<Option<PrimitiveDraw>, RenderError> {
    if behavior == Behavior::HardwareV1 || !selected_phong(draw) {
        return Ok(None);
    }
    require_interface(
        behavior,
        interpolation,
        draw,
        viewport,
        samples,
        allow_world_clipping,
        allow_unclipped_v3,
    )?;
    if matches!(behavior, Behavior::FixedGrid12V3 { .. }) {
        return empty_pruning::private_geometry(draw, viewport, subpixel_bits);
    }
    if matches!(behavior, Behavior::NativeGrid12V4 { .. }) {
        return empty_pruning::private_native_grid_v4_geometry(draw, viewport);
    }
    let rule = coverage_grid_rule(subpixel_bits);
    admit_triangles(draw, viewport, rule)?;
    Ok(None)
}

pub(super) fn selected_phong(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && draw.lighting.as_ref().is_some_and(|lighting| {
            !lighting.is_pbr() && lighting.shading() == RenderShading::Phong
        })
        // A draw this rule cannot certify stays on the device rasterizer, so one such draw does not
        // leave the whole frame unrendered. User clipping keeps its own admission in `prepare`.
        && unsupported_draw_interface(draw, true).is_none()
}

fn require_interface(
    behavior: Behavior,
    interpolation: RenderPhongInterpolationBehavior,
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    samples: vk::SampleCountFlags,
    allow_world_clipping: bool,
    allow_unclipped_v3: bool,
) -> Result<(), RenderError> {
    let empty_clip_state =
        allow_unclipped_v3 && !super::world_clip_interpolation_upload::has_world_planes(draw);
    let supported_v3 = matches!(
        behavior,
        Behavior::FixedGrid12V3 { .. } | Behavior::NativeGrid12V4 { .. }
    );
    let supported_v3 = supported_v3
        && interpolation == RenderPhongInterpolationBehavior::UnsnappedV3
        && (allow_world_clipping || empty_clip_state);
    let supported_interpolation =
        interpolation == RenderPhongInterpolationBehavior::UnsnappedV2 || supported_v3;
    if !supported_interpolation {
        return unsupported(format!(
            "Phong interpolation {interpolation:?}, world clip admitted={allow_world_clipping}, no-plane V3 composition={empty_clip_state}, active world planes={}",
            super::world_clip_interpolation_upload::has_world_planes(draw)
        ));
    }
    if samples != vk::SampleCountFlags::TYPE_1 {
        return unsupported(format!("rasterization samples {samples:?}"));
    }
    require_supported_domain(draw, viewport, allow_world_clipping || empty_clip_state)
}

fn require_supported_domain(
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    allow_world_clipping: bool,
) -> Result<(), RenderError> {
    if !viewport_in_int64_domain(viewport) {
        return unsupported(format!(
            "viewport {viewport:?} exceeds signed edge arithmetic domain"
        ));
    }
    if let Some(actual) = unsupported_draw_interface(draw, allow_world_clipping) {
        return unsupported_draw(actual);
    }
    Ok(())
}

fn unsupported_draw(actual: &str) -> Result<(), RenderError> {
    unsupported(format!("{actual}=true"))
}

fn viewport_in_int64_domain(viewport: [f32; 4]) -> bool {
    let [x, y, width, height] = viewport;
    let corners = [x, y, x + width, y + height];
    corners.into_iter().all(|value| {
        value.is_finite()
            && value.abs() <= super::super::triangle_grid_coverage::MAX_SAFE_VIEWPORT_PIXELS
    })
}

fn unsupported_draw_interface(
    draw: &PrimitiveDraw,
    allow_world_clipping: bool,
) -> Option<&'static str> {
    unsupported_coverage_overlays(draw)
        .or_else(|| unsupported_lighting_states(draw, allow_world_clipping))
}

fn unsupported_coverage_overlays(draw: &PrimitiveDraw) -> Option<&'static str> {
    [
        ("mesh edges", draw.mesh_edges.is_some()),
        ("polygon points", draw.polygon_points),
        ("polygon edges", draw.polygon_edge_style.is_some()),
        ("polygon face lines", draw.polygon_face_lines),
        ("point style", draw.point_style.is_some()),
        ("line style", draw.line_style.is_some()),
        (
            "surface texture fallback",
            draw.surface_texture_unavailable.is_some(),
        ),
        ("surface binding only", draw.surface_texture_binding_only),
        ("compatibility target", draw.compatibility_color_target),
        ("forced surface blending", draw.surface_forced_blend),
        ("unlit back color", draw.unlit_back_color.is_some()),
        ("textured style", draw.textured_style.is_some()),
        (
            "capping pass",
            draw.capping_pass != crate::RenderCappingPass::None,
        ),
    ]
    .into_iter()
    .find_map(|(name, present)| present.then_some(name))
}

fn unsupported_lighting_states(
    draw: &PrimitiveDraw,
    allow_world_clipping: bool,
) -> Option<&'static str> {
    [
        ("fixed lighting", draw.fixed_function_lighting),
        ("custom program", draw.user_program.is_some()),
        (
            "user clipping",
            draw.clipping.is_some() && !allow_world_clipping,
        ),
        ("surface texture", draw.surface_texture.is_some()),
        ("surface blending", draw.surface_blend),
        ("surface cutoff", draw.surface_cutoff.is_some()),
        ("shadow casting", shadow_casting(draw)),
        (
            "nondefault depth pass",
            draw.depth_pass != Default::default(),
        ),
    ]
    .into_iter()
    .find_map(|(name, present)| present.then_some(name))
}

fn shadow_casting(draw: &PrimitiveDraw) -> bool {
    draw.lighting.as_ref().is_some_and(|lighting| {
        lighting
            .lights
            .sources
            .iter()
            .any(|source| source.cast_shadows)
    })
}

#[derive(Clone, Copy)]
struct CoverageGridRule {
    grid_bits: u32,
    device_bits: u32,
}

fn coverage_grid_rule(device_bits: u32) -> CoverageGridRule {
    CoverageGridRule {
        grid_bits: device_bits,
        device_bits,
    }
}

fn admit_triangles(
    draw: &PrimitiveDraw,
    viewport: [f32; 4],
    rule: CoverageGridRule,
) -> Result<(), RenderError> {
    let clips = projected_clips(draw);
    draw.indices
        .as_chunks::<3>()
        .0
        .iter()
        .enumerate()
        .try_for_each(|(ordinal, indices)| {
            admit_draw_triangle(draw, indices, &clips, viewport, rule, ordinal)
        })
}

fn admit_draw_triangle(
    draw: &PrimitiveDraw,
    indices: &[u32],
    clips: &[[f32; 4]],
    viewport: [f32; 4],
    rule: CoverageGridRule,
    ordinal: usize,
) -> Result<(), RenderError> {
    let (vertices, errors) = triangle_inputs(draw, indices, clips);
    admit_triangle_on_grid(vertices, errors, viewport, rule, ordinal)
}

fn triangle_inputs(
    draw: &PrimitiveDraw,
    indices: &[u32],
    projected: &[[f32; 4]],
) -> ([[f32; 4]; 3], [Option<[f64; 4]>; 3]) {
    let clips = std::array::from_fn(|i| projected[indices[i] as usize]);
    let errors = std::array::from_fn(|i| {
        clip_rounding_bounds(draw.clip_transform, draw.vertices[indices[i] as usize])
    });
    (clips, errors)
}

fn projected_clips(draw: &PrimitiveDraw) -> Vec<[f32; 4]> {
    draw.vertices
        .iter()
        .map(|vertex| {
            super::phong_interpolation_upload::clip_position(draw.clip_transform, *vertex)
        })
        .collect()
}

fn admit_triangle(
    clips: [[f32; 4]; 3],
    clip_errors: [Option<[f64; 4]>; 3],
    viewport: [f32; 4],
    subpixel_bits: u32,
    triangle: usize,
) -> Result<(), RenderError> {
    let rule = CoverageGridRule {
        grid_bits: subpixel_bits,
        device_bits: subpixel_bits,
    };
    admit_triangle_on_grid(clips, clip_errors, viewport, rule, triangle)
}

fn admit_triangle_on_grid(
    clips: [[f32; 4]; 3],
    clip_errors: [Option<[f64; 4]>; 3],
    viewport: [f32; 4],
    rule: CoverageGridRule,
    triangle: usize,
) -> Result<(), RenderError> {
    let (grid, error) = checked_triangle_grid(clips, clip_errors, viewport, triangle)?;
    require_surviving_raster_grid(grid, clips, triangle, rule, error)
}

fn checked_triangle_grid(
    clips: [[f32; 4]; 3],
    errors: [Option<[f64; 4]>; 3],
    viewport: [f32; 4],
    triangle: usize,
) -> Result<(super::super::triangle_grid_coverage::GridTriangle, f64), RenderError> {
    require_inside_clip_volume(clips, errors, triangle)?;
    let grid = grid_triangle(clips, viewport).ok_or_else(|| invalid_triangle(triangle, clips))?;
    let error = max_projection_error(clips, errors, viewport)
        .ok_or_else(|| invalid_triangle(triangle, clips))?;
    Ok((grid, error))
}

fn require_inside_clip_volume(
    clips: [[f32; 4]; 3],
    errors: [Option<[f64; 4]>; 3],
    triangle: usize,
) -> Result<(), RenderError> {
    if clips_inside_clip_volume(clips, errors) {
        return Ok(());
    }
    Err(invalid_triangle(triangle, clips))
}

fn require_surviving_raster_grid(
    triangle_grid: super::super::triangle_grid_coverage::GridTriangle,
    clips: [[f32; 4]; 3],
    triangle: usize,
    rule: CoverageGridRule,
    projection_error: f64,
) -> Result<(), RenderError> {
    if triangle_grid.signed_area() == 0
        || !triangle_grid.survives_grid(rule.grid_bits, projection_error)
    {
        return Err(uncertain_grid_failure(rule, triangle, clips));
    }
    Ok(())
}

fn uncertain_grid_failure(
    rule: CoverageGridRule,
    triangle: usize,
    clips: [[f32; 4]; 3],
) -> RenderError {
    let clip_bits = clips.map(|clip| clip.map(f32::to_bits));
    Failure::RasterGridDegenerate {
        triangle,
        subpixel_precision_bits: rule.device_bits,
        clip_bits,
    }
    .into()
}

fn clips_inside_clip_volume(clips: [[f32; 4]; 3], errors: [Option<[f64; 4]>; 3]) -> bool {
    clips.into_iter().zip(errors).all(|(clip, error)| {
        let Some(error) = error else { return false };
        let w = f64::from(clip[3]) - error[3];
        clip.into_iter().all(f32::is_finite)
            && w > 0.0
            && [0, 1, 2].into_iter().all(|axis| {
                clip[axis].is_finite() && (f64::from(clip[axis]).abs() + error[axis]) <= w
            })
    })
}

pub(super) fn clip_rounding_bounds(
    matrix: [f32; 16],
    vertex: crate::RenderVertex,
) -> Option<[f64; 4]> {
    let clip = super::phong_interpolation_upload::clip_position(matrix, vertex);
    let operation_bounds = crate::window_depth_bounds::clip_transform_bounds(matrix, vertex)?;
    let bounds = std::array::from_fn(|row| {
        let [minimum, maximum] = operation_bounds[row].endpoints();
        (f64::from(clip[row]) - minimum)
            .abs()
            .max((maximum - f64::from(clip[row])).abs())
    });
    bounds.into_iter().all(f64::is_finite).then_some(bounds)
}

fn max_projection_error(
    clips: [[f32; 4]; 3],
    clip_errors: [Option<[f64; 4]>; 3],
    viewport: [f32; 4],
) -> Option<f64> {
    let mut maximum = 0.0_f64;
    for (clip, errors) in clips.into_iter().zip(clip_errors) {
        let errors = errors?;
        for axis in 0..2 {
            maximum = maximum.max(window_error(clip, errors, viewport, axis)?);
        }
    }
    Some(maximum)
}

fn window_error(clip: [f32; 4], errors: [f64; 4], viewport: [f32; 4], axis: usize) -> Option<f64> {
    // Three epsilons cover highp division; eight half-ULPs bound the viewport arithmetic.
    let w = f64::from(clip[3]);
    let division_min = f64::from(f32::MIN_POSITIVE);
    let division_max = martensite_cmath::powi(2.0_f64, 126);
    if !clip.into_iter().all(f32::is_finite)
        || w <= errors[3]
        || w < division_min
        || w > division_max
    {
        return None;
    }
    Some(window_error_from_valid(clip, errors, viewport, axis, w))
}

fn window_error_from_valid(
    clip: [f32; 4],
    errors: [f64; 4],
    viewport: [f32; 4],
    axis: usize,
    w: f64,
) -> f64 {
    let coordinate = f64::from(clip[axis]);
    let ratio_error = (w * errors[axis] + coordinate.abs() * errors[3]) / (w * (w - errors[3]));
    let division_error = 3.0 * f64::from(f32::EPSILON) + f64::from(f32::MIN_POSITIVE) / w;
    let origin = f64::from(viewport[axis]);
    let extent = f64::from(viewport[axis + 2]);
    let roundoff = 8.0 * (f64::from(f32::EPSILON) / 2.0) * (origin.abs() + extent.abs() + 1.0);

    (ratio_error + division_error) * extent.abs() / 2.0 + roundoff
}

fn grid_triangle(
    clips: [[f32; 4]; 3],
    viewport: [f32; 4],
) -> Option<super::super::triangle_grid_coverage::GridTriangle> {
    let vertices = clips.map(|clip| {
        let point = super::phong_interpolation_math::window_point(clip, viewport)?;
        super::super::triangle_grid_coverage::GridVertex::snap(point.map(f64::from))
    });
    Some(super::super::triangle_grid_coverage::GridTriangle::new([
        vertices[0]?,
        vertices[1]?,
        vertices[2]?,
    ]))
}

fn invalid_triangle(triangle: usize, clips: [[f32; 4]; 3]) -> RenderError {
    Failure::InvalidTriangle {
        triangle,
        clip_bits: clips.map(|clip| clip.map(f32::to_bits)),
    }
    .into()
}

fn unsupported(actual: String) -> Result<(), RenderError> {
    Err(Failure::UnsupportedInterface { actual }.into())
}

#[cfg(test)]
#[path = "triangle_grid_coverage_upload_tests.rs"]
pub(super) mod tests;
