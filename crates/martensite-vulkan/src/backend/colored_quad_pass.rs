//! The corner gradient background of GlView::drawBackground (AspectGradientFillMethod
//! Corner1..4): the quad of GlBackgroundArray drawn with getColoredQuadProgram first in the
//! frame, with depth test and writes disabled, as its own rendering scope that clears the
//! attachments.
use super::environment_pipeline::{EnvironmentPipeline, EnvironmentPipelineSpec};
use super::{frame::FrameResources, runtime::VulkanRuntime, shader_grid_pass::GridTarget};
use crate::RenderError;
use crate::colored_quad_background::ColoredQuadBackground;
use ash::vk;
use std::sync::{Arc, Mutex};

const COLORED_QUAD_SHADERS: [&[u8]; 2] = [
    include_bytes!("../../shaders/colored_quad.vert.spv"),
    include_bytes!("../../shaders/colored_quad.frag.spv"),
];
/// ColoredQuadConstants: mat4, two vec4 colours, vec2 view size, int corner (padded to 16 bytes).
const COLORED_QUAD_CONSTANTS: usize = 112;
const COLORED_QUAD_STAGES: vk::ShaderStageFlags = vk::ShaderStageFlags::from_raw(
    vk::ShaderStageFlags::VERTEX.as_raw() | vk::ShaderStageFlags::FRAGMENT.as_raw(),
);

/// Pipelines per scene attachment configuration, created on first use.
#[derive(Default)]
pub(super) struct ColoredQuadPipelineCache {
    pipelines: Mutex<Vec<(GridTarget, Arc<EnvironmentPipeline>)>>,
}

impl ColoredQuadPipelineCache {
    fn pipeline(
        &self,
        runtime: &Arc<VulkanRuntime>,
        target: GridTarget,
    ) -> Result<Arc<EnvironmentPipeline>, RenderError> {
        let mut pipelines = self.pipelines.lock().map_err(|error| {
            crate::error::rejected(error.to_string(), "an unpoisoned colored quad cache")
        })?;
        if let Some((_, pipeline)) = pipelines.iter().find(|(key, _)| *key == target) {
            return Ok(pipeline.clone());
        }
        let pipeline = Arc::new(EnvironmentPipeline::new(runtime, spec(target))?);
        pipelines.push((target, pipeline.clone()));
        Ok(pipeline)
    }
}

fn spec(target: GridTarget) -> EnvironmentPipelineSpec<'static> {
    EnvironmentPipelineSpec {
        shaders: COLORED_QUAD_SHADERS,
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        color: target.color,
        depth: target.depth,
        stencil: if target.stencil {
            target.depth
        } else {
            vk::Format::UNDEFINED
        },
        samples: target.samples,
        push: vk::PushConstantRange::default()
            .stage_flags(COLORED_QUAD_STAGES)
            .size(COLORED_QUAD_CONSTANTS as u32),
        set_layout: None,
    }
}

/// The corner gradient draw of one frame.
pub(super) struct ColoredQuadFrameDraw {
    pipeline: Arc<EnvironmentPipeline>,
    constants: [u8; COLORED_QUAD_CONSTANTS],
    /// Renders through the attachment's UNORM alias (fixed-function option).
    encoded: bool,
}

impl ColoredQuadFrameDraw {
    /// Select the pipeline of the frame's attachments.
    /// Example: `ColoredQuadFrameDraw::new(rt, &cache, &resources, &quad)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        cache: &ColoredQuadPipelineCache,
        resources: &FrameResources,
        quad: &ColoredQuadBackground,
    ) -> Result<Self, RenderError> {
        let mut target = GridTarget::of(resources);
        let encoded = quad.encoded && target.color == vk::Format::R8G8B8A8_SRGB;
        if encoded {
            target.color = vk::Format::R8G8B8A8_UNORM;
        }
        Ok(Self {
            pipeline: cache.pipeline(runtime, target)?,
            constants: colored_quad_constants(quad),
            encoded,
        })
    }

    /// Record the quad as the frame's first rendering scope (it clears the attachments).
    /// Example: `draw.record(runtime, command, &resources, clear)`.
    pub fn record(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        resources: &FrameResources,
        clear: [f32; 4],
    ) {
        // The UNORM alias stores the clear colour as given: encode it as the sRGB view would.
        let clear = match self.encoded {
            true => martensite_tkernel::quantity::ColorRgba::linear_to_srgb_vec4(clear),
            false => clear,
        };
        let encoded = self.encoded;
        super::record::begin_rendering(runtime, command, resources, clear, true, true, encoded);
        let device = runtime.device();
        self.pipeline
            .bind(device, command, None, COLORED_QUAD_STAGES, &self.constants);
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_draw(command, 6, 1, 0, 0);
            device.cmd_end_rendering(command);
        }
    }
}

fn colored_quad_constants(quad: &ColoredQuadBackground) -> [u8; COLORED_QUAD_CONSTANTS] {
    let colors = quad
        .colors
        .iter()
        .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 1.]);
    let floats = quad
        .projection
        .iter()
        .copied()
        .chain(colors)
        .chain(quad.view_size);
    let words = floats
        .map(f32::to_ne_bytes)
        .chain([(quad.corner as i32).to_ne_bytes()]);
    super::environment_pipeline::push_constant_bytes(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_hold_the_chart_colors_size_then_the_corner() {
        let mut projection = [0f32; 16];
        projection[0] = 0.5;
        let quad = ColoredQuadBackground {
            projection,
            colors: [[1., 0., 0.], [0., 0., 1.]],
            view_size: [320., 240.],
            corner: 3,
            encoded: false,
        };
        let bytes = colored_quad_constants(&quad);
        let float = |at: usize| f32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap());
        assert_eq!(float(0), 0.5);
        assert_eq!(
            [float(64), float(76), float(88), float(92)],
            [1., 1., 1., 1.]
        );
        assert_eq!([float(96), float(100)], [320., 240.]);
        assert_eq!(i32::from_ne_bytes(bytes[104..108].try_into().unwrap()), 3);
    }
}
