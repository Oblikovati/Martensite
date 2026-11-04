//! A unique ordinary draw makes the prepared field exactly the recorder's bound pipeline.
use super::{
    super::{frame::FrameResources, pipeline::DrawPipeline},
    records::{ExecutableGap, contract},
};
use crate::{RenderCappingPass, RenderFrame};
use ash::vk;

/// Borrow the unique ordinary bound field; `selected(frame, resources)?`.
pub(super) fn selected<'a>(
    frame: &RenderFrame,
    resources: &'a FrameResources,
) -> Result<&'a DrawPipeline, ExecutableGap> {
    require_unique_draw(frame, resources)?;
    require_attachments(resources)?;
    let draw = &resources.draws[0];
    require_ordinary(draw)?;
    Ok(&draw.pipeline)
}
fn require_unique_draw(
    frame: &RenderFrame,
    resources: &FrameResources,
) -> Result<(), ExecutableGap> {
    let counts = (frame.draws.len(), frame.layers.len(), resources.draws.len());
    if counts != (1, 1, 1) || frame.layers[0].draw_range != (0..1) {
        return Err(contract(
            "recorded draw domain",
            counts,
            "exactly one layer and one actual uploaded draw",
        ));
    }
    Ok(())
}
fn require_attachments(resources: &FrameResources) -> Result<(), ExecutableGap> {
    if resources.oit.is_some()
        || resources.multisample.is_some()
        || resources.color.format != vk::Format::R8G8B8A8_SRGB
    {
        return Err(contract(
            "attachments",
            resources.color.format,
            "ordinary single-sample SRGB without OIT",
        ));
    }
    Ok(())
}
fn require_ordinary(draw: &super::super::frame::UploadedDraw) -> Result<(), ExecutableGap> {
    let specialized =
        geometry_features(draw.pipeline.kind) || fragment_features(draw.pipeline.kind);
    let extra =
        specialized || bound_extra(draw) || draw.native_alpha.is_some() || draw.oit.is_some();
    if extra || draw.pipeline.kind.capping != RenderCappingPass::None {
        return Err(contract(
            "pipeline domain",
            (specialized, extra),
            "unique ordinary unlit triangle draw without specialized descriptors",
        ));
    }
    Ok(())
}
fn geometry_features(kind: super::super::pipeline_variants::PipelineVariant) -> bool {
    [
        kind.fixed_line_stipple,
        kind.fixed_line_strip,
        kind.mesh_edges,
        kind.polygon_edges,
        kind.line,
        kind.point,
        kind.text,
        kind.stencil,
    ]
    .into_iter()
    .any(|value| value)
}
fn fragment_features(kind: super::super::pipeline_variants::PipelineVariant) -> bool {
    [
        kind.lighting,
        kind.fixed_lighting,
        kind.surface_texture,
        kind.surface_alpha,
        kind.encoded_surface,
        kind.clipped,
        kind.surface_blend,
    ]
    .into_iter()
    .any(|value| value)
}
fn bound_extra(draw: &super::super::frame::UploadedDraw) -> bool {
    draw.bindings.clipping.is_some()
        || draw.bindings.lighting.is_some()
        || draw.bindings.sprite.is_some()
        || draw.bindings.surface_image.is_some()
}
