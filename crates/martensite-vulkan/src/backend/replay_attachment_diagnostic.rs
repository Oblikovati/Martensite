//! Dynamic-rendering API attachment operands, not inferred final ownership.
use ash::{vk, vk::Handle};

pub(super) fn begin(command: vk::CommandBuffer, info: &vk::RenderingInfo<'_>) {
    emit_begin(command, info);
    let colors = color_attachments(info);
    for (index, attachment) in colors.iter().enumerate() {
        emit(command, "color", index, attachment);
    }
    unsafe {
        optional(command, "depth", info.p_depth_attachment);
        optional(command, "stencil", info.p_stencil_attachment);
    }
}
fn emit_begin(command: vk::CommandBuffer, info: &vk::RenderingInfo<'_>) {
    let area = info.render_area;
    eprintln!(
        "{{\"kind\":\"command-begin-rendering\",\"command\":{},\"area\":[{},{},{},{}],\"layers\":{},\"view_mask\":{},\"color_count\":{},\"flags\":{},\"extensions_present\":{}}}",
        command.as_raw(),
        area.offset.x,
        area.offset.y,
        area.extent.width,
        area.extent.height,
        info.layer_count,
        info.view_mask,
        info.color_attachment_count,
        info.flags.as_raw(),
        !info.p_next.is_null()
    );
}
fn color_attachments<'a>(info: &'a vk::RenderingInfo<'_>) -> &'a [vk::RenderingAttachmentInfo<'a>] {
    if info.color_attachment_count == 0 {
        return &[];
    }
    // SAFETY: caller retains the successful rendering description and its attachment array.
    unsafe {
        std::slice::from_raw_parts(
            info.p_color_attachments,
            info.color_attachment_count as usize,
        )
    }
}
unsafe fn optional(
    command: vk::CommandBuffer,
    role: &str,
    pointer: *const vk::RenderingAttachmentInfo<'_>,
) {
    // SAFETY: nonnull pointers belong to the caller's live rendering description.
    if let Some(attachment) = unsafe { pointer.as_ref() } {
        emit(command, role, 0, attachment);
    }
}
fn emit(
    command: vk::CommandBuffer,
    role: &str,
    index: usize,
    attachment: &vk::RenderingAttachmentInfo<'_>,
) {
    eprintln!(
        "{}",
        attachment_json(command.as_raw(), role, index, attachment)
    );
}
fn attachment_json(
    command: u64,
    role: &str,
    index: usize,
    attachment: &vk::RenderingAttachmentInfo<'_>,
) -> String {
    let clear = clear_words(role, attachment);
    format!(
        "{{\"kind\":\"command-rendering-attachment\",\"command\":{command},\"role\":\"{role}\",\"index\":{index},\"view\":{},\"layout\":{},\"resolve\":[{},{},{}],\"load\":{},\"store\":{},\"clear_words\":{clear:?}}}",
        attachment.image_view.as_raw(),
        attachment.image_layout.as_raw(),
        attachment.resolve_mode.as_raw(),
        attachment.resolve_image_view.as_raw(),
        attachment.resolve_image_layout.as_raw(),
        attachment.load_op.as_raw(),
        attachment.store_op.as_raw()
    )
}
fn clear_words(role: &str, attachment: &vk::RenderingAttachmentInfo<'_>) -> Vec<u32> {
    if attachment.load_op != vk::AttachmentLoadOp::CLEAR {
        return Vec::new();
    }
    // Read only the active initialized member; depth/stencil does not initialize all color words.
    if role == "color" {
        return unsafe { attachment.clear_value.color.uint32 }.to_vec();
    }
    let value = unsafe { attachment.clear_value.depth_stencil };
    vec![value.depth.to_bits(), value.stencil]
}
pub(super) fn scissor(command: vk::CommandBuffer, rectangle: vk::Rect2D) {
    let words = [
        i64::from(rectangle.offset.x),
        i64::from(rectangle.offset.y),
        i64::from(rectangle.extent.width),
        i64::from(rectangle.extent.height),
    ];
    eprintln!(
        "{{\"kind\":\"command-scissor\",\"command\":{},\"xywh\":{words:?}}}",
        command.as_raw()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nonclear_load_does_not_read_an_inactive_clear_union() {
        let attachment = vk::RenderingAttachmentInfo::default().load_op(vk::AttachmentLoadOp::LOAD);
        assert!(clear_words("depth", &attachment).is_empty());
    }
    #[test]
    fn depth_stencil_reads_only_its_initialized_two_words() {
        let clear = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 0.5,
                stencil: 9,
            },
        };
        let attachment = vk::RenderingAttachmentInfo::default()
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .clear_value(clear);
        assert_eq!(clear_words("depth", &attachment), [0.5f32.to_bits(), 9]);
    }
    #[test]
    fn color_clear_bits_and_load_store_are_actual_operands() {
        let clear = vk::ClearValue {
            color: vk::ClearColorValue {
                uint32: [1, 2, 3, 4],
            },
        };
        let attachment = vk::RenderingAttachmentInfo::default()
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(clear);
        let json = attachment_json(7, "color", 2, &attachment);
        assert!(json.contains("\"clear_words\":[1, 2, 3, 4]"));
        assert!(json.contains("\"load\":1,\"store\":0"));
    }
}
