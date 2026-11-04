//! Completed dynamic API calls retain their literal operands, not inferred GPU state.
use crate::RenderStencilState;
use ash::{vk, vk::Handle};

pub(super) fn enabled(command: vk::CommandBuffer, enabled: bool) {
    eprintln!("{}", enabled_json(command.as_raw(), enabled));
}
fn enabled_json(command: u64, enabled: bool) -> String {
    format!("{{\"kind\":\"command-stencil-enable\",\"command\":{command},\"enabled\":{enabled}}}")
}
pub(super) fn operations(
    command: vk::CommandBuffer,
    faces: vk::StencilFaceFlags,
    operations: [vk::StencilOp; 3],
    compare: vk::CompareOp,
) {
    eprintln!(
        "{}",
        operations_json(command.as_raw(), faces, operations, compare)
    );
}
fn operations_json(
    command: u64,
    faces: vk::StencilFaceFlags,
    operations: [vk::StencilOp; 3],
    compare: vk::CompareOp,
) -> String {
    let operands = operations.map(vk::StencilOp::as_raw);
    format!(
        "{{\"kind\":\"command-stencil-op\",\"command\":{command},\"faces\":{},\"fail_pass_depth_fail\":{operands:?},\"compare\":{}}}",
        faces.as_raw(),
        compare.as_raw()
    )
}
pub(super) fn masks(
    command: vk::CommandBuffer,
    faces: vk::StencilFaceFlags,
    state: RenderStencilState,
) {
    eprintln!("{}", masks_json(command.as_raw(), faces, state));
}
fn masks_json(command: u64, faces: vk::StencilFaceFlags, state: RenderStencilState) -> String {
    format!(
        "{{\"kind\":\"command-stencil-masks-reference\",\"command\":{command},\"faces\":{},\"compare_mask\":{},\"write_mask\":{},\"reference\":{}}}",
        faces.as_raw(),
        state.compare_mask,
        state.write_mask,
        state.reference
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disabled_stencil_is_observed_instead_of_assumed() {
        assert_eq!(
            enabled_json(17, false),
            "{\"kind\":\"command-stencil-enable\",\"command\":17,\"enabled\":false}"
        );
    }
    #[test]
    fn operation_operands_preserve_distinct_fail_pass_depth_fail_order() {
        let values = [
            vk::StencilOp::INVERT,
            vk::StencilOp::REPLACE,
            vk::StencilOp::ZERO,
        ];
        let value = operations_json(
            19,
            vk::StencilFaceFlags::BACK,
            values,
            vk::CompareOp::NOT_EQUAL,
        );
        assert!(value.contains("\"faces\":2"));
        assert!(value.contains("\"fail_pass_depth_fail\":[5, 2, 1]"));
        assert!(value.contains("\"compare\":5"));
    }
    #[test]
    fn all_mask_bits_and_zero_reference_remain_unsigned_and_independent() {
        let mut state = RenderStencilState::default();
        state.compare_mask = u32::MAX;
        state.write_mask = 0x8000_0000;
        let value = masks_json(23, vk::StencilFaceFlags::FRONT_AND_BACK, state);
        assert!(value.contains("\"compare_mask\":4294967295"));
        assert!(value.contains("\"write_mask\":2147483648"));
        assert!(value.contains("\"reference\":0"));
    }
}
