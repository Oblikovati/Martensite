//! One closed triangle operation keeps the model-position rounding boundary explicit.
use super::pipeline_variants::PipelineVariant;
use crate::{PrimitiveDraw, RenderError, RenderProjectionBehavior as Behavior, error::rejected};
use ash::vk;

pub(super) fn enabled(behavior: Behavior, draw: &PrimitiveDraw) -> bool {
    // A draw this route cannot serve keeps its precomposed transform, so one such draw does not
    // leave the whole frame unrendered.
    behavior.factorizes_triangles()
        && draw.topology() == crate::PrimitiveTopology::Triangles
        && eligible(draw)
}
fn eligible(draw: &PrimitiveDraw) -> bool {
    !(draw.projection_factors.is_none()
        || draw.user_program.is_some()
        || draw.clipping.is_some()
        || draw.textured_style.is_some()
        || draw.surface_texture.is_some()
        || draw.surface_texture_unavailable.is_some()
        || draw.surface_cutoff.is_some()
        || draw.unlit_back_color.is_some()
        || draw.fixed_function_lighting
        || draw.mesh_edges.is_some()
        || draw.polygon_edge_style.is_some()
        || draw.polygon_points
        || !kind_supported(super::pipeline_variants::special_variant(draw, false)))
}
pub(super) fn admit(behavior: Behavior, draw: &PrimitiveDraw) -> Result<(), RenderError> {
    if !enabled(behavior, draw) {
        return Ok(());
    }
    if let Some(factors) = &draw.projection_factors {
        let camera = factors.camera_product();
        if !camera.iter().all(|value| value.is_finite()) {
            return Err(rejected(
                camera,
                "finite binary32 factorized camera product",
            ));
        }
    }
    Ok(())
}
pub(super) fn admit_pipeline(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    actual: PipelineVariant,
) -> Result<(), RenderError> {
    let selected = enabled(behavior, draw);
    if actual.factorized_projection != selected || (selected && !kind_supported(actual)) {
        return Err(rejected(
            (actual.stencil, actual.capping),
            "actual factorized triangle pipeline without stencil or alternate shader routes",
        ));
    }
    Ok(())
}
pub(super) fn admit_vertex_composition(kind: PipelineVariant) -> Result<(), RenderError> {
    if kind.window_depth.is_some_and(|p| p.clipped_fan()) && !kind.factorized_projection {
        return super::clipped_depth_upload::require_pipeline(
            kind,
            false,
            vk::SampleCountFlags::TYPE_8,
        );
    }
    if kind_supported(kind) {
        return Ok(());
    }
    if kind.window_depth.is_some() {
        return Err(crate::RenderWindowDepthFailure::UnsupportedInterface {
            actual: format!("{kind:?}"),
        }
        .into());
    }
    if kind.factorized_projection {
        return Err(rejected(
            kind,
            "factorized vertex without alternate pipeline routes",
        ));
    }
    Ok(())
}
pub(super) fn kind_supported(kind: PipelineVariant) -> bool {
    !kind.line
        && !kind.point
        && !kind.text
        && !kind.clipped
        && !kind.fixed_lighting
        && !kind.pbr
        && !kind.material_maps
        && !kind.sphere_map
        && !kind.surface_texture
        && !kind.surface_alpha
        && !kind.mesh_edges
        && !kind.polygon_edges
        && !kind.polygon_points
        && !kind.world_clip_v2
        && !kind.world_clip_v3_line
        && !kind.world_clip_grid12
        && kind.capping == crate::RenderCappingPass::None
}
pub(super) fn admit_composition(
    behavior: Behavior,
    draw: &PrimitiveDraw,
    world: crate::RenderWorldClippingBehavior,
    interpolation: crate::RenderPhongInterpolationBehavior,
    coverage: crate::RenderTriangleCoverageBehavior,
) -> Result<(), RenderError> {
    if enabled(behavior, draw)
        && (!matches!(
            world,
            crate::RenderWorldClippingBehavior::FragmentV1
                // Selects clip distances only for fixed-function frames, never factorized draws.
                | crate::RenderWorldClippingBehavior::RasterFixedFunctionV4
                | crate::RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5
        ) || interpolation != crate::RenderPhongInterpolationBehavior::HardwareV1
            || coverage != crate::RenderTriangleCoverageBehavior::HardwareV1)
    {
        return Err(rejected(
            (world, interpolation, coverage),
            "factorized triangles with fragment clipping and hardware interpolation/coverage",
        ));
    }
    Ok(())
}
pub(super) fn push_range(
    behavior: Behavior,
    kind: PipelineVariant,
    mut range: vk::PushConstantRange,
) -> vk::PushConstantRange {
    if behavior.factorizes_triangles() && kind.factorized_projection && kind_supported(kind) {
        range.size = 128;
    }
    range
}
pub(super) fn transform(behavior: Behavior, draw: &PrimitiveDraw, ordinary: Vec<u8>) -> Vec<u8> {
    if !enabled(behavior, draw) {
        return ordinary;
    }
    let factors = draw
        .projection_factors
        .as_ref()
        .expect("admitted factorized source matrices");
    factors
        .camera_product()
        .into_iter()
        .chain(factors.model())
        .flat_map(f32::to_ne_bytes)
        .collect()
}
pub(super) fn vertex(
    behavior: Behavior,
    kind: PipelineVariant,
    stock: &'static [u8],
) -> Result<&'static [u8], RenderError> {
    if !behavior.factorizes_triangles() || !kind.factorized_projection || !kind_supported(kind) {
        return Ok(stock);
    }
    for (original, factorized) in VERTICES {
        if stock == *original {
            return Ok(factorized);
        }
    }
    Err(rejected(
        stock.len(),
        "declared factorized triangle vertex module",
    ))
}
const VERTICES: &[(&[u8], &[u8])] = &[
    (
        include_bytes!("../../shaders/unlit.vert.spv"),
        include_bytes!("../../shaders/factorized-v2.unlit.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/lighting.vert.spv"),
        include_bytes!("../../shaders/factorized-v2.lighting.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/numeric-max-v2.lighting.vert.spv"),
        include_bytes!("../../shaders/factorized-v2.numeric-max-v2.lighting.vert.spv"),
    ),
];
#[cfg(test)]
#[path = "factorized_projection_tests.rs"]
mod tests;
