//! Read `RenderDeviceLimits` from the retained physical-device properties and features.
use crate::RenderDeviceLimits;
use ash::vk;
use martensite_tkservice::graphic3d::Graphic3dTypeOfLimit;

impl super::VulkanRenderer {
    /// Snapshot the selected device capacities; `renderer.device_limits()`.
    pub(crate) fn device_limits(&self) -> RenderDeviceLimits {
        let limits = &self.runtime.limits;
        let dump = self.capture_limit() as i32;
        RenderDeviceLimits {
            max_texture_size: self.raster_limit(Graphic3dTypeOfLimit::MaxTextureSize),
            max_dump_size: [dump, dump],
            max_msaa_samples: self.raster_limit(Graphic3dTypeOfLimit::MaxMsaa),
            max_combined_texture_units: combined_samplers(limits),
            max_clip_planes: signed(limits.max_clip_distances),
            max_color_attachments: signed(limits.max_color_attachments),
            max_draw_buffers: signed(limits.max_fragment_output_attachments),
            max_degree_of_anisotropy: anisotropy(self.runtime.sampler_anisotropy, limits),
            has_srgb: self.srgb_supported(),
            has_texture_multisampling: limits.sampled_image_color_sample_counts.as_raw() > 1,
            has_mesh_edges: self.runtime.geometry_shader,
            has_blended_oit: self.raster_limit(Graphic3dTypeOfLimit::HasBlendedOit) == 1,
            has_blended_oit_msaa: self.raster_limit(Graphic3dTypeOfLimit::HasBlendedOitMsaa) == 1,
            sub_texel_precision_bits: limits.sub_texel_precision_bits,
            has_ray_tracing_textures: self.runtime.raytrace_textures,
            has_ray_tracing_adaptive_sampling: self.runtime.fragment_stores,
            has_ray_tracing_adaptive_sampling_atomic: self.runtime.image_float_atomic_add,
        }
    }
    /// Native HasSRGB needs sRGB textures and sRGB framebuffers: sample, filter and render.
    fn srgb_supported(&self) -> bool {
        // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
        let features = unsafe {
            self.runtime.instance.get_physical_device_format_properties(
                self.runtime.physical,
                vk::Format::R8G8B8A8_SRGB,
            )
        }
        .optimal_tiling_features;
        features.contains(srgb_required_features())
    }
}
fn srgb_required_features() -> vk::FormatFeatureFlags {
    vk::FormatFeatureFlags::SAMPLED_IMAGE
        | vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR
        | vk::FormatFeatureFlags::COLOR_ATTACHMENT
        | vk::FormatFeatureFlags::COLOR_ATTACHMENT_BLEND
}
/// Combined image samplers count against both the sampler and the sampled-image budgets.
fn combined_samplers(limits: &vk::PhysicalDeviceLimits) -> i32 {
    signed(
        limits
            .max_descriptor_set_samplers
            .min(limits.max_descriptor_set_sampled_images),
    )
}
fn anisotropy(enabled: bool, limits: &vk::PhysicalDeviceLimits) -> i32 {
    if !enabled {
        return 1;
    }
    // GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT is read as an integer, truncating the float limit.
    (limits.max_sampler_anisotropy as i32).max(1)
}
/// Vulkan limits are unsigned; the legacy query is signed, so saturate instead of wrapping.
fn signed(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combined_units_take_the_smaller_descriptor_budget() {
        let limits = vk::PhysicalDeviceLimits {
            max_descriptor_set_samplers: 96,
            max_descriptor_set_sampled_images: 160,
            ..Default::default()
        };
        assert_eq!(combined_samplers(&limits), 96);
    }
    #[test]
    fn anisotropy_truncates_and_requires_the_feature() {
        let limits = vk::PhysicalDeviceLimits {
            max_sampler_anisotropy: 16.5,
            ..Default::default()
        };
        assert_eq!(anisotropy(true, &limits), 16);
        assert_eq!(anisotropy(false, &limits), 1);
    }
    #[test]
    fn oversized_unsigned_limits_saturate() {
        assert_eq!(signed(u32::MAX), i32::MAX);
        assert_eq!(signed(8), 8);
    }
}
