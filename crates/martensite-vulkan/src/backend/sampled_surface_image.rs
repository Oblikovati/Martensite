//! Device-local sampled images retain all mip levels until no frame/cache references them.
use super::{
    runtime::{VulkanRuntime, vk_error},
    surface_image_format::*,
};
use crate::{RenderError, error::rejected, surface_texture::SurfaceTextureImage};
use ash::vk;
use std::sync::Arc;
#[cfg(feature = "command-state-diagnostics")]
#[path = "replay_sampled_image_diagnostic.rs"]
pub(super) mod replay_diagnostic;
pub(super) struct SampledSurfaceImage {
    pub runtime: Arc<VulkanRuntime>,
    pub handle: vk::Image,
    pub format: vk::Format,
    pub levels: u32,
    pub extent: vk::Extent3D,
    /// A 3D image (`Graphic3dTypeOfTexture_3D`), viewed as `TYPE_3D` for a `sampler3D`.
    pub volume: bool,
    memory: Option<super::memory_arena::MemoryLease>,
    allocation: u64,
}
impl SampledSurfaceImage {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        source: &SurfaceTextureImage,
        format: vk::Format,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Arc<Self>, RenderError> {
        let image = Self::allocated(runtime, source, format)?;
        super::sampled_surface_upload::upload_sampled_image(&image, source, retirements)?;
        Ok(image)
    }
    /// Allocate the actual image before upload ownership; `Self::allocated(runtime, source, format)?` publishes nothing.
    pub(super) fn allocated(
        runtime: &Arc<VulkanRuntime>,
        source: &SurfaceTextureImage,
        format: vk::Format,
    ) -> Result<Arc<Self>, RenderError> {
        let levels = sampled_levels(source);
        validate_sampled_image(runtime, source, format, levels)?;
        let extent = sampled_surface_extent(source);
        let volume = source.depth.is_some();
        let handle = create_sampled_image(runtime, (extent, volume), format, levels)?;
        let mut image = Self {
            runtime: runtime.clone(),
            handle,
            format,
            levels,
            extent,
            volume,
            memory: None,
            allocation: 0,
        };
        image.allocate()?;
        Ok(Arc::new(image))
    }
    fn allocate(&mut self) -> Result<(), RenderError> {
        let lease = super::memory_arena::bind_device_image(&self.runtime, self.handle)?;
        // Frame statistics count the resource's own requirement, not the shared block.
        self.allocation = lease.size();
        self.runtime.image_memory.allocate_sampled(self.allocation);
        self.memory = Some(lease);
        Ok(())
    }
    pub fn view(&self, first: u32, count: u32) -> Result<vk::ImageView, RenderError> {
        let info = vk::ImageViewCreateInfo::default()
            .image(self.handle)
            .view_type(if self.volume {
                vk::ImageViewType::TYPE_3D
            } else {
                vk::ImageViewType::TYPE_2D
            })
            .format(self.format)
            .subresource_range(image_range(first, count));
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let view = unsafe { self.runtime.device().create_image_view(&info, None) }
            .map_err(|e| vk_error("create sampled image view", e))?;
        #[cfg(feature = "command-state-diagnostics")]
        replay_diagnostic::view(view, &info);
        Ok(view)
    }
}
impl Drop for SampledSurfaceImage {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            // The lease field drops after this body, once the image no longer uses its range.
            self.runtime.device().destroy_image(self.handle, None);
        }
        self.runtime.image_memory.release_sampled(self.allocation);
    }
}
fn create_sampled_image(
    runtime: &VulkanRuntime,
    (extent, volume): (vk::Extent3D, bool),
    format: vk::Format,
    levels: u32,
) -> Result<vk::Image, RenderError> {
    let kind = if volume {
        vk::ImageType::TYPE_3D
    } else {
        vk::ImageType::TYPE_2D
    };
    let info = vk::ImageCreateInfo::default()
        .image_type(kind)
        .format(format)
        .extent(extent)
        .mip_levels(levels)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(
            vk::ImageUsageFlags::SAMPLED
                | vk::ImageUsageFlags::TRANSFER_DST
                | vk::ImageUsageFlags::TRANSFER_SRC,
        );
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    let image = unsafe { runtime.device().create_image(&info, None) }
        .map_err(|e| vk_error("create sampled image", e))?;
    #[cfg(feature = "command-state-diagnostics")]
    replay_diagnostic::image(image, &info);
    Ok(image)
}
fn validate_sampled_image(
    runtime: &VulkanRuntime,
    source: &SurfaceTextureImage,
    format: vk::Format,
    levels: u32,
) -> Result<(), RenderError> {
    match source.depth {
        Some(depth) => validate_sampled_extent(
            &[source.extent[0], source.extent[1], depth],
            runtime.limits.max_image_dimension3_d,
        )?,
        None => validate_sampled_extent(&source.extent, runtime.limits.max_image_dimension2_d)?,
    }
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let support = unsafe {
        runtime
            .instance
            .get_physical_device_format_properties(runtime.physical, format)
    }
    .optimal_tiling_features;
    let required = required_sampled_features(source.supplied_levels.is_empty() && levels > 1);
    if !support.contains(required) {
        return Err(rejected(
            (format, support, required),
            "device support for sampled image and mip generation",
        ));
    }
    Ok(())
}

fn sampled_surface_extent(source: &SurfaceTextureImage) -> vk::Extent3D {
    vk::Extent3D {
        width: source.extent[0],
        height: source.extent[1],
        depth: source.depth.unwrap_or(1),
    }
}
fn validate_sampled_extent(extent: &[u32], maximum: u32) -> Result<(), RenderError> {
    if extent.contains(&0) || extent.iter().any(|value| *value > maximum) {
        return Err(rejected(
            (extent.to_vec(), maximum),
            "positive sampled dimensions within maxImageDimension2D (3D for a volume)",
        ));
    }
    Ok(())
}
fn required_sampled_features(generate_mips: bool) -> vk::FormatFeatureFlags {
    let mut features = vk::FormatFeatureFlags::SAMPLED_IMAGE
        | vk::FormatFeatureFlags::TRANSFER_DST
        | vk::FormatFeatureFlags::TRANSFER_SRC;
    if generate_mips {
        features |= vk::FormatFeatureFlags::BLIT_SRC
            | vk::FormatFeatureFlags::BLIT_DST
            | vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR;
    }
    features
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sampled_extent_and_mip_feature_requirements_are_explicit_before_allocating() {
        assert!(validate_sampled_extent(&[0, 4], 1024).is_err());
        assert!(validate_sampled_extent(&[4, 2048], 1024).is_err());
        assert!(validate_sampled_extent(&[4, 4], 1024).is_ok());
        assert!(validate_sampled_extent(&[4, 4, 0], 1024).is_err());
        assert!(validate_sampled_extent(&[4, 4, 6], 1024).is_ok());
        assert!(required_sampled_features(true).contains(
            vk::FormatFeatureFlags::BLIT_SRC | vk::FormatFeatureFlags::SAMPLED_IMAGE_FILTER_LINEAR
        ));
        assert!(!required_sampled_features(false).contains(vk::FormatFeatureFlags::BLIT_SRC));
    }
}
