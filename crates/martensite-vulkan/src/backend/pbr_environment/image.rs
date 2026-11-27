//! Device-local images of the OpenPBR environment (IBL maps and multi-scattering / spectral LUTs).
use super::super::runtime::{VulkanRuntime, vk_error};
use crate::openpbr_luts::{DIELECTRIC_LUT_SIZE, GGX_LUT_SIZE, SHEEN_LUT_SIZE, THIN_FILM_LUT_SIZE};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;

/// Shape of one environment image; `EnvironmentImageSpec::ggx_lut()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) struct EnvironmentImageSpec {
    pub format: vk::Format,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub levels: u32,
    pub cube: bool,
    pub usage: vk::ImageUsageFlags,
}

impl EnvironmentImageSpec {
    /// 64×64 `RG16F` GGX directional & hemispherical average albedo LUT (binding 0).
    pub fn ggx_lut() -> Self {
        Self::flat(vk::Format::R16G16_SFLOAT, GGX_LUT_SIZE, GGX_LUT_SIZE)
    }
    /// Nine `RGBA32F` spherical-harmonics irradiance coefficients (binding 1).
    pub fn diffuse_sh() -> Self {
        Self::flat(vk::Format::R32G32B32A32_SFLOAT, 9, 1).rendered()
    }
    /// Prefiltered HDR `RGBA16F` specular cube map with one mip level per roughness step (binding 2).
    pub fn specular(pow2_size: u32, levels: u32) -> Self {
        Self {
            format: vk::Format::R16G16B16A16_SFLOAT,
            width: 1 << pow2_size,
            height: 1 << pow2_size,
            depth: 1,
            levels,
            cube: true,
            usage: sampled_upload(),
        }
        .rendered()
    }
    /// 32×32×32 `RG16F` 3D multi-scatter dielectric reflectance/transmittance LUT (binding 3).
    pub fn dielectric_lut() -> Self {
        Self {
            format: vk::Format::R16G16_SFLOAT,
            width: DIELECTRIC_LUT_SIZE,
            height: DIELECTRIC_LUT_SIZE,
            depth: DIELECTRIC_LUT_SIZE,
            levels: 1,
            cube: false,
            usage: sampled_upload(),
        }
    }
    /// 32×32 `R16F` Charlie microfiber sheen directional albedo LUT (binding 4).
    pub fn sheen_lut() -> Self {
        Self::flat(vk::Format::R16_SFLOAT, SHEEN_LUT_SIZE, SHEEN_LUT_SIZE)
    }
    /// 64×1 `RGBA16F` CIE XYZ Airy thin-film Fourier sensitivity LUT (binding 5).
    pub fn thin_film_lut() -> Self {
        Self::flat(vk::Format::R16G16B16A16_SFLOAT, THIN_FILM_LUT_SIZE, 1)
    }
    /// A cube map of `size` texels with `levels` levels and the given usage.
    pub fn cube(format: vk::Format, size: u32, levels: u32, usage: vk::ImageUsageFlags) -> Self {
        Self {
            format,
            width: size,
            height: size,
            depth: 1,
            levels,
            cube: true,
            usage,
        }
    }
    fn rendered(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::COLOR_ATTACHMENT;
        self
    }
    fn flat(format: vk::Format, width: u32, height: u32) -> Self {
        Self {
            format,
            width,
            height,
            depth: 1,
            levels: 1,
            cube: false,
            usage: sampled_upload(),
        }
    }
    pub fn layers(&self) -> u32 {
        if self.cube { 6 } else { 1 }
    }
    pub fn is_3d(&self) -> bool {
        self.depth > 1
    }
    pub fn range(&self) -> vk::ImageSubresourceRange {
        vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(self.levels)
            .layer_count(self.layers())
    }
}

fn sampled_upload() -> vk::ImageUsageFlags {
    vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST
}

/// One owned image and its full view; dropped after every frame that sampled it.
pub(in super::super) struct EnvironmentImage {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Image,
    pub view: vk::ImageView,
    pub spec: EnvironmentImageSpec,
    memory: Option<super::super::memory_arena::MemoryLease>,
    allocation: u64,
}

