//! `VK_EXT_shader_atomic_float` image add: the single-pass adaptive sampling variant
//! (ADAPTIVE_SAMPLING_ATOMIC) accumulates path-traced samples with `imageAtomicAdd` on R32F,
//! as native requires GL_NV_shader_atomic_float. lavapipe offers it; RADV does not.
use ash::{Instance, vk};

/// Whether the device offers `shaderImageFloat32AtomicAdd`.
/// Example: `image_float_atomic_add_supported(&instance, physical)`.
pub(super) fn image_float_atomic_add_supported(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> bool {
    if !super::depth_equivalence::device_lists_extension(
        instance,
        physical,
        ash::ext::shader_atomic_float::NAME,
    ) {
        return false;
    }
    let mut feature = vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut feature);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    feature.shader_image_float32_atomic_add == vk::TRUE
}

/// The feature node enabling only the image add when `enabled`.
pub(super) fn image_float_atomic_add_feature(
    enabled: bool,
) -> vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT<'static> {
    vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT::default()
        .shader_image_float32_atomic_add(enabled)
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_feature_node_requests_only_the_image_add() {
        let feature = super::image_float_atomic_add_feature(true);
        assert_eq!(feature.shader_image_float32_atomic_add, ash::vk::TRUE);
        assert_eq!(feature.shader_buffer_float32_atomic_add, ash::vk::FALSE);
        let disabled = super::image_float_atomic_add_feature(false);
        assert_eq!(disabled.shader_image_float32_atomic_add, ash::vk::FALSE);
    }
}
