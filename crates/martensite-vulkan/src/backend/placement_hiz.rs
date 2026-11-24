//! Build a conservative max-depth pyramid from the current camera's isolated occluder pass.
use super::{
    host_upload_pages::{FrameUploadPages, UploadRange},
    image::{AttachmentSpec, TargetImage},
    pipeline::load_shader,
    placement_visibility::PlacementVisibilityPipeline,
    runtime::{VulkanRuntime, vk_error},
    storage_descriptor::{StorageDescriptorLayout, UploadedStorage},
    target_sample_set::TargetSampleSet,
};
use crate::RenderError;
use ash::vk;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

#[path = "placement_hiz_pyramid.rs"]
mod pyramid;
pub(super) use pyramid::HiZPyramid;

pub(super) fn optional_hiz<T>(
    prepared: Result<Option<T>, RenderError>,
) -> Result<Option<T>, RenderError> {
    match prepared {
        Err(error)
            if !super::device_lifecycle::contains_device_loss(&error)
                && super::geometry_buffer::optional_geometry_refusal(&error) =>
        {
            Ok(None)
        }
        result => result,
    }
}

#[derive(Default)]
pub(super) struct HiZCache {
    supported: OnceLock<bool>,
    pipeline: Mutex<Option<Arc<HiZPipeline>>>,
}

impl HiZCache {
    pub(super) fn supported(&self, runtime: &VulkanRuntime) -> bool {
        *self
            .supported
            .get_or_init(|| supports_depth_pyramid(runtime))
    }

    /// Prepare only after the selected depth format and graphics queue support compute sampling.
    pub(super) fn get(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<Option<Arc<HiZPipeline>>, RenderError> {
        if !self.supported(runtime) {
            return Ok(None);
        }
        let mut cached = self.pipeline.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(pipeline) = cached.as_ref() {
            return Ok(Some(pipeline.clone()));
        }
        let pipeline = Arc::new(HiZPipeline::new(runtime)?);
        *cached = Some(pipeline.clone());
        Ok(Some(pipeline))
    }
}

fn supports_depth_pyramid(runtime: &VulkanRuntime) -> bool {
    if !runtime.placement_occlusion
        || !runtime.placement_visibility
        || runtime.limits.max_per_stage_descriptor_storage_buffers < 5
        || runtime.limits.max_descriptor_set_storage_buffers < 5
        || runtime.limits.sub_pixel_precision_bits < 4
    {
        return false;
    }
    // SAFETY: the selected physical device belongs to the live instance; the query borrows no host data.
    let result = unsafe {
        runtime
            .instance
            .get_physical_device_image_format_properties(
                runtime.physical,
                runtime.depth.formats.plain,
                vk::ImageType::TYPE_2D,
                vk::ImageTiling::OPTIMAL,
                pyramid::DEPTH_USAGE,
                vk::ImageCreateFlags::empty(),
            )
    };
    result.is_ok_and(|format| format.sample_counts.contains(vk::SampleCountFlags::TYPE_1))
}

pub(super) struct HiZPipeline {
    runtime: Arc<VulkanRuntime>,
    pub(super) cull: PlacementVisibilityPipeline,
    storage: StorageDescriptorLayout,
    source_layout: vk::DescriptorSetLayout,
    layout: vk::PipelineLayout,
    shader: vk::ShaderModule,
    pipeline: vk::Pipeline,
}

impl HiZPipeline {
    fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let mut pipeline = Self {
            runtime: runtime.clone(),
            cull: PlacementVisibilityPipeline::new_mode(runtime, true)?,
            storage: StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::COMPUTE)?,
            source_layout: vk::DescriptorSetLayout::null(),
            layout: vk::PipelineLayout::null(),
            shader: vk::ShaderModule::null(),
            pipeline: vk::Pipeline::null(),
        };
        pipeline.create_layouts()?;
        pipeline.create_pipeline()?;
        Ok(pipeline)
    }

    fn create_layouts(&mut self) -> Result<(), RenderError> {
        let bindings = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE)];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let device = self.runtime.device();
        // SAFETY: the device and its borrowed descriptor information stay live through creation.
        self.source_layout = unsafe { device.create_descriptor_set_layout(&info, None) }
            .map_err(|error| vk_error("create Hi-Z sampled depth layout", error))?;
        let sets = [self.source_layout, self.storage.handle];
        let push = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .size(28)];
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&sets)
            .push_constant_ranges(&push);
        // SAFETY: the descriptor layouts belong to this device and the create description is live.
        self.layout = unsafe { device.create_pipeline_layout(&info, None) }
            .map_err(|error| vk_error("create Hi-Z reduction layout", error))?;
        Ok(())
    }

    fn create_pipeline(&mut self) -> Result<(), RenderError> {
        self.shader = load_shader(
            &self.runtime,
            include_bytes!("../../shaders/placement_hiz_reduce.comp.spv"),
        )?;
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(self.shader)
            .name(c"main");
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(self.layout);
        let device = self.runtime.device();
        // SAFETY: the shader and layout belong to this device and outlive creation.
        match unsafe { device.create_compute_pipelines(vk::PipelineCache::null(), &[info], None) } {
            Ok(pipelines) => self.pipeline = pipelines[0],
            Err((created, error)) => {
                for pipeline in created
                    .into_iter()
                    .filter(|pipeline| *pipeline != vk::Pipeline::null())
                {
                    // SAFETY: the driver returned these unscheduled pipelines on this device.
                    unsafe { device.destroy_pipeline(pipeline, None) };
                }
                return Err(vk_error("create Hi-Z reduction pipeline", error));
            }
        }
        Ok(())
    }
}

impl Drop for HiZPipeline {
    fn drop(&mut self) {
        // SAFETY: frames retain this pipeline and its layouts through retirement.
        unsafe {
            let device = self.runtime.device();
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_shader_module(self.shader, None);
            device.destroy_descriptor_set_layout(self.source_layout, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_resource_refusal_falls_back_before_recording() {
        let prepare = |code| optional_hiz::<()>(Err(vk_error("prepare Hi-Z test", code)));
        assert_eq!(
            prepare(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY).unwrap(),
            None
        );
        assert_eq!(prepare(vk::Result::ERROR_TOO_MANY_OBJECTS).unwrap(), None);
        assert!(prepare(vk::Result::ERROR_DEVICE_LOST).is_err());
        assert!(prepare(vk::Result::ERROR_FORMAT_NOT_SUPPORTED).is_err());
    }
}