impl EnvironmentImage {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        spec: EnvironmentImageSpec,
    ) -> Result<Self, RenderError> {
        require_format(runtime, spec)?;
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_image(&image_info(spec), None) }
            .map_err(|e| vk_error("create PBR environment image", e))?;
        let mut image = Self {
            runtime: runtime.clone(),
            handle,
            view: vk::ImageView::null(),
            spec,
            memory: None,
            allocation: 0,
        };
        let lease = super::super::memory_arena::bind_device_image(runtime, handle)?;
        image.allocation = lease.size();
        runtime.image_memory.allocate_sampled(image.allocation);
        image.memory = Some(lease);
        image.view = image.create_view()?;
        Ok(image)
    }
    fn create_view(&self) -> Result<vk::ImageView, RenderError> {
        let view_type = if self.spec.cube {
            vk::ImageViewType::CUBE
        } else if self.spec.is_3d() {
            vk::ImageViewType::TYPE_3D
        } else {
            vk::ImageViewType::TYPE_2D
        };
        let info = vk::ImageViewCreateInfo::default()
            .image(self.handle)
            .view_type(view_type)
            .format(self.spec.format)
            .subresource_range(self.spec.range());
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().create_image_view(&info, None) }
            .map_err(|e| vk_error("create PBR environment view", e))
    }
    pub fn face_view(&self, level: u32, layer: u32) -> Result<FaceView, RenderError> {
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(level)
            .level_count(1)
            .base_array_layer(layer)
            .layer_count(1);
        let info = vk::ImageViewCreateInfo::default()
            .image(self.handle)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(self.spec.format)
            .subresource_range(range);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { self.runtime.device().create_image_view(&info, None) }
            .map_err(|e| vk_error("create environment face view", e))?;
        Ok(FaceView {
            runtime: self.runtime.clone(),
            handle,
            extent: (self.spec.width >> level).max(1),
        })
    }
}

/// An owned single-face view and its square extent.
pub(in super::super) struct FaceView {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::ImageView,
    pub extent: u32,
}
impl Drop for FaceView {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe { self.runtime.device().destroy_image_view(self.handle, None) };
    }
}
impl Drop for EnvironmentImage {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            if self.view != vk::ImageView::null() {
                device.destroy_image_view(self.view, None);
            }
            device.destroy_image(self.handle, None);
        }
        self.runtime.image_memory.release_sampled(self.allocation);
    }
}

fn image_info(spec: EnvironmentImageSpec) -> vk::ImageCreateInfo<'static> {
    vk::ImageCreateInfo::default()
        .flags(if spec.cube {
            vk::ImageCreateFlags::CUBE_COMPATIBLE
        } else {
            vk::ImageCreateFlags::empty()
        })
        .image_type(if spec.is_3d() {
            vk::ImageType::TYPE_3D
        } else {
            vk::ImageType::TYPE_2D
        })
        .format(spec.format)
        .extent(vk::Extent3D {
            width: spec.width,
            height: spec.height,
            depth: spec.depth,
        })
        .mip_levels(spec.levels)
        .array_layers(spec.layers())
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(spec.usage)
}

fn require_format(runtime: &VulkanRuntime, spec: EnvironmentImageSpec) -> Result<(), RenderError> {
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let features = unsafe {
        runtime
            .instance
            .get_physical_device_format_properties(runtime.physical, spec.format)
    }
    .optimal_tiling_features;
    let required = required_features(spec.usage, spec.format);
    if !features.contains(required) {
        return Err(rejected(
            (spec.format, features),
            &format!("an environment image format with {required:?}"),
        ));
    }
    Ok(())
}

fn required_features(usage: vk::ImageUsageFlags, format: vk::Format) -> vk::FormatFeatureFlags {
    let mut required = vk::FormatFeatureFlags::SAMPLED_IMAGE | vk::FormatFeatureFlags::TRANSFER_DST;
    // DiffuseSH uses NEAREST sampling (RGBA32F linear filtering is optional on some GPUs).
    if format != vk::Format::R32G32B32A32_SFLOAT {
        required |= vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR;
    }
    if usage.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT) {
        required |= vk::FormatFeatureFlags::COLOR_ATTACHMENT;
    }
    if usage.contains(vk::ImageUsageFlags::TRANSFER_SRC) {
        required |= vk::FormatFeatureFlags::BLIT_SRC | vk::FormatFeatureFlags::BLIT_DST;
    }
    required
}
