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
pub(super) use layer_commands::{
    layer_dependency, record_layer, record_secondary_layer, set_depth_policy,
};
pub(super) fn record_submission_frame(
    submission: &super::submission::CommandSubmission,
    resources: &FrameResources,
    frame: &RenderFrame,
) -> Result<(), crate::RenderError> {
    let linked = submission.prepare_geometry_transfer(&resources.geometry_transfers)?;
    record_frame_impl(
        &submission.runtime,
        submission.command,
        resources,
        frame,
        !linked,
    );
    Ok(())
}

#[cfg(test)]
pub(super) fn record_frame(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    frame: &RenderFrame,
) {
    record_frame_impl(runtime, command, resources, frame, true);
}

fn record_frame_impl(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    frame: &RenderFrame,
    inline_geometry: bool,
) {
    let _recording = runtime.work.enter(super::WorkPhase::Recording);
    if inline_geometry {
        resources.geometry_transfers.record(runtime, command);
    }
    if let [draw] = resources.draws.as_slice()
        && let Some(run) = &draw.table_run
    {
        run.record_compaction(runtime, command, resources.extent, draw);
    }
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
    let start = match &resources.immediate_source {
        Some((start, images)) => {
            images.restore(runtime, command, resources);
            *start
        }
        None => {
            transition_targets(runtime, command, resources, false);
            0
        }
    };
    super::oit_record::record_layers(runtime, command, resources, frame, start);
    transition_targets(runtime, command, resources, true);
    #[cfg(feature = "vertex-output-diagnostics")]
    super::vertex_output::host_barrier(runtime, command, resources);
    super::window_depth_status::host_barrier(runtime, command, resources);
}
/// Copy the colour and depth attachments, left in `TRANSFER_SRC_OPTIMAL` by their frame, into
/// host buffers; a `None` read is skipped;
/// `record_attachment_reads(rt, command, [Some((&color, buffer)), None], extent)`.
pub(super) fn record_attachment_reads(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    reads: [Option<(&TargetImage, vk::Buffer)>; 2],
    extent: vk::Extent2D,
) {
    for (image, buffer) in reads.into_iter().flatten() {
        copy_attachment(runtime, command, image, buffer, extent);
    }
    sync::host_barrier(runtime, command);
}
pub(super) fn image_barrier(
    image: &TargetImage,
    readback: bool,
) -> vk::ImageMemoryBarrier2<'static> {
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
/// A restored frame without immediate draws reads the restore draw's attachment writes,
/// which this scope covers too.
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
pub(super) fn attachment_write_scope(
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
    let mut bindings = super::record_draw_bindings::BindingSlots::default();
    let mut bias_commands = super::depth_bias_record::DepthBiasCommands::default();
    let mut depth_policy = None;
    let mut offsets = crate::polygon_offset::PolygonOffsetState::new(layer.polygon_offset);
    let mut culling = (crate::RenderFaceCulling::None, false);
    super::capping_record::set_culling(runtime, command, culling);
    let start = layer.draw_range.start - resources.first_draw;
    let end = layer.draw_range.end - resources.first_draw;
    // Every built-in pipeline takes stencil as dynamic state, so a state recorded once holds
    // until it changes; each draw recorded five stencil commands before (#594).
    let mut stencil = None;
    // The draws after the first of an indirect run record within the run (draw_table.rs).
    let mut resume = 0;
    for (index, draw) in resources.draws[start..end].iter().enumerate() {
        if index < resume {
            continue;
        }
        let draw_index = layer.draw_range.start + index;
        if resources.stencil.clear_before[draw_index] {
            super::capping_record::clear_stencil(runtime, command, resources.attachment_extent);
        }
        let next_stencil = resources.stencil.during[draw_index];
        if cfg!(feature = "command-state-diagnostics") || stencil != Some(next_stencil) {
            super::ordered_stencil_record::apply(runtime, command, next_stencil);
            stencil = Some(next_stencil);
        }
        let pipeline = selection
            .map(|(phase, _)| {
                draw.oit
                    .as_ref()
                    .expect("uploaded OIT fill pipeline")
                    .pipeline(phase)
            })
            .unwrap_or(&draw.pipeline);
        // OIT passes record the draws of a run one by one, with their own pipelines.
        let run = draw.table_run.as_deref().filter(|_| selection.is_none());
        let pipeline = run.map_or(pipeline, |run| &*run.pipeline);
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
            bindings.lighting.invalidate();
            bindings.invalidate_sets();
            bias_commands.invalidate();
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
        // AspectPOM_Off disables offset; zero factors still bias edge-on triangles.
        bias_commands.apply(runtime, command, hardware_bias, [units, factor]);
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
            draw.bindings.lighting.as_deref(),
        );
        #[cfg(feature = "command-state-diagnostics")]
        super::geometry_command_diagnostic::before_draw((command, draw_index, handle), draw);
        if let Some((user, linked)) = user {
            bindings.lighting.invalidate();
            bias_commands.invalidate();
            bindings.invalidate_geometry();
            bindings.invalidate_sets();
            // A user pipeline may hold static stencil state, which leaves the dynamic state
            // undefined for the next draw.
            stencil = None;
            user.record(command, linked);
            continue;
        }
        if selection.is_none()
            && let Some(Some(meshlet)) = resources.meshlets.get(start + index)
        {
            meshlet.record(runtime, command, draw, resources.extent);
            state = super::draw_state::DrawState::default();
            bindings.lighting.invalidate();
            bindings.invalidate_sets();
            bias_commands.invalidate();
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
        let indices = super::record_draw_bindings::record_draw(
            runtime,
            command,
            draw,
            pipeline,
            push,
            alpha::selected_alpha_set(draw, selection.map(|(_, set)| set)),
            &mut bindings,
        );
        if let Some(wide) = &draw.line.wide {
            super::render_target_record::record_wide_line(
                runtime,
                command,
                draw.count,
                wide,
                resources.extent,
            );
            stencil = None;
            bindings.invalidate_sets();
        } else if let Some(run) = run {
            super::draw_table::record_run(runtime, command, run, &mut bindings);
            resume = index + run.count;
        } else {
            #[cfg(feature = "lit-only-command-diagnostics")]
            indexed::controlled(runtime, command, draw, draw_index, indices);
            #[cfg(not(feature = "lit-only-command-diagnostics"))]
            indexed::record(runtime, command, draw, indices);
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
