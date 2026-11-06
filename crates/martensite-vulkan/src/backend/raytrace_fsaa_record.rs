//! Rendering scopes and layout transitions of the FSAA targets (backend/raytrace_fsaa.rs).
use super::runtime::VulkanRuntime;
use super::{frame::FrameResources, raytrace_fsaa::FsaaTargets, raytrace_image::RaytraceImage};
use ash::vk;

type Scope = (vk::PipelineStageFlags2, vk::AccessFlags2);
const NONE: Scope = (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE);
// Program 0 loads target 1, so the attachment scope covers reads as well as writes.
const COLOR_WRITE: Scope = (
    vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
    vk::AccessFlags2::from_raw(
        vk::AccessFlags2::COLOR_ATTACHMENT_READ.as_raw()
            | vk::AccessFlags2::COLOR_ATTACHMENT_WRITE.as_raw(),
    ),
);
const DEPTH_WRITE: Scope = (
    vk::PipelineStageFlags2::from_raw(
        vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS.as_raw()
            | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS.as_raw(),
    ),
    vk::AccessFlags2::from_raw(
        vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ.as_raw()
            | vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE.as_raw(),
    ),
);
const SAMPLED: Scope = (
    vk::PipelineStageFlags2::FRAGMENT_SHADER,
    vk::AccessFlags2::SHADER_SAMPLED_READ,
);
const ATTACHMENT: vk::ImageLayout = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
const READ: vk::ImageLayout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
const TRANSFER_DST: vk::ImageLayout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
const CLEARED: Scope = (
    vk::PipelineStageFlags2::CLEAR,
    vk::AccessFlags2::TRANSFER_WRITE,
);

fn barriers(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    list: &[vk::ImageMemoryBarrier2<'_>],
) {
    let dependency = vk::DependencyInfo::default().image_memory_barriers(list);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_pipeline_barrier2(command, &dependency) };
}

/// Target 1 and the depth become attachments for program 0.
pub(super) fn begin_targets(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    targets: &FsaaTargets,
) {
    let depth = vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL;
    begin_accumulation(runtime, command, targets);
    barriers(
        runtime,
        command,
        &[
            targets.color[0].barrier((READ, ATTACHMENT), SAMPLED, COLOR_WRITE),
            targets
                .depth
                .barrier((vk::ImageLayout::UNDEFINED, depth), NONE, DEPTH_WRITE),
        ],
    );
}

/// Initialize new targets: both colours zeroed and readable (their alpha is never written while
/// alpha is masked, so it persists from frame to frame as in GlView's FBOs), the depth
/// readable. Example: `begin_accumulation(rt, cmd, &targets)`.
pub(super) fn begin_accumulation(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    targets: &FsaaTargets,
) {
    if targets
        .initialized
        .swap(true, std::sync::atomic::Ordering::Relaxed)
    {
        return;
    }
    targets
        .color
        .iter()
        .for_each(|image| clear_target(runtime, command, image));
    let undefined = vk::ImageLayout::UNDEFINED;
    barriers(
        runtime,
        command,
        &[targets.depth.barrier((undefined, READ), NONE, SAMPLED)],
    );
}

/// A new target starts zeroed (the texture of a new native FBO) and shader-readable.
fn clear_target(runtime: &VulkanRuntime, command: vk::CommandBuffer, image: &RaytraceImage) {
    let undefined = vk::ImageLayout::UNDEFINED;
    barriers(
        runtime,
        command,
        &[image.barrier((undefined, TRANSFER_DST), NONE, CLEARED)],
    );
    let black = vk::ClearColorValue { float32: [0.; 4] };
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_clear_color_image(
            command,
            image.handle,
            TRANSFER_DST,
            &black,
            &[image.range()],
        );
    }
    let to_read = [image.barrier((TRANSFER_DST, READ), CLEARED, SAMPLED)];
    barriers(runtime, command, &to_read);
}

/// Before a smoothing pass: `source` becomes readable, the other target writable.
pub(super) fn swap_targets(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    targets: &FsaaTargets,
    source: usize,
) {
    let target = 1 - source;
    // Target 2 is first written by pass 1; afterwards each target was last sampled.
    // Each target was last sampled (target 2 since its clear).
    let (old, previous) = (READ, SAMPLED);
    barriers(
        runtime,
        command,
        &[
            targets.color[source].barrier((ATTACHMENT, READ), COLOR_WRITE, SAMPLED),
            targets.color[target].barrier((old, ATTACHMENT), previous, COLOR_WRITE),
        ],
    );
}

/// Target 2 and the depth become readable for the output program.
pub(super) fn finish_targets(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    targets: &FsaaTargets,
) {
    let depth = vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL;
    barriers(
        runtime,
        command,
        &[
            targets.color[1].barrier((ATTACHMENT, READ), COLOR_WRITE, SAMPLED),
            targets.depth.barrier((depth, READ), DEPTH_WRITE, SAMPLED),
        ],
    );
}

/// A rendering scope over `color` and, for program 0, the depth cleared to 1
/// (`glClear(GL_DEPTH_BUFFER_BIT)`), with the frame viewport. Program 0 keeps the earlier
/// colour (it leaves missed pixels); the smoothing passes write every pixel.
pub(super) fn begin_pass(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    color: &RaytraceImage,
    depth: Option<&RaytraceImage>,
) {
    let colors = [vk::RenderingAttachmentInfo::default()
        .image_view(color.view)
        .image_layout(ATTACHMENT)
        .load_op(vk::AttachmentLoadOp::LOAD)
        .store_op(vk::AttachmentStoreOp::STORE)];
    let depth_attachment = depth.map(|depth| {
        vk::RenderingAttachmentInfo::default()
            .image_view(depth.view)
            .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.,
                    stencil: 0,
                },
            })
    });
    let area = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent: resources.attachment_extent,
    };
    let mut info = vk::RenderingInfo::default()
        .render_area(area)
        .layer_count(1)
        .color_attachments(&colors);
    if let Some(attachment) = depth_attachment.as_ref() {
        info = info.depth_attachment(attachment);
    }
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_begin_rendering(command, &info) };
    super::record::set_viewport(runtime, command, resources.extent);
}
