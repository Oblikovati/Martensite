//! Set-4 descriptor layout, samplers and set writes of the PBR environment.
use super::super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

/// Three fragment combined image samplers: EnvLUT, diffuse SH, specular cube map.
pub(in super::super) struct PbrEnvironmentLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
}
impl PbrEnvironmentLayout {
    /// Create one identically defined layout; `PbrEnvironmentLayout::new(&runtime)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let bindings = [0, 1, 2].map(|binding| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(binding)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        });
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create PBR environment layout", e))?;
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}
impl Drop for PbrEnvironmentLayout {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_set_layout(self.handle, None);
        }
    }
}
/// Native sampler states: EnvLUT Graphic3dTOTF_BILINEAR without repeat; DiffuseSH NEAREST;
/// Specular TRILINEAR with levels range `spec_levels - 1`. Example: `create_samplers(rt, 6)?`.
pub(super) fn create_samplers(
    runtime: &VulkanRuntime,
    spec_levels: u32,
) -> Result<[vk::Sampler; 3], RenderError> {
    let linear = sampler_info(vk::Filter::LINEAR, vk::SamplerMipmapMode::NEAREST, 0.);
    let nearest = sampler_info(vk::Filter::NEAREST, vk::SamplerMipmapMode::NEAREST, 0.);
    let trilinear = sampler_info(
        vk::Filter::LINEAR,
        vk::SamplerMipmapMode::LINEAR,
        (spec_levels - 1) as f32,
    )
    .flags(super::super::cube_seams::cube_sampler_flags(runtime));
    let mut samplers = [vk::Sampler::null(); 3];
    for (slot, info) in samplers.iter_mut().zip([linear, nearest, trilinear]) {
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        *slot = unsafe { runtime.device().create_sampler(&info, None) }
            .map_err(|e| vk_error("create PBR environment sampler", e))?;
    }
    Ok(samplers)
}
fn sampler_info(
    filter: vk::Filter,
    mipmap: vk::SamplerMipmapMode,
    max_lod: f32,
) -> vk::SamplerCreateInfo<'static> {
    let clamp = vk::SamplerAddressMode::CLAMP_TO_EDGE;
    vk::SamplerCreateInfo::default()
        .mag_filter(filter)
        .min_filter(filter)
        .mipmap_mode(mipmap)
        .address_mode_u(clamp)
        .address_mode_v(clamp)
        .address_mode_w(clamp)
        .min_lod(0.)
        .max_lod(max_lod)
}
/// One pool for the environment's single set; `create_pool(runtime)?`.
pub(super) fn create_pool(runtime: &VulkanRuntime) -> Result<vk::DescriptorPool, RenderError> {
    let sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: 3,
    }];
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&sizes);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_pool(&info, None) }
        .map_err(|e| vk_error("create PBR environment pool", e))
}
/// Bind the three views with their samplers; `write_set(runtime, set, views, samplers)`.
pub(super) fn write_set(
    runtime: &VulkanRuntime,
    set: vk::DescriptorSet,
    views: [vk::ImageView; 3],
    samplers: [vk::Sampler; 3],
) {
    let images = [0, 1, 2].map(|index| {
        [vk::DescriptorImageInfo::default()
            .sampler(samplers[index])
            .image_view(views[index])
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)]
    });
    let writes: Vec<_> = images
        .iter()
        .enumerate()
        .map(|(binding, info)| {
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(binding as u32)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(info)
        })
        .collect();
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().update_descriptor_sets(&writes, &[]) };
}
