//! A one-set descriptor pool that samples one target image for the main restore draw.
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use crate::backend::VulkanWorkLedgerExt;
use ash::vk;
use std::sync::Arc;

/// Owns its pool; the set reads the image in `SHADER_READ_ONLY_OPTIMAL`.
pub(super) struct TargetSampleSet {
    runtime: Arc<VulkanRuntime>,
    pool: vk::DescriptorPool,
    pub handle: vk::DescriptorSet,
}

impl TargetSampleSet {
    /// Allocate a set of `layout` and write `view` into binding 0;
    /// `TargetSampleSet::new(runtime, layout, image.view)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        layout: vk::DescriptorSetLayout,
        view: vk::ImageView,
    ) -> Result<Self, RenderError> {
        let sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::SAMPLED_IMAGE,
            descriptor_count: 1,
        }];
        let info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(1)
            .pool_sizes(&sizes);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let pool = unsafe { runtime.device().create_descriptor_pool(&info, None) }
            .map_err(|e| vk_error("create target sample pool", e))?;
        let mut set = Self {
            runtime: Arc::clone(runtime),
            pool,
            handle: vk::DescriptorSet::null(),
        };
        set.handle = set.allocate(layout)?;
        set.write(view);
        Ok(set)
    }
    fn allocate(&self, layout: vk::DescriptorSetLayout) -> Result<vk::DescriptorSet, RenderError> {
        let layouts = [layout];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the pool and layout are live and the pool has room for exactly this set.
        let sets = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate target sample set", e))?;
        self.runtime.work.add_sets(&info);
        sets.first()
            .copied()
            .ok_or_else(|| crate::error::rejected(sets.len(), "one allocated target sample set"))
    }
    fn write(&self, view: vk::ImageView) {
        let images = [sampled_image(view)];
        let writes = [vk::WriteDescriptorSet::default()
            .dst_set(self.handle)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .image_info(&images)];
        // SAFETY: the set is not bound by any command buffer yet and the view is live.
        unsafe { self.runtime.device().update_descriptor_sets(&writes, &[]) };
    }
}

impl Drop for TargetSampleSet {
    fn drop(&mut self) {
        // SAFETY: submitting frames retain the descriptor and sampled image until GPU completion.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.pool, None)
        };
    }
}

fn sampled_image(view: vk::ImageView) -> vk::DescriptorImageInfo {
    vk::DescriptorImageInfo::default()
        .image_view(view)
        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ash::vk::Handle;
    #[test]
    fn a_target_is_sampled_in_the_read_only_layout_without_a_sampler() {
        let info = sampled_image(vk::ImageView::from_raw(9));
        assert_eq!(info.image_layout, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        assert_eq!(info.sampler, vk::Sampler::null());
        assert_eq!(info.image_view.as_raw(), 9);
    }
}
