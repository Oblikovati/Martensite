//! Capability-gated mesh shader twin for single-sample opaque stock lighting (#594).
use super::{
    pipeline::{DrawPipeline, load_shader},
    pipeline_state::PipelineState,
    raster_pipeline_policy::create_pipeline,
    runtime::{VulkanRuntime, vk_error},
    storage_descriptor::StorageDescriptorLayout,
};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;

const MESH: &[u8] = include_bytes!("../../shaders/lighting_meshlet.mesh.spv");
pub(super) const PUSH_BYTES: u32 = 100;

pub(super) struct MeshletPipeline {
    pub(super) handle: vk::Pipeline,
    pub(super) layout: vk::PipelineLayout,
    pub(super) geometry: StorageDescriptorLayout,
    pub(super) lighting: StorageDescriptorLayout,
    pub(super) command: ash::ext::mesh_shader::Device,
    unused: vk::DescriptorSetLayout,
    shader: vk::ShaderModule,
    runtime: Arc<VulkanRuntime>,
    base: Arc<DrawPipeline>,
}

impl MeshletPipeline {
    pub(super) fn new(
        runtime: &Arc<VulkanRuntime>,
        base: Arc<DrawPipeline>,
    ) -> Result<Self, RenderError> {
        let mut owner = Self {
            handle: vk::Pipeline::null(),
            layout: vk::PipelineLayout::null(),
            geometry: StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::MESH_EXT)?,
            lighting: StorageDescriptorLayout::new(
                runtime,
                vk::ShaderStageFlags::MESH_EXT | vk::ShaderStageFlags::FRAGMENT,
            )?,
            command: ash::ext::mesh_shader::Device::new(&runtime.instance, runtime.device()),
            unused: vk::DescriptorSetLayout::null(),
            shader: vk::ShaderModule::null(),
            runtime: runtime.clone(),
            base,
        };
        let unused = vk::DescriptorSetLayoutCreateInfo::default();
        // SAFETY: the runtime owns the live device and this empty layout has no borrowed bindings.
        owner.unused = unsafe { runtime.device().create_descriptor_set_layout(&unused, None) }
            .map_err(|error| vk_error("create unused meshlet descriptor layout", error))?;
        owner.shader = load_shader(runtime, MESH)?;
        owner.layout = owner.create_layout()?;
        owner.handle = owner.build()?;
        Ok(owner)
    }

    fn create_layout(&self) -> Result<vk::PipelineLayout, RenderError> {
        let mut sets = [self.unused; 8];
        sets[0] = self.lighting.handle;
        if let Some(shadow) = &self.base.shadow_layout {
            sets[6] = shadow.handle;
        }
        sets[7] = self.geometry.handle;
        let constants = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::MESH_EXT)
            .size(PUSH_BYTES)];
        let request = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&sets)
            .push_constant_ranges(&constants);
        // SAFETY: each owned layout and the borrowed create-info arrays outlive this call.
        unsafe { self.runtime.device().create_pipeline_layout(&request, None) }
            .map_err(|error| vk_error("create meshlet pipeline layout", error))
    }

    fn build(&self) -> Result<vk::Pipeline, RenderError> {
        let fragment = self
            .base
            .meshlet_fragment_module()
            .ok_or_else(|| rejected(self.base.kind, "a stock fragment module for mesh shading"))?;
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::MESH_EXT)
                .module(self.shader)
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment)
                .name(c"main"),
        ];
        let kind = self.base.kind;
        let state = PipelineState::new(kind);
        let colors = [vk::Format::R8G8B8A8_SRGB];
        let depth = kind.depth_format(self.runtime.depth.formats);
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&colors)
            .depth_attachment_format(depth)
            .stencil_attachment_format(if kind.stencil {
                depth
            } else {
                vk::Format::UNDEFINED
            });
        let mut request = state
            .description(self.layout, &stages)
            .push_next(&mut rendering);
        request.p_vertex_input_state = std::ptr::null();
        request.p_input_assembly_state = std::ptr::null();
        create_pipeline(&self.runtime, request, &state.raster_description())
    }
}

impl Drop for MeshletPipeline {
    fn drop(&mut self) {
        // SAFETY: this owner holds its device, and a submitted frame retains the owner through its fence.
        unsafe {
            let device = self.runtime.device();
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_shader_module(self.shader, None);
            device.destroy_descriptor_set_layout(self.unused, None);
        }
    }
}
