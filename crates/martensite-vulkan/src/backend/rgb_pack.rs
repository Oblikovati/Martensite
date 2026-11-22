//! A compute pass packs the RGBA rows of a colour readback into RGB rows on the GPU, so an
//! RGB dump copies rows into its image instead of converting every pixel on the CPU (#594).
use super::{
    pipeline::load_shader,
    runtime::{VulkanRuntime, vk_error},
};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

/// Pixels per workgroup row, as `local_size_x` in `shaders/rgb_pack.comp`.
const WORDS_PER_GROUP: u32 = 64;

/// The rows one pack reads and writes, as the shader's push constants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub(super) struct RgbPackRows {
    pub width: u32,
    pub height: u32,
    /// The first source row of the logical frame (`AttachmentCoordinates::leading_rows`).
    pub first_row: u32,
    /// 1 when destination row 0 is the last logical source row.
    pub reversed: u32,
}

impl RgbPackRows {
    /// The bytes of one packed row, padded to a whole word; `rows.row_stride()`.
    pub fn row_stride(self) -> usize {
        (self.width as usize * 3).div_ceil(4) * 4
    }
    /// The bytes of all packed rows; `rows.packed_bytes()`.
    pub fn packed_bytes(self) -> usize {
        self.row_stride() * self.height as usize
    }
    fn words(self) -> [u32; 4] {
        [self.width, self.height, self.first_row, self.reversed]
    }
}

/// The compute pipeline and its single descriptor set; created on the first RGB dump.
pub(super) struct RgbPackPipeline {
    runtime: Arc<VulkanRuntime>,
    shader: vk::ShaderModule,
    set_layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set: vk::DescriptorSet,
    layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
}

impl RgbPackPipeline {
    /// Build the pipeline; `RgbPackPipeline::new(&runtime)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let mut pack = Self {
            runtime: runtime.clone(),
            shader: vk::ShaderModule::null(),
            set_layout: vk::DescriptorSetLayout::null(),
            pool: vk::DescriptorPool::null(),
            set: vk::DescriptorSet::null(),
            layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
        };
        pack.create_descriptors()?;
        pack.create_pipeline()?;
        Ok(pack)
    }
    fn create_descriptors(&mut self) -> Result<(), RenderError> {
        let device = self.runtime.device();
        let binding = |index: u32| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(index)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        };
        let bindings = [binding(0), binding(1)];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.set_layout = unsafe { device.create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create RGB pack descriptor layout", e))?;
        let sizes = [vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(2)];
        let info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(1)
            .pool_sizes(&sizes);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.pool = unsafe { device.create_descriptor_pool(&info, None) }
            .map_err(|e| vk_error("create RGB pack descriptor pool", e))?;
        let layouts = [self.set_layout];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the pool and layout belong to this device and the pool has room for this set.
        self.set = unsafe { device.allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate RGB pack descriptor set", e))?[0];
        Ok(())
    }
    fn create_pipeline(&mut self) -> Result<(), RenderError> {
        self.shader = load_shader(
            &self.runtime,
            include_bytes!("../../shaders/rgb_pack.comp.spv"),
        )?;
        let layouts = [self.set_layout];
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .size(std::mem::size_of::<RgbPackRows>() as u32)];
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&layouts)
            .push_constant_ranges(&ranges);
        let device = self.runtime.device();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.layout = unsafe { device.create_pipeline_layout(&info, None) }
            .map_err(|e| vk_error("create RGB pack layout", e))?;
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(self.shader)
            .name(c"main");
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(self.layout);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let pipelines =
            unsafe { device.create_compute_pipelines(vk::PipelineCache::null(), &[info], None) }
                .map_err(|(_, e)| vk_error("create RGB pack pipeline", e))?;
        self.pipeline = pipelines[0];
        Ok(())
    }
    /// Record the pack of `rows` from `source` into `packed`, both written or read by the
    /// surrounding barriers; `pack.record(command, source, packed, rows)`. The caller waits
    /// for each pack to complete, so rewriting the one descriptor set here is safe.
    pub fn record(
        &self,
        command: vk::CommandBuffer,
        source: vk::Buffer,
        packed: vk::Buffer,
        rows: RgbPackRows,
    ) {
        self.write_descriptors(source, packed);
        let device = self.runtime.device();
        let words = rows.words();
        let constants: Vec<u8> = words.iter().flat_map(|word| word.to_ne_bytes()).collect();
        let groups = (rows.row_stride() as u32 / 4).div_ceil(WORDS_PER_GROUP);
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_bind_pipeline(command, vk::PipelineBindPoint::COMPUTE, self.pipeline);
            device.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::COMPUTE,
                self.layout,
                0,
                &[self.set],
                &[],
            );
            device.cmd_push_constants(
                command,
                self.layout,
                vk::ShaderStageFlags::COMPUTE,
                0,
                &constants,
            );
            device.cmd_dispatch(command, groups, rows.height, 1);
        }
    }
    fn write_descriptors(&self, source: vk::Buffer, packed: vk::Buffer) {
        let infos = [source, packed].map(|buffer| {
            [vk::DescriptorBufferInfo::default()
                .buffer(buffer)
                .range(vk::WHOLE_SIZE)]
        });
        let writes = [0, 1].map(|index| {
            vk::WriteDescriptorSet::default()
                .dst_set(self.set)
                .dst_binding(index as u32)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(&infos[index])
        });
        // SAFETY: the set and buffers belong to this device, and no pending work uses the set.
        unsafe { self.runtime.device().update_descriptor_sets(&writes, &[]) };
    }
}

impl Drop for RgbPackPipeline {
    fn drop(&mut self) {
        // SAFETY: the handles belong to this device; the renderer drops the pipeline after its
        // frames and readbacks completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_descriptor_pool(self.pool, None);
            device.destroy_descriptor_set_layout(self.set_layout, None);
            device.destroy_shader_module(self.shader, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RgbPackRows;

    #[test]
    fn packed_rows_are_padded_to_whole_words() {
        let rows = |width| RgbPackRows {
            width,
            height: 2,
            first_row: 0,
            reversed: 0,
        };
        assert_eq!(rows(4).row_stride(), 12);
        assert_eq!(rows(5).row_stride(), 16);
        assert_eq!(rows(1920).packed_bytes(), 1920 * 3 * 2);
        assert_eq!(rows(1).packed_bytes(), 8);
    }
}
