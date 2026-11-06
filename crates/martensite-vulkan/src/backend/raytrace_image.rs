//! Device images of the ray-tracing passes: the float colour and depth targets of the smoothing
//! chain (GlView's myRaytraceFBO1/2) and the black environment map.
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

/// One single-level 2D image with its view and memory.
pub(super) struct RaytraceImage {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Image,
    pub view: vk::ImageView,
    pub aspect: vk::ImageAspectFlags,
    memory: Option<super::memory_arena::MemoryLease>,
    allocation: u64,
}
impl RaytraceImage {
    /// Allocate `extent` of `format` for `usage`; `RaytraceImage::new(rt, extent, format, usage)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        extent: vk::Extent2D,
        format: vk::Format,
        usage: vk::ImageUsageFlags,
    ) -> Result<Self, RenderError> {
        let info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage);
        let device = runtime.device();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { device.create_image(&info, None) }
            .map_err(|e| vk_error("create ray-tracing image", e))?;
        let aspect = if usage.contains(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT) {
            vk::ImageAspectFlags::DEPTH
        } else {
            vk::ImageAspectFlags::COLOR
        };
        let mut image = Self {
            runtime: runtime.clone(),
            handle,
            view: vk::ImageView::null(),
            aspect,
            memory: None,
            allocation: 0,
        };
        let lease = super::memory_arena::bind_device_image(runtime, handle)?;
        image.allocation = lease.size();
        runtime.image_memory.allocate_sampled(image.allocation);
        image.memory = Some(lease);
        let view = vk::ImageViewCreateInfo::default()
            .image(handle)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(image.range());
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        image.view = unsafe { device.create_image_view(&view, None) }
            .map_err(|e| vk_error("create ray-tracing image view", e))?;
        Ok(image)
    }
    /// The whole image; `image.range()`.
    pub fn range(&self) -> vk::ImageSubresourceRange {
        vk::ImageSubresourceRange::default()
            .aspect_mask(self.aspect)
            .level_count(1)
            .layer_count(1)
    }
    /// A layout transition of the whole image with the given scopes.
    pub fn barrier(
        &self,
        (old, new): (vk::ImageLayout, vk::ImageLayout),
        source: (vk::PipelineStageFlags2, vk::AccessFlags2),
        target: (vk::PipelineStageFlags2, vk::AccessFlags2),
    ) -> vk::ImageMemoryBarrier2<'static> {
        vk::ImageMemoryBarrier2::default()
            .image(self.handle)
            .subresource_range(self.range())
            .old_layout(old)
            .new_layout(new)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .src_stage_mask(source.0)
            .src_access_mask(source.1)
            .dst_stage_mask(target.0)
            .dst_access_mask(target.1)
    }
}
impl Drop for RaytraceImage {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_image_view(self.view, None);
            device.destroy_image(self.handle, None);
        }
        self.runtime.image_memory.release_sampled(self.allocation);
    }
}
