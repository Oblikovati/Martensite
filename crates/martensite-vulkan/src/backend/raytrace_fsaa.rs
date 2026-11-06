//! GlView::runRaytrace with FSAA: program 0 renders the image and its depth
//! into float target 1, three smoothing passes of program 1 ping-pong between targets 1 and 2
//! with the native sub-pixel offsets, and the output program (Display.fs) writes target 2 and the
//! depth of target 1 into the frame, depth-tested.
use super::raytrace_image::RaytraceImage;
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

/// The float colour of myRaytraceFBO1/2 and the depth attachment.
pub(super) const FSAA_COLOR: vk::Format = vk::Format::R32G32B32A32_SFLOAT;
pub(super) const FSAA_DEPTH: vk::Format = vk::Format::D32_SFLOAT;

/// A set layout of `count` fragment combined image samplers at bindings 0..count.
pub(super) fn sampler_layout(
    runtime: &VulkanRuntime,
    count: u32,
) -> Result<vk::DescriptorSetLayout, RenderError> {
    let bindings: Vec<_> = (0..count)
        .map(|binding| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(binding)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        })
        .collect();
    let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
        .map_err(|e| vk_error("create ray-tracing sampler layout", e))
}

/// The `uFsaaOffset` of smoothing pass `iteration` (1 to 3) for a `size` window.
/// Example: `fsaa_offset(1, [256, 256])`.
pub(super) fn fsaa_offset(iteration: u32, size: [i32; 2]) -> [f32; 2] {
    let unit = [1. / size[0] as f32, 1. / size[1] as f32];
    let factor = match iteration {
        1 => [-0.55, 0.55],
        2 => [0.00, -0.55],
        _ => [0.55, 0.00],
    };
    [unit[0] * factor[0], unit[1] * factor[1]]
}

/// The two float targets, the depth target and the sets that read them.
pub(super) struct FsaaTargets {
    runtime: Arc<VulkanRuntime>,
    pub color: [RaytraceImage; 2],
    pub depth: RaytraceImage,
    sampler: vk::Sampler,
    layouts: [vk::DescriptorSetLayout; 2],
    pool: vk::DescriptorPool,
    /// Sets reading target 1 and target 2 (`uFSAAInputTexture`).
    pub inputs: [vk::DescriptorSet; 2],
    /// The output sets: target 1 or 2 with the depth (`uInputTexture`, `uDepthTexture`).
    pub outputs: [vk::DescriptorSet; 2],
    /// The image size the targets were made for.
    pub extent: vk::Extent2D,
    /// Whether target 1 holds an earlier frame: GlView keeps myRaytraceFBO1 between
    /// frames, and program 0 leaves the pixels of missed rays (depth 1, GL_LESS) unwritten.
    pub initialized: std::sync::atomic::AtomicBool,
}
impl FsaaTargets {
    /// Targets of `extent`; `FsaaTargets::new(rt, resources.attachment_extent)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>, extent: vk::Extent2D) -> Result<Self, RenderError> {
        let usage = vk::ImageUsageFlags::COLOR_ATTACHMENT
            | vk::ImageUsageFlags::SAMPLED
            | vk::ImageUsageFlags::TRANSFER_DST;
        let depth_usage =
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED;
        let mut targets = Self {
            runtime: runtime.clone(),
            color: [
                RaytraceImage::new(runtime, extent, FSAA_COLOR, usage)?,
                RaytraceImage::new(runtime, extent, FSAA_COLOR, usage)?,
            ],
            depth: RaytraceImage::new(runtime, extent, FSAA_DEPTH, depth_usage)?,
            sampler: super::raytrace_environment::linear_sampler(runtime)?,
            layouts: [sampler_layout(runtime, 1)?, sampler_layout(runtime, 2)?],
            pool: vk::DescriptorPool::null(),
            inputs: [vk::DescriptorSet::null(); 2],
            outputs: [vk::DescriptorSet::null(); 2],
            extent,
            initialized: Default::default(),
        };
        targets.allocate_sets()?;
        Ok(targets)
    }
    fn allocate_sets(&mut self) -> Result<(), RenderError> {
        let sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 6,
        }];
        let info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(4)
            .pool_sizes(&sizes);
        let device = self.runtime.device();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.pool = unsafe { device.create_descriptor_pool(&info, None) }
            .map_err(|e| vk_error("create ray-tracing FSAA pool", e))?;
        let layouts = [
            self.layouts[0],
            self.layouts[0],
            self.layouts[1],
            self.layouts[1],
        ];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let sets = unsafe { device.allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate ray-tracing FSAA sets", e))?;
        (self.inputs, self.outputs) = ([sets[0], sets[1]], [sets[2], sets[3]]);
        let image = |view| {
            [vk::DescriptorImageInfo::default()
                .sampler(self.sampler)
                .image_view(view)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)]
        };
        let views = [
            (sets[0], 0, image(self.color[0].view)),
            (sets[1], 0, image(self.color[1].view)),
            (sets[2], 0, image(self.color[0].view)),
            (sets[2], 1, image(self.depth.view)),
            (sets[3], 0, image(self.color[1].view)),
            (sets[3], 1, image(self.depth.view)),
        ];
        let writes: Vec<_> = views
            .iter()
            .map(|(set, binding, info)| {
                vk::WriteDescriptorSet::default()
                    .dst_set(*set)
                    .dst_binding(*binding)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(info)
            })
            .collect();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { device.update_descriptor_sets(&writes, &[]) };
        Ok(())
    }
}
impl Drop for FsaaTargets {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_sampler(self.sampler, None);
            for layout in self.layouts {
                device.destroy_descriptor_set_layout(layout, None);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoothing_offsets_follow_run_raytrace() {
        assert_eq!(
            super::fsaa_offset(1, [200, 100]),
            [(1. / 200.) * -0.55, (1. / 100.) * 0.55]
        );
        assert_eq!(super::fsaa_offset(2, [200, 100]), [0., (1. / 100.) * -0.55]);
        assert_eq!(super::fsaa_offset(3, [200, 100]), [(1. / 200.) * 0.55, 0.]);
    }
}
