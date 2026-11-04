//! Per-face cube map filtering. OpenGL filters each cube side on its own (clamped to the side's
//! edge) unless `GL_TEXTURE_CUBE_MAP_SEAMLESS` is enabled, and Renderer never enables it: the
//! skydome, the background cube maps, the environment sampled by the PBR bake and the baked
//! specular map all filter per face. Vulkan filters across sides unless the sampler carries
//! `VK_SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT` (`VK_EXT_non_seamless_cube_map`); on a
//! device without the extension, cube samplers stay seamless and bilinear texels next to a side
//! edge differ from the native image.
use super::runtime::VulkanRuntime;
use ash::{Instance, vk};

/// Whether `physical` offers `nonSeamlessCubeMap`; enabled at device creation when it does.
/// Example: `let per_face = non_seamless_cube_supported(&instance, physical);`.
pub(super) fn non_seamless_cube_supported(
    instance: &Instance,
    physical: vk::PhysicalDevice,
) -> bool {
    if !super::depth_equivalence::device_lists_extension(
        instance,
        physical,
        ash::ext::non_seamless_cube_map::NAME,
    ) {
        return false;
    }
    let mut feature = vk::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut feature);
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe { instance.get_physical_device_features2(physical, &mut query) };
    feature.non_seamless_cube_map == vk::TRUE
}

/// The creation flags of every cube map sampler: per-face filtering where the device has it.
/// Example: `vk::SamplerCreateInfo::default().flags(cube_sampler_flags(&runtime))`.
pub(super) fn cube_sampler_flags(runtime: &VulkanRuntime) -> vk::SamplerCreateFlags {
    per_face_flags(runtime.non_seamless_cube)
}

fn per_face_flags(non_seamless: bool) -> vk::SamplerCreateFlags {
    if non_seamless {
        vk::SamplerCreateFlags::NON_SEAMLESS_CUBE_MAP_EXT
    } else {
        vk::SamplerCreateFlags::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_samplers_filter_per_face_only_with_the_extension() {
        assert_eq!(
            per_face_flags(true),
            vk::SamplerCreateFlags::NON_SEAMLESS_CUBE_MAP_EXT
        );
        assert_eq!(per_face_flags(false), vk::SamplerCreateFlags::empty());
    }
}
