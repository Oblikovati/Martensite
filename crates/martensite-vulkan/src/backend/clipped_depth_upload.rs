//! V4 deliberately extends one fixed-lighting/MSAA interface; every other selected state refuses.
use super::{
    runtime::VulkanRuntime, window_depth_parameters::WindowDepthPipelineParameters as Parameters,
};
use crate::{PrimitiveDraw, PrimitiveTopology, RenderError, RenderWindowDepthFailure as Failure};
use ash::vk;
fn refusal(actual: impl std::fmt::Debug) -> RenderError {
    Failure::UnsupportedInterface { actual: format!("V4 draw {actual:?}; expected ordinary fixed Smooth3 clipped triangles, LAST bottom-up D24S8/8x, precomposed projection and no alternate coverage") }.into()
}
pub(super) fn applies(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && draw.fixed_function_lighting
        && draw
            .lighting
            .as_ref()
            .is_some_and(|light| light.fixed.is_some())
        && draw
            .clipping
            .as_ref()
            .is_some_and(|clip| !clip.chains().is_empty())
        && draw.capping_pass == crate::RenderCappingPass::None
        && draw.depth_pass == Default::default()
}
fn supported_policy(runtime: &VulkanRuntime) -> bool {
    runtime.attachment_coordinates == crate::RenderCoordinateConvention::LegacyBottomUpV1
        && runtime.raster_policy.requires_last_vertex()
        && runtime.depth.formats.stencil == vk::Format::D24_UNORM_S8_UINT
        && runtime.depth_bias_behavior == crate::RenderDepthBiasBehavior::Native24UnitsV2
        && runtime.projection_behavior == crate::RenderProjectionBehavior::PrecomposedV1
        && matches!(
            runtime.world_clipping,
            crate::RenderWorldClippingBehavior::FragmentV1
                | crate::RenderWorldClippingBehavior::RasterClipDistanceV2
        )
}
pub(super) fn require_interface(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    samples: vk::SampleCountFlags,
) -> Result<(), RenderError> {
    let admitted = supported_policy(runtime)
        && samples == vk::SampleCountFlags::TYPE_8
        && runtime.sample_rate_shading
        && runtime.fragment_stores;
    if !admitted {
        return Err(refusal((
            samples,
            supported_policy(runtime),
            runtime.sample_rate_shading,
            runtime.fragment_stores,
        )));
    }
    require_draw(draw)?;
    require_constants(runtime, draw)?;
    require_policies(runtime)
}
fn require_policies(runtime: &VulkanRuntime) -> Result<(), RenderError> {
    if runtime.triangle_coverage != crate::RenderTriangleCoverageBehavior::HardwareV1
        || runtime.phong_interpolation != crate::RenderPhongInterpolationBehavior::HardwareV1
        || runtime.world_clip_interpolation
            != crate::RenderWorldClipInterpolationBehavior::HardwareV1
    {
        return Err(refusal("alternate triangle/Phong/world-clip interpolation"));
    }
    Ok(())
}
fn unsupported_surface(draw: &PrimitiveDraw) -> bool {
    draw.user_program.is_some()
        || draw.depth_clamp
        || draw.polygon_points
        || draw.polygon_face_lines
        || draw.polygon_edge_style.is_some()
        || draw.surface_texture.is_some()
        || draw.textured_style.is_some()
        || draw.mesh_edges.is_some()
        || draw.surface_blend
        || draw.hatch_rows.is_some()
        || draw.native_material_cutoff.is_some()
}
fn require_draw(draw: &PrimitiveDraw) -> Result<(), RenderError> {
    let style = draw.lighting.as_ref().and_then(|light| light.fixed);
    if style != Some(crate::fixed_lighting::FixedLightingInterpolation::Smooth)
        || unsupported_surface(draw)
    {
        return Err(refusal((style, "unsupported source interface")));
    }
    Ok(())
}
fn require_constants(runtime: &VulkanRuntime, draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if matches!(
        runtime.window_depth_behavior,
        crate::RenderWindowDepthBehavior::ClippedIncidenceFramebufferV6 { .. }
    ) {
        super::clipped_depth_uniforms::incidence_equations(draw)?;
    } else {
        super::clipped_depth_uniforms::equations(draw)?;
    }
    Ok(())
}
pub(super) fn admit_vertices(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    parameters: Parameters,
) -> Result<(), RenderError> {
    let projected =
        super::window_depth_transforms::projected_bounds(draw, runtime.projection_behavior, true)
            .ok_or_else(|| refusal("unavailable actual-source transform bounds"))?;
    for (vertex, coordinates) in projected.iter().enumerate() {
        require_canonical_bounds(vertex, coordinates.map(|value| value.endpoints()))?;
    }
    let view = parameters.viewport();
    if view.x != 0. || view.y != 0. || view.width <= 0. || view.height <= 0. {
        return Err(refusal(view));
    }
    Ok(())
}
fn alternate_pipeline(kind: super::pipeline_variants::PipelineVariant) -> bool {
    kind.clamp
        || kind.triangle_grid_coverage
        || kind.surface_texture
        || kind.material_maps
        || kind.pbr
        || kind.mesh_edges
        || kind.polygon_edges
        || kind.polygon_points
        || kind.surface_blend
        || kind.line
        || kind.point
        || kind.text
        || kind.factorized_projection
}
pub(super) fn require_pipeline(
    kind: super::pipeline_variants::PipelineVariant,
    oit: bool,
    samples: vk::SampleCountFlags,
) -> Result<(), RenderError> {
    let fixed = kind.fixed_lighting && kind.lighting && kind.clipped && kind.world_clip_v2;
    if oit || samples != vk::SampleCountFlags::TYPE_8 || !fixed || alternate_pipeline(kind) {
        return Err(refusal((kind, oit, samples)));
    }
    Ok(())
}

fn require_canonical_bounds(vertex: usize, coordinates: [[f64; 2]; 4]) -> Result<(), RenderError> {
    let [minimum, maximum] = coordinates[3];
    let interior = coordinates[..3]
        .iter()
        .all(|range| range[0] >= -minimum && range[1] <= minimum);
    if minimum >= f64::from(f32::MIN_POSITIVE) && maximum.is_finite() && interior {
        return Ok(());
    }
    Err(refusal((
        vertex,
        coordinates,
        "expected certified canonical interior and positive normal W",
    )))
}
#[cfg(test)]
#[path = "clipped_depth_upload_tests.rs"]
mod tests;
