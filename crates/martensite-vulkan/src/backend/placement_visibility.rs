//! An optional compute dispatch compacts placements without reordering their indirect draws.
use super::{
    pipeline::load_shader,
    runtime::{VulkanRuntime, vk_error},
    storage_descriptor::StorageDescriptorLayout,
};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::{Arc, Mutex, PoisonError};

#[path = "placement_compaction.rs"]
mod compaction;
pub(super) use compaction::{HiZRequest, PlacementCompaction};

pub(super) const PLACEMENTS_PER_GROUP: u32 = 64;

pub(super) fn supported(
    runtime: &VulkanRuntime,
    physical: vk::PhysicalDevice,
    family: u32,
) -> bool {
    if !runtime.draw_tables || runtime.limits.max_draw_indirect_count < PLACEMENTS_PER_GROUP {
        return false;
    }
    let mut features = vk::PhysicalDeviceVulkan12Features::default();
    let mut query = vk::PhysicalDeviceFeatures2::default().push_next(&mut features);
    // SAFETY: the selected physical device belongs to this instance and the driver only writes to the scoped query storage.
    let families = unsafe {
        runtime
            .instance
            .get_physical_device_features2(physical, &mut query);
        runtime
            .instance
            .get_physical_device_queue_family_properties(physical)
    };
    let compute = families
        .get(family as usize)
        .is_some_and(|family| family.queue_flags.contains(vk::QueueFlags::COMPUTE));
    compute && features.draw_indirect_count == vk::TRUE
}

/// A cached pipeline is shared with every frame that recorded a visibility dispatch.
#[derive(Default)]
pub(super) struct PlacementVisibilityCache(Mutex<Option<Arc<PlacementVisibilityPipeline>>>);

impl PlacementVisibilityCache {
    pub(super) fn get(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<Option<Arc<PlacementVisibilityPipeline>>, RenderError> {
        if !runtime.placement_visibility || cfg!(feature = "command-state-diagnostics") {
            return Ok(None);
        }
        let mut cached = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(pipeline) = cached.as_ref() {
            return Ok(Some(pipeline.clone()));
        }
        let pipeline = Arc::new(PlacementVisibilityPipeline::new(runtime)?);
        *cached = Some(pipeline.clone());
        Ok(Some(pipeline))
    }
}

pub(super) struct PlacementVisibilityPipeline {
    runtime: Arc<VulkanRuntime>,
    storage: StorageDescriptorLayout,
    shader: vk::ShaderModule,
    layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
}

impl PlacementVisibilityPipeline {
    /// Prepare one pipeline whose descriptors are allocated separately for each frame.
    pub(super) fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        Self::new_mode(runtime, false)
    }

    pub(super) fn new_mode(runtime: &Arc<VulkanRuntime>, hiz: bool) -> Result<Self, RenderError> {
        if !runtime.placement_visibility {
            return Err(rejected(false, "GPU placement visibility support"));
        }
        let mut visibility = Self {
            runtime: runtime.clone(),
            storage: StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::COMPUTE)?,
            shader: vk::ShaderModule::null(),
            layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
        };
        visibility.create_layout(hiz)?;
        visibility.create_pipeline(hiz)?;
        Ok(visibility)
    }

    fn create_layout(&mut self, hiz: bool) -> Result<(), RenderError> {
        let descriptors = [self.storage.handle; 5];
        let set_count = if hiz { 5 } else { 3 };
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .size(if hiz { 60 } else { 28 })];
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&descriptors[..set_count])
            .push_constant_ranges(&ranges);
        // SAFETY: this pipeline owns the descriptor layout, and all create information lives through the call.
        self.layout = unsafe { self.runtime.device().create_pipeline_layout(&info, None) }
            .map_err(|error| vk_error("create placement visibility pipeline layout", error))?;
        Ok(())
    }

    fn create_pipeline(&mut self, hiz: bool) -> Result<(), RenderError> {
        let bytes: &[u8] = if hiz {
            include_bytes!("../../shaders/placement_hiz_cull.comp.spv")
        } else {
            include_bytes!("../../shaders/placement_visibility.comp.spv")
        };
        self.shader = load_shader(&self.runtime, bytes)?;
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(self.shader)
            .name(c"main");
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(self.layout);
        let device = self.runtime.device();
        // SAFETY: the shader and layout belong to the live device and outlive creation.
        match unsafe { device.create_compute_pipelines(vk::PipelineCache::null(), &[info], None) } {
            Ok(pipelines) => self.pipeline = pipelines[0],
            Err((created, error)) => {
                for pipeline in created
                    .into_iter()
                    .filter(|pipeline| *pipeline != vk::Pipeline::null())
                {
                    // SAFETY: these pipelines were returned by creation on this device and have never been scheduled.
                    unsafe { device.destroy_pipeline(pipeline, None) };
                }
                return Err(vk_error("create placement visibility pipeline", error));
            }
        }
        Ok(())
    }

    fn bind_compute(&self, command: vk::CommandBuffer, sets: &[vk::DescriptorSet]) {
        // SAFETY: the caller records on a compute-capable command buffer, and the frame owns every set and the pipeline.
        unsafe {
            let device = self.runtime.device();
            device.cmd_bind_pipeline(command, vk::PipelineBindPoint::COMPUTE, self.pipeline);
            device.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::COMPUTE,
                self.layout,
                0,
                sets,
                &[],
            );
        }
    }

    fn dispatch<const N: usize>(&self, command: vk::CommandBuffer, words: [u32; N], groups: u32) {
        let constants = words.map(u32::to_ne_bytes);
        // SAFETY: the caller bound these descriptors and pipeline before recording this work.
        unsafe {
            let device = self.runtime.device();
            device.cmd_push_constants(
                command,
                self.layout,
                vk::ShaderStageFlags::COMPUTE,
                0,
                constants.as_flattened(),
            );
            device.cmd_dispatch(command, groups, 1, 1);
        }
    }
}

impl Drop for PlacementVisibilityPipeline {
    fn drop(&mut self) {
        // SAFETY: each dependent frame owns a lease, so this last owner drops after execution retires.
        unsafe {
            let device = self.runtime.device();
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_shader_module(self.shader, None);
        }
    }
}
