//! Geometry-free pipelines of the environment passes (PBR environment baking, skydome
//! generation, cubemap background): vertices come from gl_VertexIndex, uniforms are push
//! constants, the only descriptor is the sampled cube map, and there is no blending.
use super::{
    pipeline::load_shader,
    pipeline_shader_stages::shader_stages,
    raster_pipeline_policy::create_pipeline,
    runtime::{VulkanRuntime, vk_error},
};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

/// Everything that distinguishes one environment pipeline from another.
#[derive(Clone, Copy)]
pub(super) struct EnvironmentPipelineSpec<'a> {
    pub shaders: [&'a [u8]; 2],
    pub topology: vk::PrimitiveTopology,
    pub color: vk::Format,
    /// Depth (and stencil) formats of the scene attachments; `UNDEFINED` for texture passes.
    pub depth: vk::Format,
    pub stencil: vk::Format,
    pub samples: vk::SampleCountFlags,
    pub push: vk::PushConstantRange,
    pub set_layout: Option<vk::DescriptorSetLayout>,
}

/// One pipeline with its layout and shader modules.
pub(super) struct EnvironmentPipeline {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    shaders: Vec<vk::ShaderModule>,
}

impl EnvironmentPipeline {
    /// Create the pipeline; `EnvironmentPipeline::new(&runtime, spec)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        spec: EnvironmentPipelineSpec<'_>,
    ) -> Result<Self, RenderError> {
        let mut pipeline = Self {
            runtime: runtime.clone(),
            handle: vk::Pipeline::null(),
            layout: vk::PipelineLayout::null(),
            shaders: Vec::with_capacity(2),
        };
        for bytes in spec.shaders {
            pipeline.shaders.push(load_shader(runtime, bytes)?);
        }
        pipeline.layout = create_layout(runtime, &spec)?;
        pipeline.handle = pipeline.create(&spec)?;
        Ok(pipeline)
    }
    fn create(&self, spec: &EnvironmentPipelineSpec<'_>) -> Result<vk::Pipeline, RenderError> {
        let state = FixedState::new(spec);
        let stages = shader_stages(&self.shaders);
        let blends = [vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)];
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&blends);
        let formats = [spec.color];
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&formats)
            .depth_attachment_format(spec.depth)
            .stencil_attachment_format(spec.stencil);
        let info = state
            .description(self.layout, &stages)
            .color_blend_state(&blend)
            .push_next(&mut rendering);
        create_pipeline(&self.runtime, info, &state.raster)
    }
    /// Bind the pipeline, its sampled cube map (if any) and the push constants.
    /// Example: `pipeline.bind(device, command, Some(cube.set), &constants)`.
    pub fn bind(
        &self,
        device: &ash::Device,
        command: vk::CommandBuffer,
        set: Option<vk::DescriptorSet>,
        stages: vk::ShaderStageFlags,
        constants: &[u8],
    ) {
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, self.handle);
            if let Some(set) = set {
                device.cmd_bind_descriptor_sets(
                    command,
                    vk::PipelineBindPoint::GRAPHICS,
                    self.layout,
                    0,
                    &[set],
                    &[],
                );
            }
            device.cmd_push_constants(command, self.layout, stages, 0, constants);
        }
    }
}

impl Drop for EnvironmentPipeline {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
            for shader in self.shaders.drain(..) {
                device.destroy_shader_module(shader, None);
            }
        }
    }
}

fn create_layout(
    runtime: &VulkanRuntime,
    spec: &EnvironmentPipelineSpec<'_>,
) -> Result<vk::PipelineLayout, RenderError> {
    let sets: Vec<_> = spec.set_layout.into_iter().collect();
    let ranges = [spec.push];
    let info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&sets)
        .push_constant_ranges(&ranges);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_pipeline_layout(&info, None) }
        .map_err(|e| vk_error("create environment pipeline layout", e))
}

