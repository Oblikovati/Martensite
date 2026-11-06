//! Layer scopes retain their attachment dependencies and inherited depth flags.
use super::super::scene_color_scope::SceneColorScope;
use super::{
    FrameResources, VulkanRuntime, begin_rendering, finish_rendering, record_draws_selected,
};
use ash::vk;
use ash::vk::{AccessFlags2 as Access, PipelineStageFlags2 as Stages};
pub(in super::super) fn record_layer(
    ctx: &VulkanRuntime,
    cmd: vk::CommandBuffer,
    target: &FrameResources,
    clear: [f32; 4],
    index: usize,
    layer: &crate::RenderLayerSpan,
) {
    // A cubemap background has already cleared and drawn the attachments.
    let first = index == 0 && target.background.is_none();
    if !first {
        layer_dependency(ctx, cmd);
    }
    begin_layer_scope(ctx, cmd, target, clear, layer, first);
    set_depth_policy(ctx, cmd, layer);
    record_layer_draws(ctx, cmd, target, layer, first);
    super::super::ordered_stencil_record::finish_layer(ctx, cmd, target, index);
    finish_rendering(ctx, cmd);
}
fn record_layer_draws(
    ctx: &VulkanRuntime,
    cmd: vk::CommandBuffer,
    target: &FrameResources,
    layer: &crate::RenderLayerSpan,
    first: bool,
) {
    let scope = if first {
        SceneColorScope::initialized_srgb()
    } else {
        SceneColorScope::default()
    };
    record_draws_selected(ctx, cmd, target, layer, None, scope);
}
fn begin_layer_scope(
    ctx: &VulkanRuntime,
    cmd: vk::CommandBuffer,
    target: &FrameResources,
    clear: [f32; 4],
    layer: &crate::RenderLayerSpan,
    first: bool,
) {
    if !first {
        super::super::render_target_record::begin_empty_layer_rendering(
            ctx,
            cmd,
            target,
            layer.clear_depth,
        );
        return;
    }
    begin_rendering(ctx, cmd, target, clear, first, layer.clear_depth, false);
}
pub(in super::super) fn layer_dependency(runtime: &VulkanRuntime, command: vk::CommandBuffer) {
    let access = Access::COLOR_ATTACHMENT_WRITE | Access::DEPTH_STENCIL_ATTACHMENT_WRITE;
    let reads = Access::COLOR_ATTACHMENT_READ | Access::DEPTH_STENCIL_ATTACHMENT_READ;
    // Layers only exchange attachment contents: fragment tests and colour output on both sides.
    let stages = Stages::COLOR_ATTACHMENT_OUTPUT
        | Stages::EARLY_FRAGMENT_TESTS
        | Stages::LATE_FRAGMENT_TESTS;
    let barriers = [vk::MemoryBarrier2::default()
        .src_stage_mask(stages)
        .src_access_mask(access)
        .dst_stage_mask(stages)
        .dst_access_mask(access | reads)];
    let info = vk::DependencyInfo::default().memory_barriers(&barriers);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_pipeline_barrier2(command, &info) };
}

pub(in super::super) fn set_depth_policy(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    layer: &crate::RenderLayerSpan,
) {
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime
            .device()
            .cmd_set_depth_test_enable(command, layer.depth_test);
        runtime
            .device()
            .cmd_set_depth_write_enable(command, layer.depth_write);
    }
}
