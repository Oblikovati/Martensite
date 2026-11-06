//! Actual dynamic raster and indexed-command operands, separate from logical policy.
use ash::{vk, vk::Handle};
use serde_json::json;

pub(super) fn line_width(command: vk::CommandBuffer, width: f32) {
    eprintln!("{}", line_width_json(command.as_raw(), width));
}
fn line_width_json(command: u64, width: f32) -> serde_json::Value {
    json!({"kind":"command-line-width","command":command,"width_bits":width.to_bits()})
}
pub(super) fn line_stipple(command: vk::CommandBuffer, factor: u32, pattern: u16) {
    eprintln!("{}", line_stipple_json(command.as_raw(), factor, pattern));
}
fn line_stipple_json(command: u64, factor: u32, pattern: u16) -> serde_json::Value {
    json!({"kind":"command-line-stipple","command":command,"factor":factor,"pattern":pattern})
}

pub(super) fn hardware_bias_enable(command: vk::CommandBuffer, enabled: bool) {
    eprintln!(
        "{}",
        json!({"kind":"command-hardware-depth-bias-enable","command":command.as_raw(),"enabled":enabled})
    );
}
pub(super) fn hardware_bias(
    command: vk::CommandBuffer,
    values: [f32; 3],
    representation: Option<(i32, bool)>,
) {
    eprintln!(
        "{}",
        json!({"kind":"command-hardware-depth-bias","command":command.as_raw(),
        "constant_clamp_slope_bits":values.map(f32::to_bits),"representation_exact":representation})
    );
}
pub(super) fn geometry(command: vk::CommandBuffer, vertex: vk::Buffer, index: vk::Buffer) {
    eprintln!(
        "{}",
        json!({"kind":"command-geometry-bound","command":command.as_raw(),
        "vertex_binding":0,"vertices":[vertex.as_raw()],"vertex_offsets":[0],"index":index.as_raw(),
        "index_offset":0,"index_type":vk::IndexType::UINT32.as_raw()})
    );
}
pub(super) fn indexed(command: vk::CommandBuffer, arguments: (u32, u32, u32, i32, u32)) {
    eprintln!(
        "{}",
        json!({"kind":"command-draw-indexed","command":command.as_raw(),"count":arguments.0,
        "instances":arguments.1,"first_index":arguments.2,"vertex_offset":arguments.3,"first_instance":arguments.4})
    );
}
pub(super) fn end_rendering(command: vk::CommandBuffer) {
    eprintln!(
        "{}",
        json!({"kind":"command-end-rendering","command":command.as_raw()})
    );
}
pub(super) fn clear_stencil(
    command: vk::CommandBuffer,
    clear: vk::ClearAttachment,
    rect: vk::ClearRect,
) {
    let value = unsafe { clear.clear_value.depth_stencil };
    eprintln!(
        "{}",
        json!({"kind":"command-clear-stencil","command":command.as_raw(),"aspect":clear.aspect_mask.as_raw(),
        "color_attachment":clear.color_attachment,"depth_bits":value.depth.to_bits(),"stencil":value.stencil,
        "rect":[rect.rect.offset.x as i64,rect.rect.offset.y as i64,i64::from(rect.rect.extent.width),i64::from(rect.rect.extent.height)],
        "base_layer":rect.base_array_layer,"layers":rect.layer_count})
    );
}

pub(super) fn submission_begin(
    device: vk::Device,
    command: vk::CommandBuffer,
    fence: vk::Fence,
    pool: vk::CommandPool,
) {
    eprintln!(
        "{}",
        json!({"kind":"submission-recording-begin","device":device.as_raw(),"command":command.as_raw(),"fence":fence.as_raw(),"pool":pool.as_raw()})
    );
}
pub(super) fn submitted(
    command: vk::CommandBuffer,
    fence: vk::Fence,
    queue: vk::Queue,
    waits: usize,
    signals: usize,
) {
    eprintln!(
        "{}",
        json!({"kind":"submission-queue-submit-succeeded","command":command.as_raw(),"fence":fence.as_raw(),
        "queue":queue.as_raw(),"wait_count":waits,"signal_count":signals})
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_width_keeps_actual_bits_including_diagnostic_invalid_values() {
        for bits in [1.25_f32.to_bits(), 0x8000_0000, 0x7fc0_1234] {
            assert_eq!(
                line_width_json(17, f32::from_bits(bits)),
                json!({"kind":"command-line-width","command":17,"width_bits":bits})
            );
        }
    }

    #[test]
    fn stipple_keeps_factor_and_all_pattern_bits_independent() {
        assert_eq!(
            line_stipple_json(19, 256, 0x8001),
            json!({"kind":"command-line-stipple","command":19,"factor":256,"pattern":32769})
        );
    }
}
