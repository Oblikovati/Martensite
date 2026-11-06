//! Actual draw bindings and push commands share one diagnostic observation boundary.
use super::{frame::UploadedDraw, pipeline::DrawPipeline, runtime::VulkanRuntime};
use ash::vk;

pub(super) fn record_draw(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
    pipeline: &DrawPipeline,
    push: bool,
    oit_set: Option<vk::DescriptorSet>,
) {
    super::window_depth_status::bind(runtime, command, draw, pipeline);
    line_state(runtime, command, draw);
    bind_lighting(runtime, command, draw, pipeline);
    for (index, set) in remaining_descriptors(draw, pipeline, oit_set) {
        if let Some(set) = set {
            bind_descriptor(runtime, command, pipeline.layout, index, set);
        }
    }
    bind_geometry(runtime, command, draw);
    if push {
        push_draw_transform(runtime, command, draw, pipeline);
    }
}
fn line_state(runtime: &VulkanRuntime, command: vk::CommandBuffer, draw: &UploadedDraw) {
    if let Some((factor, pattern)) = draw.line.stipple {
        // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
        unsafe {
            runtime
                .line_stipple
                .as_ref()
                .expect("fixed stipple capability validated")
                .set(command, factor, pattern);
        }
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_raster_commands::line_stipple(command, factor, pattern);
    }
    if let Some(width) = draw.line.width {
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            runtime.device().cmd_set_line_width(command, width);
        }
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_raster_commands::line_width(command, width);
    }
}
fn bind_lighting(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
    pipeline: &DrawPipeline,
) {
    let Some(lighting) = &draw.bindings.lighting else {
        return;
    };
    bind_descriptor(runtime, command, pipeline.layout, 0, lighting.set);
    #[cfg(feature = "command-state-diagnostics")]
    super::command_state_diagnostic::bound(
        (command, pipeline.handle, pipeline.layout),
        lighting.set,
    );
}
fn remaining_descriptors(
    draw: &UploadedDraw,
    pipeline: &DrawPipeline,
    oit_set: Option<vk::DescriptorSet>,
) -> [(u32, Option<vk::DescriptorSet>); 8] {
    let bindings = &draw.bindings;
    [
        (2, bindings.sphere_map.as_ref().map(|value| value.set)),
        (
            pipeline.sprite_set,
            bindings.sprite.as_ref().map(|value| value.set),
        ),
        (
            pipeline.sprite_set,
            bindings.surface_image.as_ref().map(|value| value.set),
        ),
        (1, bindings.clipping.as_ref().map(|value| value.set)),
        (4, bindings.pbr_environment.as_ref().map(|value| value.set)),
        (6, bindings.shadow_maps.as_ref().map(|value| value.set)),
        (5, bindings.material_maps.as_ref().map(|value| value.set)),
        (3, oit_set),
    ]
}
pub(super) fn bind_descriptor(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    index: u32,
    set: vk::DescriptorSet,
) {
    let dynamic_offsets = [];
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_bind_descriptor_sets(
            command,
            vk::PipelineBindPoint::GRAPHICS,
            layout,
            index,
            &[set],
            &dynamic_offsets,
        );
    }
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_command_diagnostic::descriptor_bound(
        command,
        layout,
        index,
        set,
        &dynamic_offsets,
    );
}
fn bind_geometry(runtime: &VulkanRuntime, command: vk::CommandBuffer, draw: &UploadedDraw) {
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime
            .device()
            .cmd_bind_vertex_buffers(command, 0, &[draw.vertices.handle], &[0]);
        runtime.device().cmd_bind_index_buffer(
            command,
            draw.indices.handle,
            0,
            vk::IndexType::UINT32,
        );
    }
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_raster_commands::geometry(command, draw.vertices.handle, draw.indices.handle);
}
fn push_draw_transform(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
    pipeline: &DrawPipeline,
) {
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_push_constants(
            command,
            pipeline.layout,
            pipeline.push_stages,
            0,
            &draw.transform,
        );
    }
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_command_diagnostic::push(
        command,
        pipeline.layout,
        pipeline.push_stages,
        0,
        &draw.transform,
    );
}
