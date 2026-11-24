//! Admit a depth-only witness only for the ordinary single-sample opaque raster scope.
use super::{
    draw_table::RunSegment,
    frame::UploadedDraw,
    pipeline::DrawPipeline,
    pipeline_variants::RasterPipelines,
    placement_hiz_history::HiZView,
    placement_visibility::{HiZRequest, PlacementCompaction},
    record_draw_bindings::{self, BindingSlots},
    runtime::VulkanRuntime,
};
use crate::{
    RenderError, RenderLayerSpan, depth_prepass::RenderDepthPass, draw_instances::DrawInstances,
};
use ash::vk;
use std::sync::Arc;

pub(super) fn request<'a>(
    runtime: &Arc<VulkanRuntime>,
    pipelines: &'a RasterPipelines,
    (extent, samples, layer): (vk::Extent2D, vk::SampleCountFlags, Option<&RenderLayerSpan>),
    head: &UploadedDraw,
    instances: &DrawInstances,
) -> Result<Option<HiZRequest<'a>>, RenderError> {
    let Some(history) = &instances.hiz_history else {
        return Ok(None);
    };
    let Some(layer) = layer else { return Ok(None) };
    if !eligible(pipelines, samples, layer, head) {
        return Ok(None);
    }
    let Some(reduction) = pipelines.placement_hiz.get(runtime)? else {
        return Ok(None);
    };
    let Some(occluder) = pipelines.hiz_depth_twin(&head.pipeline)? else {
        return Ok(None);
    };
    let full = vk::Rect2D::default().extent(extent);
    let viewport = runtime
        .attachment_coordinates
        .raster_regions(extent, full, full)
        .viewport;
    let view = HiZView::new(runtime, extent, instances.projection_view);
    let extent = runtime.attachment_coordinates.attachment_extent(extent);
    Ok(Some(HiZRequest {
        history: history.clone(),
        view,
        reduction,
        occluder,
        extent,
        viewport,
        targets: &pipelines.frame_targets,
    }))
}

fn eligible(
    pipelines: &RasterPipelines,
    samples: vk::SampleCountFlags,
    layer: &RenderLayerSpan,
    head: &UploadedDraw,
) -> bool {
    let kind = head.pipeline.kind;
    let draw = &head.presentation;
    let mut bias = crate::polygon_offset::PolygonOffsetState::new(layer.polygon_offset);
    bias.advance(draw.polygon_offset, draw.polygon);
    samples == vk::SampleCountFlags::TYPE_1
        && !pipelines.stencil
        && super::capping_record::depth_policy(head, layer) == (true, true)
        && kind.depth_pass == RenderDepthPass::Shaded
        && !kind.clamp
        && !kind.compact_vertices
        && !kind.surface_blend
        && !draw.polygon_edge
        && !draw.text_shadow_bias
        && !head.sample_alpha_cutoff
        && !bias.raster_bias_enabled(draw.polygon, false)
}

pub(super) fn record(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    extent: vk::Extent2D,
    head: &UploadedDraw,
    (pipeline, table): (&DrawPipeline, vk::DescriptorSet),
    segments: &[RunSegment],
    compaction: &PlacementCompaction,
) {
    begin_raster(runtime, command, extent, head, pipeline);
    record_draw_bindings::bind_descriptor(runtime, command, pipeline.layout, 7, table);
    let mut slots = BindingSlots::default();
    for (index, segment) in segments.iter().enumerate() {
        slots.bind_buffers(
            runtime,
            command,
            (segment.vertices, segment.indices, segment.index_format),
        );
        compaction.draw_occluder_segment(command, index);
    }
}

fn begin_raster(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    extent: vk::Extent2D,
    head: &UploadedDraw,
    pipeline: &DrawPipeline,
) {
    super::render_target_record::set_viewport(runtime, command, extent);
    let culling = super::fixed_grid_record::fixed_grid_draw_culling(head, false);
    super::capping_record::set_culling(runtime, command, culling);
    let compare = super::pipeline_state::depth_pass_state(pipeline.kind).depth_compare_op;
    super::layer_depth_policy::DepthCommands {
        test: true,
        write: true,
        compare,
    }
    .apply(runtime, command);
    super::depth_bias_record::DepthBiasCommands::default().apply(runtime, command, false, [0.; 2]);
    super::ordered_stencil_record::apply(runtime, command, Default::default());
    // SAFETY: the scope is open with the plain depth format used by this frame-owned pipeline.
    unsafe {
        runtime.device().cmd_bind_pipeline(
            command,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline.handle,
        )
    };
}
