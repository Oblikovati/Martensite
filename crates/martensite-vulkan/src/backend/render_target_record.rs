//! Dynamic scene attachments preserve resolves through every encoded and linear pass boundary.
use super::{frame::FrameResources, runtime::VulkanRuntime};
use ash::vk;

pub(super) fn begin_rendering(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    clear: [f32; 4],
    first: bool,
    clear_depth: bool,
    encoded_attachment: bool,
) {
    let colors = [scene_color_attachment(
        resources,
        clear,
        first,
        encoded_attachment,
    )];
    let depth = scene_depth_attachment(resources, first || clear_depth);
    let mut stencil = scene_depth_attachment(resources, first);
    stencil.clear_value = vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
            depth: 1.,
            stencil: u32::from(resources.stencil_clear_value),
        },
    };
    begin_scene_attachments(runtime, command, resources, &colors, &depth, &stencil);
    set_viewport(runtime, command, resources.extent);
}
// Layer initialization can clear depth/stencil but must not reinterpret prior color samples.
pub(super) fn begin_empty_layer_rendering(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    clear_depth: bool,
) {
    let depth = scene_depth_attachment(resources, clear_depth);
    let stencil = scene_depth_attachment(resources, false);
    begin_scene_attachments(runtime, command, resources, &[], &depth, &stencil);
    set_viewport(runtime, command, resources.extent);
}
pub(super) fn resume_color_draw_rendering(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    encoded: bool,
) {
    // Loading both aspects preserves color, depth and stencil through every XOR boundary.
    // Native FFP encoded-space blending uses this same UNORM alias without enabling logicOp.
    begin_rendering(runtime, command, resources, [0.; 4], false, false, encoded);
}
pub(super) fn begin_colorless_draw_rendering(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    encoded: bool,
) {
    let color = scene_color_attachment(resources, [0.; 4], false, encoded);
    let colors = [without_color_resolve(color)];
    let depth = scene_depth_attachment(resources, false);
    let stencil = scene_depth_attachment(resources, false);
    begin_scene_attachments(runtime, command, resources, &colors, &depth, &stencil);
    set_viewport(runtime, command, resources.extent);
}
fn without_color_resolve(
    color: vk::RenderingAttachmentInfo<'static>,
) -> vk::RenderingAttachmentInfo<'static> {
    color
        .resolve_mode(vk::ResolveModeFlags::NONE)
        .resolve_image_view(vk::ImageView::null())
        .resolve_image_layout(vk::ImageLayout::UNDEFINED)
}
fn scene_color_attachment(
    resources: &FrameResources,
    clear: [f32; 4],
    first: bool,
    encoded: bool,
) -> vk::RenderingAttachmentInfo<'static> {
    let source = if encoded {
        resources.render_color().xor_view
    } else {
        resources.render_color().view
    };
    let target = if encoded {
        resources.color.xor_view
    } else {
        resources.color.view
    };
    super::multisample_targets::resolve_attachment(
        render_attachment(source, color_clear(clear), first),
        target,
        vk::ResolveModeFlags::AVERAGE,
        resources.multisample.is_some(),
    )
}
fn scene_depth_attachment(
    resources: &FrameResources,
    reset: bool,
) -> vk::RenderingAttachmentInfo<'static> {
    super::multisample_targets::resolve_attachment(
        render_attachment(resources.render_depth().view, depth_clear(), reset),
        resources.depth.view,
        vk::ResolveModeFlags::SAMPLE_ZERO,
        resources.multisample.is_some(),
    )
}
fn begin_scene_attachments(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    colors: &[vk::RenderingAttachmentInfo<'_>],
    depth: &vk::RenderingAttachmentInfo<'_>,
    stencil: &vk::RenderingAttachmentInfo<'_>,
) {
    let area = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent: resources.attachment_extent,
    };
    let mut info = vk::RenderingInfo::default()
        .render_area(area)
        .layer_count(1)
        .color_attachments(colors)
        .depth_attachment(depth);
    if resources
        .depth
        .aspect
        .contains(vk::ImageAspectFlags::STENCIL)
    {
        info = info.stencil_attachment(stencil);
    }
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_begin_rendering(command, &info) };
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_attachment_diagnostic::begin(command, &info);
}

