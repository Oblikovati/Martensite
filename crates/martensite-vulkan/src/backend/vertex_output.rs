//! Opt-in actual-draw TF owns its buffers until the normal submission fence completes.
mod admission;
mod barrier;
pub(super) use barrier::host_barrier;
mod invocation;
mod resources;
mod shader;
use super::{frame::FrameResources, runtime::VulkanRuntime};
use crate::{RenderError, VertexOutputFailure};
pub(super) use admission::admit;
use ash::vk;
pub(super) use resources::VertexOutputResources;

pub(super) fn unsupported(operation: &'static str, actual: impl std::fmt::Debug) -> RenderError {
    VertexOutputFailure::Unsupported {
        operation,
        actual: format!("{actual:?}"),
    }
    .into()
}
pub(super) fn completed(resources: &FrameResources) -> Result<(), RenderError> {
    for (ordinal, draw) in resources.draws.iter().enumerate() {
        if let Some(capture) = &draw.vertex_output {
            capture.completed(ordinal, draw)?;
        }
    }
    Ok(())
}
pub(super) fn reset(runtime: &VulkanRuntime, command: vk::CommandBuffer, frame: &FrameResources) {
    for draw in &frame.draws {
        if let Some(capture) = &draw.vertex_output {
            capture.reset(runtime, command);
        }
    }
}
pub(super) use shader::{fragment_for_format, selected_for_format};
#[cfg(test)]
use shader::{selected as shader_bytes, selected_fragment as fragment_bytes};
pub(super) fn validate_frame(
    runtime: &VulkanRuntime,
    frame: &crate::RenderFrame,
) -> Result<(), RenderError> {
    validate_request_frame(runtime.vertex_output.as_deref(), frame)
}
fn validate_request_frame(
    request: Option<&crate::VertexOutputRequest>,
    frame: &crate::RenderFrame,
) -> Result<(), RenderError> {
    let Some(request) = request else {
        return Ok(());
    };
    let Some(draw) = selected_request_draw(request, frame)? else {
        return Ok(());
    };
    validate_draw(request, draw)
}
fn selected_request_draw<'a>(
    request: &crate::VertexOutputRequest,
    frame: &'a crate::RenderFrame,
) -> Result<Option<&'a crate::PrimitiveDraw>, RenderError> {
    let token = request
        .source_token
        .ok_or_else(|| unsupported("explicit source token", "missing"))?;
    let selected = frame
        .draws
        .iter()
        .filter(|draw| draw.vertex_output_target == Some(token))
        .collect::<Vec<_>>();
    unique_request_draw(&selected, token)
}
fn unique_request_draw<'a>(
    selected: &[&'a crate::PrimitiveDraw],
    token: std::num::NonZeroU64,
) -> Result<Option<&'a crate::PrimitiveDraw>, RenderError> {
    match selected.len() {
        0 => Ok(None),
        1 => Ok(Some(selected[0])),
        matches => Err(VertexOutputFailure::AmbiguousSource {
            source: format!("source_token={token}"),
            matches,
        }
        .into()),
    }
}
fn validate_draw(
    request: &crate::VertexOutputRequest,
    draw: &crate::PrimitiveDraw,
) -> Result<(), RenderError> {
    if draw.indices == request.indices
        && match request.format() {
            crate::VertexOutputFormat::TrianglesClipNormalId32 => ordinary_phong_draw(draw),
            crate::VertexOutputFormat::PointsClipId20 => ordinary_plain_point(draw),
            crate::VertexOutputFormat::TextClipUvId28 => ordinary_text(draw),
        }
    {
        return Ok(());
    }
    Err(unsupported(
        "unchanged original indexed Phong VS+FS draw",
        draw.topology(),
    ))
}
fn ordinary_text(draw: &crate::PrimitiveDraw) -> bool {
    draw.topology() == crate::PrimitiveTopology::Triangles
        && draw.textured_style.is_some()
        && draw.surface_texture.is_none()
        && draw.surface_texture_unavailable.is_none()
        && draw.user_program.is_none()
        && draw.lighting.is_none()
        && !draw.polygon_points
        && !draw.polygon_face_lines
        && !draw.fixed_function_lighting
        && draw.clipping.is_none()
}
fn ordinary_plain_point(draw: &crate::PrimitiveDraw) -> bool {
    draw.topology() == crate::PrimitiveTopology::Points
        && draw
            .point_style
            .as_ref()
            .is_some_and(|s| s.color_policy == crate::point_style::PointColorPolicy::Plain)
        && draw.user_program.is_none()
        && draw.lighting.is_none()
        && !draw.fixed_grid_point
        && !draw.polygon_points
        && !draw.polygon_face_lines
        && draw.clipping.is_none()
        && !draw.fixed_function_lighting
        && draw.surface_texture.is_none()
        && draw.surface_texture_unavailable.is_none()
}
fn ordinary_phong_draw(draw: &crate::PrimitiveDraw) -> bool {
    draw.topology() == crate::PrimitiveTopology::Triangles
        && draw.user_program.is_none()
        && !draw.polygon_face_lines
        && !draw.polygon_points
        && draw.clipping.is_none()
        && !draw.fixed_function_lighting
        && draw.lighting.as_ref().is_some_and(|lit| {
            lit.shading() == crate::RenderShading::Phong && !lit.pbr && lit.fixed.is_none()
        })
}
pub(super) fn attach(
    runtime: &std::sync::Arc<VulkanRuntime>,
    frame: &crate::RenderFrame,
    resources: &mut FrameResources,
) -> Result<(), RenderError> {
    let Some(request) = &runtime.vertex_output else {
        return Ok(());
    };
    let Some(ordinal) = target_ordinal(request, frame) else {
        return Ok(());
    };
    if request.format() == crate::VertexOutputFormat::PointsClipId20
        && (runtime.point_raster != crate::RenderPointRasterBehavior::HardwareQuadV1
            || resources.multisample.is_some())
    {
        return Err(unsupported(
            "nonmultisampled V1 point command",
            runtime.point_raster,
        ));
    }
    require_raster_uploads(frame, resources)?;
    attach_draw(runtime, request, &mut resources.draws[ordinal])
}
fn target_ordinal(
    request: &crate::VertexOutputRequest,
    frame: &crate::RenderFrame,
) -> Option<usize> {
    frame
        .draws
        .iter()
        .position(|draw| draw.vertex_output_target == request.source_token)
}
fn require_raster_uploads(
    frame: &crate::RenderFrame,
    resources: &FrameResources,
) -> Result<(), RenderError> {
    if resources.draws.len() != frame.draws.len()
        || resources.oit.is_some()
        || resources.raytrace.is_some()
    {
        return Err(unsupported(
            "one ordinary raster draw per input",
            resources.draws.len(),
        ));
    }
    Ok(())
}
fn attach_draw(
    runtime: &std::sync::Arc<VulkanRuntime>,
    request: &std::sync::Arc<crate::VertexOutputRequest>,
    draw: &mut super::frame::UploadedDraw,
) -> Result<(), RenderError> {
    require_indexed_command(request, draw)?;
    draw.vertex_output = Some(VertexOutputResources::new(runtime, request.clone())?);
    Ok(())
}
fn require_indexed_command(
    request: &crate::VertexOutputRequest,
    draw: &super::frame::UploadedDraw,
) -> Result<(), RenderError> {
    if !draw.pipeline.vertex_output_shader
        || draw.pipeline.diagnostic_modules().len() != 2
        || draw.count as usize != request.indices.len()
        || draw.user_program.is_some()
        || draw.line.wide.is_some()
        || draw.pipeline.kind.fixed_grid_point
    {
        return Err(unsupported(
            "ordinary unexpanded indexed command",
            draw.count,
        ));
    }
    require_single_instance(draw.instances)?;
    Ok(())
}

