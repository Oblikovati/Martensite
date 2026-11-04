//! Test-only fragment substitution over the production raytrace vertex program.
use super::super::{
    pipeline::{load_shader, shader_stages},
    raster_pipeline_policy::create_pipeline,
    runtime::VulkanRuntime,
    shader_grid_pass::{GridFixedState, GridTarget},
};
use ash::vk;
use std::sync::Arc;
pub(super) struct PixelPipeline {
    runtime: Arc<VulkanRuntime>,
    handle: vk::Pipeline,
    layout: vk::PipelineLayout,
    shaders: Vec<vk::ShaderModule>,
}
impl PixelPipeline {
    pub(super) fn new(runtime: &Arc<VulkanRuntime>, descriptor: vk::DescriptorSetLayout) -> Self {
        let fragment = include_bytes!("../../shaders/raytrace_controls/pixel.frag.spv");
        let data = reference_parameters(runtime);
        let entries = [specialization_entry(0), specialization_entry(1)];
        Self::with_fragment(runtime, &[descriptor], fragment, (&data, &entries))
    }
    pub(super) fn with_fragment(
        runtime: &Arc<VulkanRuntime>,
        descriptors: &[vk::DescriptorSetLayout],
        fragment: &[u8],
        constants: (&[u8], &[vk::SpecializationMapEntry]),
    ) -> Self {
        let mut result = Self {
            runtime: runtime.clone(),
            handle: vk::Pipeline::null(),
            layout: vk::PipelineLayout::null(),
            shaders: vec![],
        };
        result.load(fragment);
        result.layout(descriptors);
        result.create(constants);
        result
    }
    fn load(&mut self, fragment: &[u8]) {
        let vertex = include_bytes!("../../shaders/raytrace.vert.spv");
        for source in [vertex.as_slice(), fragment] {
            self.shaders
                .push(load_shader(&self.runtime, source).unwrap());
        }
    }
    fn layout(&mut self, descriptors: &[vk::DescriptorSetLayout]) {
        let info = vk::PipelineLayoutCreateInfo::default().set_layouts(descriptors);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.layout = unsafe { self.runtime.device().create_pipeline_layout(&info, None) }.unwrap();
    }
    fn create(&mut self, (data, entries): (&[u8], &[vk::SpecializationMapEntry])) {
        let state = GridFixedState::new(probe_target());
        let depth = vk::PipelineDepthStencilStateCreateInfo::default();
        let mut rendering = vk::PipelineRenderingCreateInfo::default();
        let mut stages = shader_stages(&self.shaders);
        let specialization = vk::SpecializationInfo::default()
            .map_entries(entries)
            .data(data);
        stages[1] = stages[1].specialization_info(&specialization);
        let info = state
            .description(self.layout, &stages)
            .depth_stencil_state(&depth)
            .push_next(&mut rendering);
        self.handle = create_pipeline(&self.runtime, info, &state.raster).unwrap();
    }
    pub(super) fn draw(&self, command: vk::CommandBuffer, descriptor: vk::DescriptorSet) {
        self.draw_sets(command, &[descriptor]);
    }
    pub(super) fn draw_sets(&self, command: vk::CommandBuffer, descriptors: &[vk::DescriptorSet]) {
        let device = self.runtime.device();
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, self.handle);
            device.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::GRAPHICS,
                self.layout,
                0,
                descriptors,
                &[],
            );
            device.cmd_draw(command, 6, 1, 0, 0);
        }
    }
}
impl Drop for PixelPipeline {
    fn drop(&mut self) {
        let device = self.runtime.device();
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
            for shader in self.shaders.drain(..) {
                device.destroy_shader_module(shader, None);
            }
        }
    }
}

fn probe_target() -> GridTarget {
    GridTarget {
        color: vk::Format::UNDEFINED,
        depth: vk::Format::UNDEFINED,
        stencil: false,
        samples: vk::SampleCountFlags::TYPE_1,
    }
}
fn reference_flip(runtime: &VulkanRuntime) -> u32 {
    use super::super::attachment_coordinates::AttachmentCoordinates;
    u32::from(runtime.attachment_coordinates == AttachmentCoordinates::TopLeftLogicalV2)
}

fn reference_parameters(runtime: &VulkanRuntime) -> Vec<u8> {
    [
        reference_flip(runtime),
        runtime.attachment_coordinates.leading_rows(super::EXTENT),
    ]
    .into_iter()
    .flat_map(u32::to_ne_bytes)
    .collect()
}
pub(super) fn specialization_entry(index: u32) -> vk::SpecializationMapEntry {
    vk::SpecializationMapEntry::default()
        .constant_id(index)
        .offset(index * 4)
        .size(4)
}
