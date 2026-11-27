//! Set-4 descriptor layout, samplers, and set writes of the OpenPBR environment & LUT bundle.
//!
//! Bindings (shared across fragment rasterization and compute path tracing):
//! - 0: `openpbrGgxLut` (2D `RG16F`, bilinear)
//! - 1: `pbrDiffuseSH` (2D 9×1 `RGBA32F`, nearest)
//! - 2: `pbrSpecularMap` (Cube `RGBA16F` HDR, trilinear)
//! - 3: `openpbrDielectricLut` (3D `RG16F`, bilinear)
//! - 4: `openpbrSheenLut` (2D `R16F`, bilinear)
//! - 5: `openpbrThinFilmLut` (2D `RGBA16F`, bilinear)
use super::super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

pub(super) const ENVIRONMENT_BINDING_COUNT: usize = 6;

/// Six combined image samplers for OpenPBR IBL + LUTs (fragment & compute stages).
pub(in super::super) struct PbrEnvironmentLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
}

impl PbrEnvironmentLayout {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let stages = vk::ShaderStageFlags::FRAGMENT | vk::ShaderStageFlags::COMPUTE;
        let bindings = [0u32, 1, 2, 3, 4, 5].map(|binding| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(binding)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(stages)
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

pub(super) fn create_samplers(
    runtime: &VulkanRuntime,
    spec_levels: u32,
) -> Result<[vk::Sampler; ENVIRONMENT_BINDING_COUNT], RenderError> {
    let linear = sampler_info(vk::Filter::LINEAR, vk::SamplerMipmapMode::NEAREST, 0.);
    let nearest = sampler_info(vk::Filter::NEAREST, vk::SamplerMipmapMode::NEAREST, 0.);
    let trilinear = sampler_info(
        vk::Filter::LINEAR,
        vk::SamplerMipmapMode::LINEAR,
        (spec_levels.saturating_sub(1)) as f32,
    )
    .flags(super::super::cube_seams::cube_sampler_flags(runtime));
    let infos = [linear, nearest, trilinear, linear, linear, linear];
    let mut samplers = [vk::Sampler::null(); ENVIRONMENT_BINDING_COUNT];
    for (slot, info) in samplers.iter_mut().zip(infos) {
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

pub(super) fn create_pool(runtime: &VulkanRuntime) -> Result<vk::DescriptorPool, RenderError> {
    let sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: ENVIRONMENT_BINDING_COUNT as u32,
    }];
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&sizes);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_pool(&info, None) }
        .map_err(|e| vk_error("create PBR environment pool", e))
}

pub(super) fn write_set(
    runtime: &VulkanRuntime,
    set: vk::DescriptorSet,
    views: [vk::ImageView; ENVIRONMENT_BINDING_COUNT],
    samplers: [vk::Sampler; ENVIRONMENT_BINDING_COUNT],
) {
    let images = std::array::from_fn::<_, ENVIRONMENT_BINDING_COUNT, _>(|index| {
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
