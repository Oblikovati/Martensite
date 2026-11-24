//! The full-screen draw that restores retained main images into an immediate frame's targets.
//! RADV records an image copy through a meta pass that saves and restores the command state,
//! and two of them cost about 10 µs per hover pair; one fetch draw costs a fraction (#594).
use super::{
    pipeline::load_shader,
    pipeline_shader_stages::shader_stages,
    raster_pipeline_policy::create_pipeline,
    runtime::{VulkanRuntime, vk_error},
};
use crate::RenderError;
use ash::vk;
use std::sync::{Arc, Mutex, PoisonError};

/// The colour bytes go through the UNORM alias, so the draw copies them without an sRGB round trip.
pub(super) const RESTORE_COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;

pub(super) struct MainRestorePipeline {
    runtime: Arc<VulkanRuntime>,
    /// One sampled image at binding 0; set 0 holds the colour, set 1 the depth.
    pub set_layout: vk::DescriptorSetLayout,
    layout: vk::PipelineLayout,
    handle: vk::Pipeline,
    shaders: Vec<vk::ShaderModule>,
}

/// The restore pipeline of one renderer, built when the first immediate frame restores.
#[derive(Default)]
pub(super) struct MainRestoreCache(Mutex<Option<Arc<MainRestorePipeline>>>);

impl MainRestoreCache {
    /// The cached pipeline for `depth` targets; `pipelines.main_restore.get(runtime)?`.
    pub fn get(
        &self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<Arc<MainRestorePipeline>, RenderError> {
        let mut cached = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(pipeline) = cached.as_ref() {
            return Ok(Arc::clone(pipeline));
        }
        let pipeline = Arc::new(MainRestorePipeline::new(
            runtime,
            runtime.depth.formats.plain,
        )?);
        *cached = Some(Arc::clone(&pipeline));
        Ok(pipeline)
    }
}

impl MainRestorePipeline {
    /// Build the pipeline for UNORM colour and `depth` attachments; `MainRestorePipeline::new(rt, f)?`.
    pub fn new(runtime: &Arc<VulkanRuntime>, depth: vk::Format) -> Result<Self, RenderError> {
        let mut pipeline = Self {
            runtime: Arc::clone(runtime),
            set_layout: vk::DescriptorSetLayout::null(),
            layout: vk::PipelineLayout::null(),
            handle: vk::Pipeline::null(),
            shaders: Vec::new(),
        };
        pipeline.create_layouts()?;
        for bytes in RESTORE_SHADERS {
            pipeline.shaders.push(load_shader(runtime, bytes)?);
        }
        pipeline.handle = pipeline.create(depth)?;
        Ok(pipeline)
    }
    fn create_layouts(&mut self) -> Result<(), RenderError> {
        let bindings = [restore_binding()];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let device = self.runtime.device();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.set_layout = unsafe { device.create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create main restore descriptor layout", e))?;
        let sets = [self.set_layout; 2];
        let info = vk::PipelineLayoutCreateInfo::default().set_layouts(&sets);
        // SAFETY: as above; both set layouts are live.
        self.layout = unsafe { device.create_pipeline_layout(&info, None) }
            .map_err(|e| vk_error("create main restore layout", e))?;
        Ok(())
    }
    fn create(&self, depth: vk::Format) -> Result<vk::Pipeline, RenderError> {
        let state = RestoreFixedState::new();
        let stages = shader_stages(&self.shaders);
        let blends = [restore_blend()];
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&blends);
        let formats = [RESTORE_COLOR_FORMAT];
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&formats)
            .depth_attachment_format(depth);
        let info = state
            .description(self.layout, &stages)
            .color_blend_state(&blend)
            .push_next(&mut rendering);
        create_pipeline(&self.runtime, info, &state.raster)
    }
    /// Draw the saved images, bound as `sets` (colour, depth), over `extent` of the open
    /// rendering scope; `pipeline.record(command, sets, extent)`.
    pub fn record(
        &self,
        command: vk::CommandBuffer,
        sets: [vk::DescriptorSet; 2],
        extent: vk::Extent2D,
    ) {
        let device = self.runtime.device();
        let (viewport, scissor) = restore_region(extent);
        // SAFETY: the command buffer is recording on this thread inside a rendering scope whose
        // formats match the pipeline, and the sets name images that outlive the submission.
        unsafe {
            device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, self.handle);
            device.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::GRAPHICS,
                self.layout,
                0,
                &sets,
                &[],
            );
            device.cmd_set_viewport(command, 0, &[viewport]);
            device.cmd_set_scissor(command, 0, &[scissor]);
            device.cmd_draw(command, 3, 1, 0, 0);
        }
    }
}

