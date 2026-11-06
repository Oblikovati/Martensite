//! The shader grid pass (GlView::renderGrid): one geometry-free triangle with the grid
//! block, depth-tested LEQUAL without depth writes, alpha blended, never culled.
use super::{
    frame::FrameResources,
    pipeline::load_shader,
    pipeline_shader_stages::shader_stages,
    raster_pipeline_policy::create_pipeline,
    runtime::{VulkanRuntime, vk_error},
    storage_descriptor::{StorageDescriptorLayout, UploadedStorage},
};
use crate::RenderError;
use ash::vk;
use std::sync::{Arc, Mutex};

/// Attachment formats and samples the grid pipeline must match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct GridTarget {
    pub color: vk::Format,
    pub depth: vk::Format,
    pub stencil: bool,
    pub samples: vk::SampleCountFlags,
}

impl GridTarget {
    /// The scene attachments of a frame (multisampled when the frame renders with MSAA).
    pub fn of(resources: &FrameResources) -> Self {
        let depth = resources.render_depth();
        Self {
            color: resources.render_color().format,
            depth: depth.format,
            stencil: depth.aspect.contains(vk::ImageAspectFlags::STENCIL),
            samples: resources.render_color().samples,
        }
    }
}

pub(super) struct GridPipeline {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::Pipeline,
    pub layout: vk::PipelineLayout,
    pub descriptors: StorageDescriptorLayout,
    shaders: Vec<vk::ShaderModule>,
}

impl GridPipeline {
    fn new(runtime: &Arc<VulkanRuntime>, target: GridTarget) -> Result<Self, RenderError> {
        let descriptors = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT)?;
        let mut pipeline = Self {
            runtime: runtime.clone(),
            handle: vk::Pipeline::null(),
            layout: vk::PipelineLayout::null(),
            descriptors,
            shaders: vec![],
        };
        pipeline.initialize_layout()?;
        pipeline.create(target)?;
        Ok(pipeline)
    }
    fn initialize_layout(&mut self) -> Result<(), RenderError> {
        for bytes in GRID_SHADERS {
            self.shaders.push(load_shader(&self.runtime, bytes)?);
        }
        let layouts = [self.descriptors.handle];
        let info = vk::PipelineLayoutCreateInfo::default().set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.layout = unsafe { self.runtime.device().create_pipeline_layout(&info, None) }
            .map_err(|e| vk_error("create shader grid layout", e))?;
        Ok(())
    }
    fn create(&mut self, target: GridTarget) -> Result<(), RenderError> {
        let state = GridFixedState::new(target);
        // Constant 0 (kMirroredQuads): top-left attachments mirror the OpenGL derivative quads.
        let mirrored = u32::from(
            self.runtime.attachment_coordinates
                == super::attachment_coordinates::AttachmentCoordinates::TopLeftLogicalV2,
        )
        .to_ne_bytes();
        let entries = [vk::SpecializationMapEntry::default()
            .constant_id(0)
            .offset(0)
            .size(4)];
        let specialization = vk::SpecializationInfo::default()
            .map_entries(&entries)
            .data(&mirrored);
        let mut stages = shader_stages(&self.shaders);
        stages[1] = stages[1].specialization_info(&specialization);
        let blends = [grid_blend()];
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&blends);
        let formats = [target.color];
        let stencil = if target.stencil {
            target.depth
        } else {
            vk::Format::UNDEFINED
        };
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&formats)
            .depth_attachment_format(target.depth)
            .stencil_attachment_format(stencil);
        let info = state
            .description(self.layout, &stages)
            .color_blend_state(&blend)
            .push_next(&mut rendering);
        self.handle = create_pipeline(&self.runtime, info, &state.raster)?;
        Ok(())
    }
}

