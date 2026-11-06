//! Explicit Vulkan 1.3 barriers connect rendering, transfer and coherent host readback.
pub(super) use super::render_target_record::{
    begin_rendering, depth_clear, render_attachment, set_viewport,
};
use super::{frame::FrameResources, image::TargetImage, runtime::VulkanRuntime};
#[path = "record_layer_commands.rs"]
mod layer_commands;
use super::layer_depth_policy::resolve_layer_draw_depth;
use crate::RenderFrame;
use ash::vk;
#[path = "record_alpha.rs"]
mod alpha;
#[path = "record_indexed.rs"]
mod indexed;
#[path = "record_sync.rs"]
mod sync;
pub(super) use layer_commands::{layer_dependency, record_layer, set_depth_policy};
pub(super) fn record_frame(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    frame: &RenderFrame,
) {
    super::window_depth_status::begin(resources);
    #[cfg(feature = "command-state-diagnostics")]
    {
        super::command_state_diagnostic::frame_start(command, resources);
        super::replay_device_diagnostic::frame(runtime, command);
    }
    #[cfg(feature = "lit-only-command-diagnostics")]
    runtime.lit_only_commands.begin_frame(
        command,
        &resources.draws,
        runtime.limits.sub_pixel_precision_bits,
    );
    #[cfg(feature = "vertex-output-diagnostics")]
    super::vertex_output::reset(runtime, command, resources);
    #[cfg(feature = "raytrace-input-diagnostics")]
    super::raytrace_input_diagnostic::frame(command, resources, frame);
    transition_targets(runtime, command, resources, false);
    super::oit_record::record_layers(runtime, command, resources, frame);
    transition_targets(runtime, command, resources, true);
    #[cfg(feature = "vertex-output-diagnostics")]
    super::vertex_output::host_barrier(runtime, command, resources);
    super::window_depth_status::host_barrier(runtime, command, resources);
    read_attachments(runtime, command, resources);
}
fn read_attachments(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
) {
    copy_attachment(
        runtime,
        command,
        &resources.color,
        resources.color_read.handle,
        resources.attachment_extent,
    );
    copy_attachment(
        runtime,
        command,
        &resources.depth,
        resources.depth_read.handle,
        resources.attachment_extent,
    );
    sync::host_barrier(runtime, command);
}
fn image_barrier(image: &TargetImage, readback: bool) -> vk::ImageMemoryBarrier2<'static> {
    let barrier = vk::ImageMemoryBarrier2::default()
        .image(image.handle)
        .subresource_range(image.range())
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED);
    if readback {
        return readback_transition(barrier, image.aspect);
    }
    attachment_transition(barrier)
}
fn attachment_transition(barrier: vk::ImageMemoryBarrier2<'_>) -> vk::ImageMemoryBarrier2<'_> {
    barrier
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
        .dst_stage_mask(
            vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT
                | vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
                | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
        )
        .dst_access_mask(
            vk::AccessFlags2::COLOR_ATTACHMENT_WRITE
                | vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE
                | vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ,
        )
}
fn readback_transition(
    barrier: vk::ImageMemoryBarrier2<'_>,
    aspect: vk::ImageAspectFlags,
) -> vk::ImageMemoryBarrier2<'_> {
    let (stage, access) = attachment_write_scope(aspect);
    barrier
        .old_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
        .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .src_stage_mask(stage)
        .src_access_mask(access)
        .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
        .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
}
/// The stages and accesses that write one attachment; `attachment_write_scope(aspect)`.
fn attachment_write_scope(
    aspect: vk::ImageAspectFlags,
) -> (vk::PipelineStageFlags2, vk::AccessFlags2) {
    if aspect.intersects(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL) {
        // A multisample depth/stencil resolve writes in COLOR_ATTACHMENT_OUTPUT with
        // COLOR_ATTACHMENT_WRITE (Vulkan "Render Pass Load/Store/Resolve" ordering).
        return (
            vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
                | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS
                | vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE
                | vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
        );
    }
    (
        vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
        vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
    )
}
#[cfg(test)]
fn record_draws(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    layer: &crate::RenderLayerSpan,
) {
    record_draws_selected(
        runtime,
        command,
        resources,
        layer,
        None,
        super::scene_color_scope::SceneColorScope::initialized_srgb(),
    );
}
pub(super) fn record_draws_selected(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    layer: &crate::RenderLayerSpan,
    selection: Option<(super::oit_pipeline::OitGeometryPhase, vk::DescriptorSet)>,
    mut color_scope: super::scene_color_scope::SceneColorScope,
) {
    let mut state = super::draw_state::DrawState::default();
    let mut depth_policy = None;
    let mut offsets = crate::polygon_offset::PolygonOffsetState::new(layer.polygon_offset);
    let mut culling = (crate::RenderFaceCulling::None, false);
    super::capping_record::set_culling(runtime, command, culling);
    for (index, draw) in resources.draws[layer.draw_range.clone()].iter().enumerate() {
        let draw_index = layer.draw_range.start + index;
        if resources.stencil.clear_before[draw_index] {
            super::capping_record::clear_stencil(runtime, command, resources.attachment_extent);
        }
        super::ordered_stencil_record::apply(
            runtime,
            command,
            resources.stencil.during[draw_index],
        );
        let pipeline = selection
            .map(|(phase, _)| {
                draw.oit
                    .as_ref()
                    .expect("uploaded OIT fill pipeline")
                    .pipeline(phase)
            })
            .unwrap_or(&draw.pipeline);
        // A registered user program replaces the built-in pipeline; in an OIT pass only its
        // OIT variants (selected at upload) draw.
        let phase = selection.map(|(phase, _)| phase);
        let user = draw
            .user_program
            .as_ref()
            .filter(|user| selection.is_none() || user.fixed.oit.is_some())
            .map(|user| (user, user.pipeline_for(phase)));
        let kind = user.map_or(pipeline.kind, |(user, _)| user.fixed.kind);
        let next_scope = super::scene_color_scope::SceneColorScope::for_pipeline(
            draw.presentation.text_logic_xor || draw.presentation.encoded_surface,
            kind,
            selection.is_none(),
        );
        color_scope.advance(runtime, command, resources, next_scope);
        let handle = user.map_or(pipeline.handle, |(_, linked)| linked.handle);
        let (bind, push) = state.advance(handle, &draw.transform);
        if bind {
            // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
            unsafe {
                runtime
                    .device()
                    .cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, handle)
            };
        }
        if draw.presentation.capping == crate::RenderCappingPass::MaskStart {
            super::capping_record::clear_stencil(runtime, command, resources.attachment_extent);
        }
        let policy = super::capping_record::depth_policy(draw, layer);
        let next = resolve_layer_draw_depth(policy, layer.depth_test, &draw.presentation, kind);
        if depth_policy != Some(next) {
            depth_policy = Some(next);
            next.apply(runtime, command);
        }
        let logical_facing = super::fixed_grid_record::logical_facing_active(
            runtime.triangle_coverage,
            pipeline.triangle_grid_coverage_shader,
        );
        let selected_culling =
            super::fixed_grid_record::fixed_grid_draw_culling(draw, logical_facing);
        if culling != selected_culling {
            culling = selected_culling;
            super::capping_record::set_culling(runtime, command, culling);
        }
        let ordinary_bias = if draw.presentation.polygon_edge {
            offsets.advance_polygon_line(draw.presentation.polygon_offset)
        } else {
            offsets.advance(draw.presentation.polygon_offset, draw.presentation.polygon)
        };
        let [units, factor] = if draw.presentation.text_shadow_bias && policy.0 {
            offsets.shadow_bias()
        } else {
            ordinary_bias
        };
        let requested_bias = (draw.presentation.text_shadow_bias && policy.0)
            || offsets
                .raster_bias_enabled(draw.presentation.polygon, draw.presentation.polygon_edge);
        let shader_depth = logical_facing;
        let bias_enabled = super::fixed_grid_record::fixed_grid_bias_enabled(
            draw.point_viewports.is_some(),
            requested_bias,
            shader_depth,
        );
        let hardware_bias = bias_enabled
            && !kind
                .window_depth
                .is_some_and(|parameters| parameters.fragment_depth());
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            // AspectPOM_Off disables offset; zero factors still bias edge-on triangles.
            runtime
                .device()
                .cmd_set_depth_bias_enable(command, hardware_bias);
            super::depth_bias_record::record(runtime, command, units, factor)
        };
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_raster_commands::hardware_bias_enable(command, hardware_bias);
        super::window_depth_record::record(
            runtime,
            command,
            pipeline.layout,
            kind,
            units,
            factor,
            bias_enabled,
        );
        #[cfg(feature = "command-state-diagnostics")]
        super::command_state_diagnostic::before_draw(
            (command, draw_index, handle),
            &draw.transform,
            super::command_state_diagnostic::DrawRasterArguments {
                depth: (next.test, next.write),
                bias: [units, factor],
                enabled: bias_enabled,
                user: user.is_some(),
                clipping: super::clipped_depth_composition::pipeline_modes(
                    runtime.world_clipping,
                    kind,
                ),
            },
            draw.bindings.lighting.as_ref(),
        );
        #[cfg(feature = "command-state-diagnostics")]
        super::geometry_command_diagnostic::before_draw((command, draw_index, handle), draw);
        if let Some((user, linked)) = user {
            user.record(command, linked);
            continue;
        }
        super::phong_interpolation_record::push_viewport(
            runtime,
            command,
            pipeline,
            draw,
            resources.extent,
            [units, factor],
            requested_bias,
        );
        super::phong_interpolation_record::push_text_grid_viewport(
            runtime,
            command,
            pipeline,
            resources.extent,
        );
        super::fixed_grid_record::begin_fixed_grid_draw(runtime, command, draw);
        super::record_draw_bindings::record_draw(
            runtime,
            command,
            draw,
            pipeline,
            push,
            alpha::selected_alpha_set(draw, selection.map(|(_, set)| set)),
        );
        if let Some(wide) = &draw.line.wide {
            super::render_target_record::record_wide_line(
                runtime,
                command,
                draw.count,
                wide,
                resources.extent,
            );
        } else {
            #[cfg(feature = "lit-only-command-diagnostics")]
            indexed::controlled(runtime, command, draw, draw_index);
            #[cfg(not(feature = "lit-only-command-diagnostics"))]
            indexed::record(runtime, command, draw);
        }
        super::fixed_grid_record::restore_fixed_grid_draw(runtime, command, draw, culling.1);
    }
}
fn copy_attachment(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    image: &TargetImage,
    buffer: vk::Buffer,
    extent: vk::Extent2D,
) {
    let layers = vk::ImageSubresourceLayers::default()
        .aspect_mask(image.aspect & !vk::ImageAspectFlags::STENCIL)
        .layer_count(1);
    let region = vk::BufferImageCopy::default()
        .image_subresource(layers)
        .image_extent(vk::Extent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        });
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_copy_image_to_buffer(
            command,
            image.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            buffer,
            &[region],
        );
    }
    #[cfg(feature = "command-state-diagnostics")]
    super::sampled_surface_image::replay_diagnostic::readback(
        command,
        image.handle,
        buffer,
        &[region],
    );
}
fn transition_targets(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    readback: bool,
) {
    let mut barriers = vec![
        image_barrier(&resources.color, readback),
        image_barrier(&resources.depth, readback),
    ];
    if !readback && let Some(targets) = &resources.multisample {
        barriers.extend([
            image_barrier(&targets.color, false),
            image_barrier(&targets.depth, false),
        ]);
    }
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().image_memory_barriers(&barriers),
        );
    }
}

pub(super) fn finish_rendering(runtime: &VulkanRuntime, command: vk::CommandBuffer) {
    // All callers end an active dynamic-rendering scope before transfer barriers.
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_end_rendering(command) };
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_raster_commands::end_rendering(command);
}
#[cfg(test)]
#[path = "record_region_tests.rs"]
mod region_tests;
