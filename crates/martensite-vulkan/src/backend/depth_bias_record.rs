//! Native24 is a named equivalence policy, independent of physical depth format or exponent.
use super::runtime::VulkanRuntime;
use crate::RenderDepthBiasBehavior as Behavior;
use ash::vk;
const NATIVE24_UNIT: f32 = 1.0 / 8_388_608.0;
pub(super) fn record(runtime: &VulkanRuntime, command: vk::CommandBuffer, units: f32, factor: f32) {
    let bias = parameters(runtime.depth_bias_behavior, units, factor);
    if runtime.depth_bias_behavior == Behavior::Native24UnitsV2 {
        record_exact(runtime, command, bias);
        return;
    }
    // SAFETY: recording owns the live command buffer and retains existing V1 behavior.
    unsafe {
        runtime
            .device()
            .cmd_set_depth_bias(command, bias[0], bias[1], bias[2])
    };
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_raster_commands::hardware_bias(command, bias, None);
}
fn record_exact(runtime: &VulkanRuntime, command: vk::CommandBuffer, bias: [f32; 3]) {
    let mut representation = exact_float_representation();
    let info = description(bias).push_next(&mut representation);
    let loader = runtime
        .depth_bias_command
        .as_ref()
        .expect("admitted Native24UnitsV2 command");
    // SAFETY: admission checked the live device command and all enabled feature bits.
    unsafe { (loader.fp().cmd_set_depth_bias2_ext)(command, &info) };
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_raster_commands::hardware_bias(
        command,
        bias,
        Some((
            representation.depth_bias_representation.as_raw(),
            representation.depth_bias_exact != 0,
        )),
    );
}
fn description(bias: [f32; 3]) -> vk::DepthBiasInfoEXT<'static> {
    vk::DepthBiasInfoEXT::default()
        .depth_bias_constant_factor(bias[0])
        .depth_bias_clamp(bias[1])
        .depth_bias_slope_factor(bias[2])
}
fn parameters(behavior: Behavior, units: f32, factor: f32) -> [f32; 3] {
    let constant = if behavior == Behavior::Native24UnitsV2 {
        units * NATIVE24_UNIT
    } else {
        units
    };
    [constant, 0., factor]
}
fn exact_float_representation() -> vk::DepthBiasRepresentationInfoEXT<'static> {
    vk::DepthBiasRepresentationInfoEXT::default()
        .depth_bias_representation(vk::DepthBiasRepresentationEXT::FLOAT)
        .depth_bias_exact(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absolute_units_keep_sign_slope_and_unbounded_clamp() {
        for units in [-2., -1., -0., 0., 1., 2.] {
            for factor in [-2., 0., 3.] {
                assert_eq!(
                    parameters(Behavior::HardwareFormatV1, units, factor),
                    [units, 0., factor]
                );
                assert_eq!(
                    parameters(Behavior::Native24UnitsV2, units, factor),
                    [units / 8_388_608., 0., factor]
                );
            }
        }
        assert!(parameters(Behavior::Native24UnitsV2, -0., 1.)[0].is_sign_negative());
    }
    #[test]
    fn fixed_bias_is_exact_float_not_format_or_force_unorm() {
        let info = description([-1., 0., 2.]);
        assert_eq!(
            [
                info.depth_bias_constant_factor,
                info.depth_bias_clamp,
                info.depth_bias_slope_factor
            ],
            [-1., 0., 2.]
        );
        let f = exact_float_representation();
        assert_eq!(
            f.depth_bias_representation,
            vk::DepthBiasRepresentationEXT::FLOAT
        );
        assert_eq!(f.depth_bias_exact, vk::TRUE);
    }
}