const DYNAMIC: &[vk::DynamicState] = &[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

/// Never culled (Graphic3dTypeOfBackfacingModel_DoubleSided), no depth test or write
/// (GlPBREnvironmentSentry, GlView::drawBackground), one sample per pixel unless the
/// scene is multisampled.
struct FixedState {
    vertex: vk::PipelineVertexInputStateCreateInfo<'static>,
    assembly: vk::PipelineInputAssemblyStateCreateInfo<'static>,
    viewport: vk::PipelineViewportStateCreateInfo<'static>,
    raster: vk::PipelineRasterizationStateCreateInfo<'static>,
    samples: vk::PipelineMultisampleStateCreateInfo<'static>,
    depth: vk::PipelineDepthStencilStateCreateInfo<'static>,
    dynamic: vk::PipelineDynamicStateCreateInfo<'static>,
}

impl FixedState {
    fn new(spec: &EnvironmentPipelineSpec<'_>) -> Self {
        Self {
            vertex: Default::default(),
            assembly: vk::PipelineInputAssemblyStateCreateInfo::default().topology(spec.topology),
            viewport: vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1),
            raster: vk::PipelineRasterizationStateCreateInfo::default()
                .polygon_mode(vk::PolygonMode::FILL)
                .cull_mode(vk::CullModeFlags::NONE)
                .line_width(1.),
            samples: vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(spec.samples),
            depth: vk::PipelineDepthStencilStateCreateInfo::default(),
            dynamic: vk::PipelineDynamicStateCreateInfo::default().dynamic_states(DYNAMIC),
        }
    }
    fn description<'a>(
        &'a self,
        layout: vk::PipelineLayout,
        stages: &'a [vk::PipelineShaderStageCreateInfo<'a>],
    ) -> vk::GraphicsPipelineCreateInfo<'a> {
        vk::GraphicsPipelineCreateInfo::default()
            .layout(layout)
            .stages(stages)
            .vertex_input_state(&self.vertex)
            .input_assembly_state(&self.assembly)
            .viewport_state(&self.viewport)
            .rasterization_state(&self.raster)
            .multisample_state(&self.samples)
            .depth_stencil_state(&self.depth)
            .dynamic_state(&self.dynamic)
    }
}

/// Pack native-endian 32-bit words into a push-constant block of `N` bytes.
/// Example: `push_constant_bytes::<8>([1i32.to_ne_bytes(), 2f32.to_ne_bytes()])`.
pub(super) fn push_constant_bytes<const N: usize>(
    words: impl IntoIterator<Item = [u8; 4]>,
) -> [u8; N] {
    let mut bytes = [0u8; N];
    for (chunk, word) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(words) {
        *chunk = word;
    }
    bytes
}

/// Record a square texture pass into `view`: a positive viewport of `extent` texels.
/// Example: `begin_texture_pass(device, command, face.handle, [size, size])`.
pub(super) fn begin_texture_pass(
    device: &ash::Device,
    command: vk::CommandBuffer,
    view: vk::ImageView,
    extent: [u32; 2],
) {
    let colors = [vk::RenderingAttachmentInfo::default()
        .image_view(view)
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::DONT_CARE)
        .store_op(vk::AttachmentStoreOp::STORE)];
    let area = vk::Rect2D::default().extent(vk::Extent2D {
        width: extent[0],
        height: extent[1],
    });
    let info = vk::RenderingInfo::default()
        .render_area(area)
        .layer_count(1)
        .color_attachments(&colors);
    let viewport = vk::Viewport::default()
        .width(extent[0] as f32)
        .height(extent[1] as f32)
        .max_depth(1.);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_begin_rendering(command, &info);
        device.cmd_set_viewport(command, 0, &[viewport]);
        device.cmd_set_scissor(command, 0, &[area]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_passes_never_cull_test_or_write_depth() {
        let spec = EnvironmentPipelineSpec {
            shaders: [&[], &[]],
            topology: vk::PrimitiveTopology::TRIANGLE_STRIP,
            color: vk::Format::R8G8B8A8_UNORM,
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            samples: vk::SampleCountFlags::TYPE_1,
            push: vk::PushConstantRange::default(),
            set_layout: None,
        };
        let state = FixedState::new(&spec);
        assert_eq!(state.raster.cull_mode, vk::CullModeFlags::NONE);
        assert_eq!(state.depth.depth_test_enable, vk::FALSE);
        assert_eq!(state.depth.depth_write_enable, vk::FALSE);
        assert_eq!(
            state.assembly.topology,
            vk::PrimitiveTopology::TRIANGLE_STRIP
        );
        assert_eq!(state.vertex.vertex_binding_description_count, 0);
    }
}
