//! V3 consumes the same resolved ordered bias state as the legacy dynamic command.
use super::{pipeline_variants::PipelineVariant, runtime::VulkanRuntime};
use ash::vk;
pub(super) fn record(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    kind: PipelineVariant,
    units: f32,
    factor: f32,
    enabled: bool,
) {
    if !kind
        .window_depth
        .is_some_and(|parameters| parameters.fragment_depth())
    {
        return;
    }
    let values = bias_words(units, factor, enabled);
    let bytes: Vec<_> = values.into_iter().flat_map(f32::to_ne_bytes).collect();
    // SAFETY: V3 admission/layout guarantee this disjoint 16-byte geometry/fragment range.
    unsafe {
        runtime
            .device()
            .cmd_push_constants(command, layout, bias_stages(), 128, &bytes)
    };
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_command_diagnostic::push(command, layout, bias_stages(), 128, &bytes);
}
pub(super) fn bias_stages() -> vk::ShaderStageFlags {
    vk::ShaderStageFlags::GEOMETRY | vk::ShaderStageFlags::FRAGMENT
}
pub(super) fn push_range() -> vk::PushConstantRange {
    vk::PushConstantRange::default()
        .stage_flags(bias_stages())
        .offset(128)
        .size(16)
}

fn bias_words(units: f32, factor: f32, enabled: bool) -> [f32; 4] {
    [
        units,
        factor,
        if enabled { 1.0 } else { 0.0 },
        1.0 / 8_388_608.0,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_effective_enable_sign_and_units_are_not_inferred_from_draw_aspect() {
        assert_eq!(bias_words(-2., 3., true), [-2., 3., 1., 1.0 / 8_388_608.0]);
        assert_eq!(bias_words(7., -4., false), [7., -4., 0., 1.0 / 8_388_608.0]);
        assert_eq!(
            bias_words(f32::MAX, f32::MAX, false),
            [f32::MAX, f32::MAX, 0., 1.0 / 8_388_608.0]
        );
    }
    #[test]
    fn validation_and_evaluation_receive_the_same_disjoint_effective_bias_push() {
        let range = push_range();
        assert_eq!(range.offset, 128);
        assert_eq!(range.size, 16);
        assert_eq!(
            range.stage_flags,
            vk::ShaderStageFlags::GEOMETRY | vk::ShaderStageFlags::FRAGMENT
        );
    }
}