pub(super) fn set_viewport(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    extent: vk::Extent2D,
) {
    let full = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent,
    };
    set_raster_region(runtime, command, extent, full, full);
}
/// Draw an emulated wide line (`crate::wide_line`): every one-pixel replica of the x-major
/// indices, shifted towards GL up, then of the y-major ones, shifted right; the viewport is
/// restored afterwards. `record_wide_line(ctx, cmd, draw.count, wide, resources.extent)`.
pub(super) fn record_wide_line(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    count: u32,
    wide: &super::frame::WideLineDraw,
    extent: vk::Extent2D,
) {
    let full = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent,
    };
    let base = runtime
        .attachment_coordinates
        .raster_regions(extent, full, full)
        .viewport;
    let groups = [
        (0, wide.x_major, [0., 1.]),
        (wide.x_major, count - wide.x_major, [1., 0.]),
    ];
    for (first, indices, axis) in groups.into_iter().filter(|group| group.1 > 0) {
        for &offset in &wide.offsets {
            let mut viewport = base;
            viewport.x += offset * axis[0];
            // GL up is the direction of increasing NDC y, whatever the viewport orientation.
            viewport.y += offset * axis[1] * base.height.signum();
            // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
            unsafe {
                let device = runtime.device();
                device.cmd_set_viewport(command, 0, &[viewport]);
                device.cmd_draw_indexed(command, indices, 1, first, 0, 0);
            }
        }
    }
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_set_viewport(command, 0, &[base]) };
}
/// Record canonical regions; `set_raster_region(ctx, cmd, extent, full, full)` keeps top-left inputs.
pub(super) fn set_raster_region(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    extent: vk::Extent2D,
    viewport: vk::Rect2D,
    scissor: vk::Rect2D,
) {
    let regions = runtime
        .attachment_coordinates
        .raster_regions(extent, viewport, scissor);
    #[cfg(feature = "command-state-diagnostics")]
    super::command_state_diagnostic::viewport(command, regions.viewport);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime
            .device()
            .cmd_set_viewport(command, 0, &[regions.viewport]);
        runtime
            .device()
            .cmd_set_scissor(command, 0, &[regions.scissor]);
    }
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_attachment_diagnostic::scissor(command, regions.scissor);
}

pub(super) fn render_attachment(
    view: vk::ImageView,
    clear: vk::ClearValue,
    reset: bool,
) -> vk::RenderingAttachmentInfo<'static> {
    vk::RenderingAttachmentInfo::default()
        .image_view(view)
        .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
        .load_op(if reset {
            vk::AttachmentLoadOp::CLEAR
        } else {
            vk::AttachmentLoadOp::LOAD
        })
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(clear)
}

fn color_clear(value: [f32; 4]) -> vk::ClearValue {
    vk::ClearValue {
        color: vk::ClearColorValue { float32: value },
    }
}

pub(super) fn depth_clear() -> vk::ClearValue {
    vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
            depth: 1.,
            stencil: 0,
        },
    }
}

#[cfg(test)]
mod colorless_tests {
    use super::*;
    use ash::vk::Handle;
    #[test]
    fn no_color_resolve_preserves_source_view_layout_and_load_store() {
        let source = vk::ImageView::from_raw(17);
        let color = render_attachment(source, color_clear([0.; 4]), false)
            .resolve_mode(vk::ResolveModeFlags::AVERAGE)
            .resolve_image_view(vk::ImageView::from_raw(18))
            .resolve_image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL);
        let colorless = without_color_resolve(color);
        assert_eq!(colorless.image_view, source);
        assert_eq!(colorless.image_layout, color.image_layout);
        assert_eq!(colorless.load_op, vk::AttachmentLoadOp::LOAD);
        assert_eq!(colorless.store_op, vk::AttachmentStoreOp::STORE);
        assert_eq!(colorless.resolve_mode, vk::ResolveModeFlags::NONE);
        assert_eq!(colorless.resolve_image_view, vk::ImageView::null());
        assert_eq!(colorless.resolve_image_layout, vk::ImageLayout::UNDEFINED);
    }
}