impl Drop for MainRestorePipeline {
    fn drop(&mut self) {
        // SAFETY: the handles belong to this device and the renderer drops its pipelines only
        // after the GPU work that used them has completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_pipeline(self.handle, None);
            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_descriptor_set_layout(self.set_layout, None);
            for shader in self.shaders.drain(..) {
                device.destroy_shader_module(shader, None);
            }
        }
    }
}

const RESTORE_SHADERS: [&[u8]; 2] = [
    include_bytes!("../../shaders/oit_composite.vert.spv"),
    include_bytes!("../../shaders/main_restore.frag.spv"),
];

fn restore_binding() -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}

/// Every fragment fetches its own texel, so a plain viewport over the attachment suffices.
fn restore_region(extent: vk::Extent2D) -> (vk::Viewport, vk::Rect2D) {
    let viewport = vk::Viewport {
        x: 0.,
        y: 0.,
        width: extent.width as f32,
        height: extent.height as f32,
        min_depth: 0.,
        max_depth: 1.,
    };
    let scissor = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent,
    };
    (viewport, scissor)
}

/// Writes every channel without blending.
fn restore_blend() -> vk::PipelineColorBlendAttachmentState {
    vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA)
}

struct RestoreFixedState {
    vertex: vk::PipelineVertexInputStateCreateInfo<'static>,
    assembly: vk::PipelineInputAssemblyStateCreateInfo<'static>,
    viewport: vk::PipelineViewportStateCreateInfo<'static>,
    raster: vk::PipelineRasterizationStateCreateInfo<'static>,
    samples: vk::PipelineMultisampleStateCreateInfo<'static>,
    depth: vk::PipelineDepthStencilStateCreateInfo<'static>,
    dynamic: vk::PipelineDynamicStateCreateInfo<'static>,
}

const RESTORE_DYNAMIC: &[vk::DynamicState] =
    &[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

impl RestoreFixedState {
    fn new() -> Self {
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
                .rasterization_samples(vk::SampleCountFlags::TYPE_1),
            // The fetched depth replaces whatever the target held: test always, write always.
            depth: vk::PipelineDepthStencilStateCreateInfo::default()
                .depth_test_enable(true)
                .depth_write_enable(true)
                .depth_compare_op(vk::CompareOp::ALWAYS),
            dynamic: vk::PipelineDynamicStateCreateInfo::default().dynamic_states(RESTORE_DYNAMIC),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_restore_replaces_every_channel_and_the_depth_of_the_whole_attachment() {
        let state = RestoreFixedState::new();
        assert_eq!(state.depth.depth_write_enable, vk::TRUE);
        assert_eq!(state.depth.depth_compare_op, vk::CompareOp::ALWAYS);
        assert_eq!(state.raster.cull_mode, vk::CullModeFlags::NONE);
        let blend = restore_blend();
        assert_eq!(blend.blend_enable, vk::FALSE);
        assert_eq!(blend.color_write_mask, vk::ColorComponentFlags::RGBA);
        let extent = vk::Extent2D {
            width: 48,
            height: 32,
        };
        let (viewport, scissor) = restore_region(extent);
        assert_eq!([viewport.width, viewport.height], [48., 32.]);
        assert_eq!([viewport.min_depth, viewport.max_depth], [0., 1.]);
        assert_eq!(scissor.extent, extent);
        assert_eq!(
            restore_binding().descriptor_type,
            vk::DescriptorType::SAMPLED_IMAGE
        );
    }
}
