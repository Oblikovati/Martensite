//! V4 refuses unit bias when its declared policy cannot resolve a window-depth scale.
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderDepthBiasBehavior, RenderDepthBiasFailure, RenderError,
    RenderFrame, RenderTriangleCoverageBehavior,
};

// Clip admission bounds Q12 coordinates below 2^30 ticks and depth to [0,1]; area is >=1 tick².
// This bounds shader slope below 2^44 per pixel; the 1e25 factor product stays below f32::MAX.
// Native24 units times 2^-23 stay below 2^106; V4 clamps the finite final depth to its range.
const MAX_V4_SLOPE_FACTOR: f32 = 1.0e25;

pub(super) fn validate_frame(
    coverage: RenderTriangleCoverageBehavior,
    bias: RenderDepthBiasBehavior,
    frame: &RenderFrame,
) -> Result<(), RenderError> {
    if !is_v4(coverage) {
        return Ok(());
    }
    validate_layers(bias, frame)
}

fn is_v4(coverage: RenderTriangleCoverageBehavior) -> bool {
    matches!(
        coverage,
        RenderTriangleCoverageBehavior::NativeGrid12V4 { .. }
    )
}

fn validate_layers(bias: RenderDepthBiasBehavior, frame: &RenderFrame) -> Result<(), RenderError> {
    for layer in frame.layers() {
        validate_layer(bias, frame, layer)?;
    }
    Ok(())
}

fn validate_layer(
    bias: RenderDepthBiasBehavior,
    frame: &RenderFrame,
    layer: &crate::RenderLayerSpan,
) -> Result<(), RenderError> {
    let mut offsets = crate::polygon_offset::PolygonOffsetState::new(layer.polygon_offset);
    for draw in &frame.draws()[layer.draw_range.clone()] {
        validate_draw(bias, draw, layer, &mut offsets)?;
    }
    Ok(())
}

fn validate_draw(
    bias: RenderDepthBiasBehavior,
    draw: &PrimitiveDraw,
    layer: &crate::RenderLayerSpan,
    offsets: &mut crate::polygon_offset::PolygonOffsetState,
) -> Result<(), RenderError> {
    let (units, factor, active) = effective_bias(draw, layer, offsets);
    if !selected_phong_triangle(draw) {
        return Ok(());
    }
    refuse_v4_bias(bias, units, factor, active)
}

fn selected_phong_triangle(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && draw.lighting().is_some_and(|lighting| {
            !lighting.is_pbr() && lighting.shading() == crate::RenderShading::Phong
        })
}

fn effective_bias(
    draw: &PrimitiveDraw,
    layer: &crate::RenderLayerSpan,
    offsets: &mut crate::polygon_offset::PolygonOffsetState,
) -> (f32, f32, bool) {
    let presentation = super::frame_draw_presentation::from_semantic_draw(draw);
    let ordinary = if presentation.polygon_edge {
        offsets.advance_polygon_line(presentation.polygon_offset)
    } else {
        offsets.advance(presentation.polygon_offset, presentation.polygon)
    };
    if presentation.text_shadow_bias && shadow_depth_test(layer, &presentation) {
        let [units, factor] = offsets.shadow_bias();
        return (units, factor, true);
    }
    let active = offsets.raster_bias_enabled(presentation.polygon, presentation.polygon_edge);
    (ordinary[0], ordinary[1], active)
}

fn shadow_depth_test(
    layer: &crate::RenderLayerSpan,
    presentation: &super::frame::DrawPresentation,
) -> bool {
    layer.depth_test
        || (layer.depth_write
            && !presentation.annotation
            && !presentation.stencil_rect
            && presentation.capping == crate::RenderCappingPass::None)
}

fn refuse_v4_bias(
    bias: RenderDepthBiasBehavior,
    units: f32,
    factor: f32,
    active: bool,
) -> Result<(), RenderError> {
    if !active {
        return Ok(());
    }
    refuse_slope_factor(factor)?;
    refuse_unresolved_units(bias, units)
}

fn refuse_unresolved_units(bias: RenderDepthBiasBehavior, units: f32) -> Result<(), RenderError> {
    if units == 0.0 || bias != RenderDepthBiasBehavior::HardwareFormatV1 {
        return Ok(());
    }
    Err(RenderDepthBiasFailure::UnresolvedHardwareUnits {
        units_bits: units.to_bits(),
    }
    .into())
}

fn refuse_slope_factor(factor: f32) -> Result<(), RenderError> {
    if factor.abs() <= MAX_V4_SLOPE_FACTOR {
        return Ok(());
    }
    Err(RenderDepthBiasFailure::UnboundedSlopeFactor {
        factor_bits: factor.to_bits(),
    }
    .into())
}

#[cfg(test)]
#[path = "v4_depth_admission_tests.rs"]
mod tests;
