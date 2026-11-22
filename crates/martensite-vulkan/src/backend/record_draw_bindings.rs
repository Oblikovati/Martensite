//! Actual draw bindings and push commands share one diagnostic observation boundary.
use super::{frame::UploadedDraw, pipeline::DrawPipeline, runtime::VulkanRuntime};
use ash::vk;

#[derive(Default)]
pub(super) struct LightingSlot(Option<vk::DescriptorSet>);

impl LightingSlot {
    pub(super) fn invalidate(&mut self) {
        self.0 = None;
    }

    fn was_bound(&mut self, set: vk::DescriptorSet, cacheable: bool) -> bool {
        let previous = self.0;
        self.0 = cacheable.then_some(set);
        self.0.is_some() && previous == self.0
    }
}

#[cfg(test)]
mod lighting_slot_tests {
    use super::*;
    use ash::vk::Handle;

    #[test]
    fn only_an_undisturbed_static_binding_can_be_reused() {
        let mut slot = LightingSlot::default();
        let first = vk::DescriptorSet::from_raw(1);
        let other = vk::DescriptorSet::from_raw(2);
        assert!(!slot.was_bound(first, true));
        assert!(slot.was_bound(first, true));
        assert!(!slot.was_bound(other, true));
        assert!(!slot.was_bound(other, false));
        assert!(!slot.was_bound(other, true));
        slot.invalidate();
        assert!(!slot.was_bound(other, true));
    }
}

pub(super) fn record_draw(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
    pipeline: &DrawPipeline,
    push: bool,
    oit_set: Option<vk::DescriptorSet>,
    lighting: &mut LightingSlot,
) {
    super::window_depth_status::bind(runtime, command, draw, pipeline);
    line_state(runtime, command, draw);
    bind_lighting(runtime, command, draw, pipeline, lighting);
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
    slot: &mut LightingSlot,
) {
    let Some(lighting) = &draw.bindings.lighting else {
        slot.invalidate();
        return;
    };
    if pipeline.kind.dynamic_lighting
        && pipeline
            .lighting_layout
            .as_ref()
            .is_some_and(|layout| layout.uses_dynamic_offset())
    {
        slot.invalidate();
        bind_descriptor_offsets(
            runtime,
            command,
            pipeline.layout,
            0,
            lighting.set,
            &[lighting.dynamic_offset()],
        );
        return;
    }
    let cacheable = !cfg!(feature = "command-state-diagnostics")
        && (pipeline.sprite_set != 0
            || (draw.bindings.sprite.is_none() && draw.bindings.surface_image.is_none()));
    if slot.was_bound(lighting.set, cacheable) {
        return;
    }
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
    bind_descriptor_offsets(runtime, command, layout, index, set, &[]);
}
fn bind_descriptor_offsets(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    index: u32,
    set: vk::DescriptorSet,
    dynamic_offsets: &[u32],
) {
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_bind_descriptor_sets(
            command,
            vk::PipelineBindPoint::GRAPHICS,
            layout,
            index,
            &[set],
            dynamic_offsets,
        );
    }
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_command_diagnostic::descriptor_bound(
        command,
        layout,
        index,
        set,
        dynamic_offsets,
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