impl Drop for GridPipeline {
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

const GRID_SHADERS: [&[u8]; 2] = [
    include_bytes!("../../shaders/grid.vert.spv"),
    include_bytes!("../../shaders/grid.frag.spv"),
];

/// `glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)` for color and alpha.
fn grid_blend() -> vk::PipelineColorBlendAttachmentState {
    vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
        .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
        .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(vk::ColorComponentFlags::RGBA)
}

const GRID_DYNAMIC: &[vk::DynamicState] = &[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

pub(super) struct GridFixedState {
    vertex: vk::PipelineVertexInputStateCreateInfo<'static>,
    assembly: vk::PipelineInputAssemblyStateCreateInfo<'static>,
    viewport: vk::PipelineViewportStateCreateInfo<'static>,
    pub raster: vk::PipelineRasterizationStateCreateInfo<'static>,
    samples: vk::PipelineMultisampleStateCreateInfo<'static>,
    depth: vk::PipelineDepthStencilStateCreateInfo<'static>,
    dynamic: vk::PipelineDynamicStateCreateInfo<'static>,
}

impl GridFixedState {
    pub fn new(target: GridTarget) -> Self {
        Self {
            vertex: Default::default(),
            assembly: vk::PipelineInputAssemblyStateCreateInfo::default()
                .topology(vk::PrimitiveTopology::TRIANGLE_LIST),
            viewport: vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1),
            raster: vk::PipelineRasterizationStateCreateInfo::default()
                .polygon_mode(vk::PolygonMode::FILL)
                .cull_mode(vk::CullModeFlags::NONE)
                .line_width(1.),
            samples: vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(target.samples),
            depth: vk::PipelineDepthStencilStateCreateInfo::default()
                .depth_test_enable(true)
                .depth_write_enable(false)
                .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL),
            dynamic: vk::PipelineDynamicStateCreateInfo::default().dynamic_states(GRID_DYNAMIC),
        }
    }
    /// The same state writing depth: the ray-tracing quad stores its hit depth.
    pub fn with_depth_write(mut self) -> Self {
        self.depth = self.depth.depth_write_enable(true);
        self
    }
    pub fn description<'a>(
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

/// Pipelines per attachment configuration, created on first use and kept with the renderer.
#[derive(Default)]
pub(super) struct GridPipelineCache {
    pipelines: Mutex<Vec<(GridTarget, Arc<GridPipeline>)>>,
}

impl GridPipelineCache {
    fn pipeline(
        &self,
        runtime: &Arc<VulkanRuntime>,
        target: GridTarget,
    ) -> Result<Arc<GridPipeline>, RenderError> {
        let mut pipelines = self.pipelines.lock().map_err(|error| {
            crate::error::rejected(error.to_string(), "an unpoisoned shader grid cache")
        })?;
        if let Some((_, pipeline)) = pipelines.iter().find(|(key, _)| *key == target) {
            return Ok(pipeline.clone());
        }
        let pipeline = Arc::new(GridPipeline::new(runtime, target)?);
        pipelines.push((target, pipeline.clone()));
        Ok(pipeline)
    }
}

/// The uploaded grid of one frame.
pub(super) struct GridFrameDraw {
    pub span: usize,
    pub pipeline: Arc<GridPipeline>,
    pub block: UploadedStorage,
}

/// Prepare the frame's grid draw when the frame carries one; `prepare_grid(rt, cache, frame, res)?`.
pub(super) fn prepare_grid(
    runtime: &Arc<VulkanRuntime>,
    cache: &GridPipelineCache,
    frame: &crate::RenderFrame,
    resources: &FrameResources,
) -> Result<Option<GridFrameDraw>, RenderError> {
    let Some(grid) = &frame.shader_grid else {
        return Ok(None);
    };
    let pipeline = cache.pipeline(runtime, GridTarget::of(resources))?;
    let block = UploadedStorage::new(runtime, &pipeline.descriptors, &grid.uniforms.bytes())?;
    Ok(Some(GridFrameDraw {
        span: grid.span,
        pipeline,
        block,
    }))
}

/// Record the grid as its own rendering scope after the main-pass spans.
pub(super) fn record_grid(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    clear: [f32; 4],
    grid: &GridFrameDraw,
) {
    let first = grid.span == 0 && resources.background.is_none();
    if !first {
        super::record::layer_dependency(runtime, command);
    }
    super::record::begin_rendering(runtime, command, resources, clear, first, false, false);
    let device = runtime.device();
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_bind_pipeline(
            command,
            vk::PipelineBindPoint::GRAPHICS,
            grid.pipeline.handle,
        );
        device.cmd_bind_descriptor_sets(
            command,
            vk::PipelineBindPoint::GRAPHICS,
            grid.pipeline.layout,
            0,
            &[grid.block.set],
            &[],
        );
        device.cmd_draw(command, 3, 1, 0, 0);
        device.cmd_end_rendering(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_state_matches_native_render_grid() {
        let target = GridTarget {
            color: vk::Format::R8G8B8A8_SRGB,
            depth: vk::Format::D32_SFLOAT,
            stencil: false,
            samples: vk::SampleCountFlags::TYPE_1,
        };
        let state = GridFixedState::new(target);
        assert_eq!(state.depth.depth_test_enable, vk::TRUE);
        assert_eq!(state.depth.depth_write_enable, vk::FALSE);
        assert_eq!(state.depth.depth_compare_op, vk::CompareOp::LESS_OR_EQUAL);
        assert_eq!(state.raster.cull_mode, vk::CullModeFlags::NONE);
        assert_eq!(
            grid_blend().dst_color_blend_factor,
            vk::BlendFactor::ONE_MINUS_SRC_ALPHA
        );
        assert_eq!(state.vertex.vertex_binding_description_count, 0);
    }
}