#[cfg(test)]
mod tests;

impl super::VulkanRenderer {
    /// Observe one explicitly marked indexed Phong draw; `Self::new_with_vertex_output(options, raster, coordinates, clipping, dots, request)?`.
    #[cfg(feature = "vertex-output-diagnostics")]
    pub fn new_with_vertex_output(
        options: super::VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        dots: crate::RenderLightDotBehavior,
        request: crate::VertexOutputRequest,
    ) -> Result<Self, RenderError> {
        let configuration = crate::RenderBehaviorConfiguration::default().with_light_dots(dots);
        Self::new_with_vertex_output_behavior(
            options,
            raster,
            coordinates,
            clipping,
            configuration,
            request,
        )
    }
    /// Observe actual outputs without replacing selected behavior; `Self::new_with_vertex_output_behavior(options, raster, coordinates, clipping, configuration, request)?`.
    #[cfg(feature = "vertex-output-diagnostics")]
    pub fn new_with_vertex_output_behavior(
        options: super::VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        configuration: crate::RenderBehaviorConfiguration,
        request: crate::VertexOutputRequest,
    ) -> Result<Self, RenderError> {
        let runtime = VulkanRuntime::create_with_vertex_output(
            options,
            raster,
            coordinates,
            clipping,
            configuration,
            request,
        )?;
        Self::initialized_renderer(runtime)
    }
}

pub(super) fn attached(
    runtime: &std::sync::Arc<VulkanRuntime>,
    frame: &crate::RenderFrame,
    mut resources: FrameResources,
) -> Result<FrameResources, RenderError> {
    attach(runtime, frame, &mut resources)?;
    Ok(resources)
}

fn require_single_instance(instances: u32) -> Result<(), RenderError> {
    if instances == 1 {
        return Ok(());
    }
    Err(unsupported("one noninstanced diagnostic draw", instances))
}
