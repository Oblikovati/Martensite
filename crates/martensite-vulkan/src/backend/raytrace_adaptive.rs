//! The adaptive-sampling images of the view (initRaytraceResources and
//! bindRaytraceTextures): the R32F output image of 3Wx2H texels (colour, sample count, depth
//! and the even-sample luminance of each pixel), the R32I visual error per tile, and the R32I
//! tile samples (multi-pass) or the RG32I tile offsets (atomic variant). They live in GENERAL
//! layout between frames, as storage images.
use super::raytrace_image::RaytraceImage;
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use crate::raytrace::AdaptivePasses;
use ash::vk;
use std::sync::Arc;

/// The image sizes the targets were made for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct AdaptiveTargetsKey {
    pub window: [i32; 2],
    pub tiles: [i32; 2],
    /// The tile-offsets image covers the larger offset map (NbOffsetTilesMax).
    pub offset_tiles_max: [i32; 2],
    pub atomic: bool,
}
impl AdaptiveTargetsKey {
    /// The targets `passes` of an `atomic` frame use; `AdaptiveTargetsKey::of(&passes, false)`.
    pub fn of(passes: &AdaptivePasses, atomic: bool) -> Self {
        Self {
            window: passes.window,
            tiles: passes.tiles,
            offset_tiles_max: passes.offset_tiles_max,
            atomic,
        }
    }
    fn tiles_image(&self) -> (vk::Extent2D, vk::Format) {
        match self.atomic {
            true => (extent(self.offset_tiles_max), vk::Format::R32G32_SINT),
            false => (extent(self.tiles), vk::Format::R32_SINT),
        }
    }
}

fn extent(size: [i32; 2]) -> vk::Extent2D {
    vk::Extent2D {
        width: size[0].max(1) as u32,
        height: size[1].max(1) as u32,
    }
}

/// A set layout of two fragment storage images at bindings 0 and 1: `uRenderImage` with
/// `uTilesImage`/`uOffsetImage` (program 0) or `uVarianceImage` (output program).
pub(super) fn storage_layout(
    runtime: &VulkanRuntime,
) -> Result<vk::DescriptorSetLayout, RenderError> {
    let bindings: Vec<_> = (0..2)
        .map(|binding| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(binding)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        })
        .collect();
    let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
        .map_err(|e| vk_error("create adaptive-sampling layout", e))
}

/// The persistent images and the sets of program 0 and the output program.
pub(super) struct AdaptiveTargets {
    runtime: Arc<VulkanRuntime>,
    /// myRaytraceOutputTexture.
    pub render: RaytraceImage,
    /// myRaytraceVisualErrorTexture.
    pub variance: RaytraceImage,
    /// myRaytraceTileSamplesTexture, or myRaytraceTileOffsetsTexture of the atomic variant.
    pub tiles: RaytraceImage,
    layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    /// Set 1 of program 0 and set 0 of the output program.
    pub render_set: vk::DescriptorSet,
    pub display_set: vk::DescriptorSet,
    pub key: AdaptiveTargetsKey,
    /// Whether the images left UNDEFINED layout (the first frame clears them all).
    pub initialized: std::sync::atomic::AtomicBool,
}
impl AdaptiveTargets {
    /// Images for `key`; `AdaptiveTargets::new(&runtime, key)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>, key: AdaptiveTargetsKey) -> Result<Self, RenderError> {
        let usage = vk::ImageUsageFlags::STORAGE
            | vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::TRANSFER_SRC;
        let image = |size, format| RaytraceImage::new(runtime, size, format, usage);
        let (tiles_size, tiles_format) = key.tiles_image();
        let mut targets = Self {
            runtime: runtime.clone(),
            render: image(
                extent([3 * key.window[0], 2 * key.window[1]]),
                vk::Format::R32_SFLOAT,
            )?,
            variance: image(extent(key.tiles), vk::Format::R32_SINT)?,
            tiles: image(tiles_size, tiles_format)?,
            layout: storage_layout(runtime)?,
            pool: vk::DescriptorPool::null(),
            render_set: vk::DescriptorSet::null(),
            display_set: vk::DescriptorSet::null(),
            key,
            initialized: Default::default(),
        };
        targets.allocate_sets()?;
        Ok(targets)
    }
    /// The images in recording order: output, visual error, tiles.
    pub fn images(&self) -> [&RaytraceImage; 3] {
        [&self.render, &self.variance, &self.tiles]
    }
    fn allocate_sets(&mut self) -> Result<(), RenderError> {
        let sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_IMAGE,
            descriptor_count: 4,
        }];
        let info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(2)
            .pool_sizes(&sizes);
        let device = self.runtime.device();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.pool = unsafe { device.create_descriptor_pool(&info, None) }
            .map_err(|e| vk_error("create adaptive-sampling pool", e))?;
        let layouts = [self.layout; 2];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let sets = unsafe { device.allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate adaptive-sampling sets", e))?;
        (self.render_set, self.display_set) = (sets[0], sets[1]);
        self.write_sets();
        Ok(())
    }
    fn write_sets(&self) {
        let image = |view| {
            [vk::DescriptorImageInfo::default()
                .image_view(view)
                .image_layout(vk::ImageLayout::GENERAL)]
        };
        let views = [
            (self.render_set, 0, image(self.render.view)),
            (self.render_set, 1, image(self.tiles.view)),
            (self.display_set, 0, image(self.render.view)),
            (self.display_set, 1, image(self.variance.view)),
        ];
        let writes: Vec<_> = views
            .iter()
            .map(|(set, binding, info)| {
                vk::WriteDescriptorSet::default()
                    .dst_set(*set)
                    .dst_binding(*binding)
                    .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                    .image_info(info)
            })
            .collect();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().update_descriptor_sets(&writes, &[]) };
    }
}
impl Drop for AdaptiveTargets {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(atomic: bool) -> AdaptiveTargetsKey {
        AdaptiveTargetsKey {
            window: [100, 70],
            tiles: [4, 3],
            offset_tiles_max: [16, 8],
            atomic,
        }
    }

    #[test]
    fn the_tiles_image_holds_samples_or_the_larger_offset_map() {
        let samples = key(false).tiles_image();
        assert_eq!((samples.0.width, samples.0.height), (4, 3));
        assert_eq!(samples.1, vk::Format::R32_SINT);
        let offsets = key(true).tiles_image();
        assert_eq!((offsets.0.width, offsets.0.height), (16, 8));
        assert_eq!(offsets.1, vk::Format::R32G32_SINT);
    }
}
