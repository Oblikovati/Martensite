//! Source cube maps of the environment passes: an uploaded Graphic3dCubeMap (GlTexture::
//! InitCubeMap with generated mipmaps, as GlView::SetBackgroundImage requests) or the
//! skydome texture of GlView::updateSkydomeBg, each with the sampler of its native texture
//! and one descriptor set (set 0, binding 0) for the background and baking stages.
pub(super) mod background;
pub(super) mod skydome;
pub(super) mod upload;
use super::pbr_environment::image::{EnvironmentImage, EnvironmentImageSpec};
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use crate::environment_frame::CubeMapPixels;
use ash::vk;
use std::sync::Arc;

/// The single fragment cube-map sampler of the environment pipelines.
pub(super) struct CubeSamplerLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
}
impl CubeSamplerLayout {
    /// `CubeSamplerLayout::new(&runtime)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let bindings = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create environment cube layout", e))?;
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}
impl Drop for CubeSamplerLayout {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_set_layout(self.handle, None)
        };
    }
}

/// Native filtering of the source texture: Graphic3dTextureParams' default NEAREST filter with
/// the generated mipmaps (GL_NEAREST_MIPMAP_NEAREST), or the skydome's BILINEAR without mipmaps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CubeFilter {
    NearestMipmapped,
    Bilinear,
}

/// One source cube map, sampled by the environment passes.
pub(super) struct EnvironmentCube {
    runtime: Arc<VulkanRuntime>,
    pub image: EnvironmentImage,
    sampler: vk::Sampler,
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
}
impl EnvironmentCube {
    /// Upload six sides and generate the mipmap chain (or copy the supplied compressed levels); `EnvironmentCube::uploaded(rt, layout, &pixels)?`.
    pub fn uploaded(
        runtime: &Arc<VulkanRuntime>,
        layout: &CubeSamplerLayout,
        pixels: &CubeMapPixels,
    ) -> Result<Arc<Self>, RenderError> {
        let format = super::surface_image_format::sampled_format(pixels.encoding);
        // Compressed sides bring their own levels (InitCubeMap's myMaxMipLevel); others generate.
        let levels = match pixels.levels.len() {
            0 => u32::BITS - pixels.size.leading_zeros(),
            supplied => supplied as u32,
        };
        // Generated levels blit from the level above; supplied compressed levels are only copied.
        let blits = match pixels.levels.is_empty() {
            true => vk::ImageUsageFlags::TRANSFER_SRC,
            false => vk::ImageUsageFlags::empty(),
        };
        let usage = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST | blits;
        let spec = EnvironmentImageSpec::cube(format, pixels.size, levels, usage);
        let image = upload::upload_faces(runtime, EnvironmentImage::new(runtime, spec)?, pixels)?;
        Self::sampled(runtime, layout, image, CubeFilter::NearestMipmapped).map(Arc::new)
    }
    /// Wrap a filled image with its sampler and descriptor set.
    fn sampled(
        runtime: &Arc<VulkanRuntime>,
        layout: &CubeSamplerLayout,
        image: EnvironmentImage,
        filter: CubeFilter,
    ) -> Result<Self, RenderError> {
        let mut cube = Self {
            runtime: runtime.clone(),
            sampler: vk::Sampler::null(),
            pool: vk::DescriptorPool::null(),
            set: vk::DescriptorSet::null(),
            image,
        };
        let info = sampler_info(filter, cube.image.spec.levels)
            .flags(super::cube_seams::cube_sampler_flags(runtime));
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        cube.sampler = unsafe { runtime.device().create_sampler(&info, None) }
            .map_err(|e| vk_error("create environment cube sampler", e))?;
        cube.pool = create_pool(runtime)?;
        cube.set = cube.allocate_set(layout)?;
        Ok(cube)
    }
    fn allocate_set(&self, layout: &CubeSamplerLayout) -> Result<vk::DescriptorSet, RenderError> {
        let layouts = [layout.handle];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let set = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate environment cube set", e))?[0];
        let images = [vk::DescriptorImageInfo::default()
            .sampler(self.sampler)
            .image_view(self.image.view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let write = vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&images);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().update_descriptor_sets(&[write], &[]) };
        Ok(set)
    }
    /// The combined image sampler of the cube; `cube.image_info()`.
    pub fn image_info(&self) -> vk::DescriptorImageInfo {
        vk::DescriptorImageInfo::default()
            .sampler(self.sampler)
            .image_view(self.image.view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
    }
    /// Texels per side of level 0 (GlTexture::SizeX of the baking source).
    pub fn size(&self) -> u32 {
        self.image.spec.width
    }
}
impl Drop for EnvironmentCube {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            if self.pool != vk::DescriptorPool::null() {
                device.destroy_descriptor_pool(self.pool, None);
            }
            if self.sampler != vk::Sampler::null() {
                device.destroy_sampler(self.sampler, None);
            }
        }
    }
}

/// GlSampler::applySamplerParams for a cube map: clamped, NEAREST or LINEAR, mipmapped
/// over every generated level. Example: `sampler_info(CubeFilter::Bilinear, 1)`.
fn sampler_info(filter: CubeFilter, levels: u32) -> vk::SamplerCreateInfo<'static> {
    let (texel, max_lod) = match filter {
        CubeFilter::NearestMipmapped => (vk::Filter::NEAREST, (levels - 1) as f32),
        CubeFilter::Bilinear => (vk::Filter::LINEAR, 0.),
    };
    let clamp = vk::SamplerAddressMode::CLAMP_TO_EDGE;
    vk::SamplerCreateInfo::default()
        .mag_filter(texel)
        .min_filter(texel)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(clamp)
        .address_mode_v(clamp)
        .address_mode_w(clamp)
        .min_lod(0.)
        .max_lod(max_lod)
}

fn create_pool(runtime: &VulkanRuntime) -> Result<vk::DescriptorPool, RenderError> {
    let sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: 1,
    }];
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&sizes);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_pool(&info, None) }
        .map_err(|e| vk_error("create environment cube pool", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samplers_follow_the_native_texture_parameters() {
        let nearest = sampler_info(CubeFilter::NearestMipmapped, 10);
        assert_eq!(nearest.min_filter, vk::Filter::NEAREST);
        assert_eq!(nearest.mag_filter, vk::Filter::NEAREST);
        assert_eq!(nearest.mipmap_mode, vk::SamplerMipmapMode::NEAREST);
        assert_eq!(nearest.max_lod, 9.);
        let bilinear = sampler_info(CubeFilter::Bilinear, 1);
        assert_eq!(bilinear.min_filter, vk::Filter::LINEAR);
        assert_eq!(bilinear.max_lod, 0.);
        assert_eq!(
            bilinear.address_mode_w,
            vk::SamplerAddressMode::CLAMP_TO_EDGE
        );
    }
}
