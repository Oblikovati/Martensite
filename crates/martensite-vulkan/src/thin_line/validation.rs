//! Fail closed before replacing a submitted thin-line draw.
use crate::error::rejected;
use crate::{PrimitiveDraw, RenderError, RenderThinAxisCoverage, RenderThinLineCoverage};
use ash::vk;

pub(super) fn validate_selection(
    behavior: RenderThinLineCoverage,
    axes: RenderThinAxisCoverage,
    draw: &PrimitiveDraw,
    extent: vk::Extent2D,
    single_sample: bool,
) -> Result<(), RenderError> {
    if axes != RenderThinAxisCoverage::HardwareV1 {
        return Err(rejected(
            (axes, draw),
            "diamond-exit line behavior without simultaneous FixedGridV2 thin-axis coverage",
        ));
    }
    if !single_sample
        || extent.width == 0
        || extent.height == 0
        || extent.width > 32768
        || extent.height > 32768
    {
        return Err(rejected(
            (single_sample, extent),
            "single-sample viewport dimensions in 1..=32768",
        ));
    }
    validate_draw(behavior, draw)
}

fn validate_draw(
    behavior: RenderThinLineCoverage,
    draw: &PrimitiveDraw,
) -> Result<(), RenderError> {
    let style = draw
        .line_style
        .ok_or_else(|| rejected(draw, "explicit one-pixel line style"))?;
    if style.raster_width() != 1. || style.alpha_blend {
        return Err(rejected(
            (style, draw.source_line_strip),
            "opaque one-pixel line segments",
        ));
    }
    validate_strip(behavior, draw, style.pattern)?;
    validate_features(draw)
}

fn validate_strip(
    behavior: RenderThinLineCoverage,
    draw: &PrimitiveDraw,
    pattern: u16,
) -> Result<(), RenderError> {
    if !draw.source_line_strip {
        return Ok(());
    }
    let pairs = draw.indices.as_chunks::<2>();
    if behavior != RenderThinLineCoverage::DiamondExitFusedProjectionV4
        || pattern != 0xffff
        || !pairs.1.is_empty()
        || pairs.0.windows(2).any(|pair| pair[0][1] != pair[1][0])
    {
        return Err(rejected(
            (behavior, pattern, &draw.indices),
            "V4 solid strip with ordered connected index pairs",
        ));
    }
    // Solid strips have no cross-segment counter; preserve the ordered half-open diamonds.
    Ok(())
}

fn validate_features(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if draw.user_program.is_some()
        || draw.lighting.is_some()
        || draw.fixed_function_lighting
        || draw.native_material_cutoff.is_some()
        || !draw.native_flat_sources.is_empty()
        || draw.mesh_edges.is_some()
        || draw.hatch_rows.is_some()
        || draw.face_culling != crate::RenderFaceCulling::None
        || draw.polygon_face_lines
        || draw.polygon_points
        || draw.fixed_grid_point
        || draw.fixed_grid_point_unit12
        || draw.polygon_edge_style.is_some()
        || draw.projected_dash.is_some()
        || draw.annotation
        || draw.stencil_rect
        || draw.text_shadow_bias
        || draw.text_logic_xor
        || draw.text_background_blend
        || draw.surface_texture_binding_only
        || draw.point_style.is_some()
        || draw.unlit_back_color.is_some()
        || draw.capping_pass != crate::RenderCappingPass::None
    {
        return Err(rejected(
            draw,
            "built-in unlit lines without shading, polygon or texture features",
        ));
    }
    if draw.surface_blend
        || draw.surface_forced_blend
        || draw.surface_cutoff.is_some()
        || draw.textured_style.is_some()
        || draw.surface_texture.is_some()
        || draw.surface_texture_unavailable.is_some()
    {
        return Err(rejected(
            draw,
            "opaque line colors without surface texture or alpha features",
        ));
    }
    Ok(())
}
