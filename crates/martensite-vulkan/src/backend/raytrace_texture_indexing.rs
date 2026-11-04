//! Textures of ray-traced materials (GlContext::HasRayTracingTextures). Native samples them
//! through GL_ARB_bindless_texture handles; the Vulkan programs index a sampler array
//! (`uTextureSamplers[nonuniformEXT (i)]`), which needs `shaderSampledImageArrayNonUniformIndexing`.
//! The flag matches native per driver (user decision 2026-10-29): lavapipe reports 0 and renders
//! path-traced materials untextured, as llvmpipe (no bindless textures) does; RADV reports 1 like
//! radeonsi.
use ash::{Instance, vk};

/// Whether ray-traced materials may be textured on `physical`.
/// Example: `ray_tracing_textures_supported(&instance, physical)`.
pub(super) fn ray_tracing_textures_supported(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> bool {
    let mut driver = vk::PhysicalDeviceDriverProperties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut driver);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_properties2(physical, &mut properties) };
    let mut core12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut core12);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    textures_for_driver(
        driver.driver_id,
        core12.shader_sampled_image_array_non_uniform_indexing == vk::TRUE,
    )
}

/// The flag for a driver offering (or not) non-uniform sampler indexing: the Mesa software
/// rasterizer follows llvmpipe, which lacks bindless textures.
fn textures_for_driver(driver: vk::DriverId, non_uniform_indexing: bool) -> bool {
    non_uniform_indexing && driver != vk::DriverId::MESA_LLVMPIPE
}

/// The Vulkan 1.2 feature node enabling only non-uniform sampler indexing when `enabled`.
pub(super) fn ray_tracing_textures_feature(
    enabled: bool,
) -> vk::PhysicalDeviceVulkan12Features<'static> {
    vk::PhysicalDeviceVulkan12Features::default()
        .shader_sampled_image_array_non_uniform_indexing(enabled)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lavapipe_follows_llvmpipe_and_other_drivers_need_the_feature() {
        assert!(!textures_for_driver(vk::DriverId::MESA_LLVMPIPE, true));
        assert!(textures_for_driver(vk::DriverId::MESA_RADV, true));
        assert!(!textures_for_driver(vk::DriverId::MESA_RADV, false));
    }
    #[test]
    fn the_feature_node_requests_only_non_uniform_sampler_indexing() {
        let feature = ray_tracing_textures_feature(true);
        assert_eq!(
            feature.shader_sampled_image_array_non_uniform_indexing,
            vk::TRUE
        );
        assert_eq!(feature.runtime_descriptor_array, vk::FALSE);
        assert_eq!(feature.descriptor_indexing, vk::FALSE);
        let disabled = ray_tracing_textures_feature(false);
        assert_eq!(
            disabled.shader_sampled_image_array_non_uniform_indexing,
            vk::FALSE
        );
    }
}
