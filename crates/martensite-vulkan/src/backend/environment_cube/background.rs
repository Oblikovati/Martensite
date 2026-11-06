//! The cubemap background of GlView::drawBackground (Graphic3dTOB_CUBEMAP): the box of
//! GlBackgroundArray drawn with getBgCubeMapProgram first in the frame, with depth test and
//! writes disabled, as its own rendering scope that clears the attachments.
use super::super::environment_pipeline::{EnvironmentPipeline, EnvironmentPipelineSpec};
use super::super::{frame::FrameResources, runtime::VulkanRuntime, shader_grid_pass::GridTarget};
use super::EnvironmentCube;
use crate::RenderError;
use ash::vk;
use std::sync::{Arc, Mutex};

const BACKGROUND_SHADERS: [&[u8]; 2] = [
    include_bytes!("../../../shaders/background_cubemap.vert.spv"),
    include_bytes!("../../../shaders/background_cubemap.frag.spv"),
];

/// Pipelines per scene attachment configuration, created on first use.
#[derive(Default)]
pub(in super::super) struct BackgroundPipelineCache {
    pipelines: Mutex<Vec<(GridTarget, Arc<EnvironmentPipeline>)>>,
}

impl BackgroundPipelineCache {
    fn pipeline(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: vk::DescriptorSetLayout,
        target: GridTarget,
    ) -> Result<Arc<EnvironmentPipeline>, RenderError> {
        let mut pipelines = self.pipelines.lock().map_err(|error| {
            crate::error::rejected(error.to_string(), "an unpoisoned background pipeline cache")
        })?;
        if let Some((_, pipeline)) = pipelines.iter().find(|(key, _)| *key == target) {
            return Ok(pipeline.clone());
        }
        let pipeline = Arc::new(EnvironmentPipeline::new(runtime, spec(layout, target))?);
        pipelines.push((target, pipeline.clone()));
        Ok(pipeline)
    }
}

fn spec(layout: vk::DescriptorSetLayout, target: GridTarget) -> EnvironmentPipelineSpec<'static> {
    EnvironmentPipelineSpec {
        shaders: BACKGROUND_SHADERS,
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
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .size(72),
        set_layout: Some(layout),
    }
}

/// The background draw of one frame.
pub(in super::super) struct BackgroundFrameDraw {
    pipeline: Arc<EnvironmentPipeline>,
    cube: Arc<EnvironmentCube>,
    constants: [u8; 72],
}

impl BackgroundFrameDraw {
    /// Select the pipeline of the frame's attachments.
    /// Example: `BackgroundFrameDraw::new(rt, cache, layout, &resources, cube, &background)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        cache: &BackgroundPipelineCache,
        layout: vk::DescriptorSetLayout,
        resources: &FrameResources,
        cube: Arc<EnvironmentCube>,
        background: &crate::environment_frame::CubeBackground,
    ) -> Result<Self, RenderError> {
        let pipeline = cache.pipeline(runtime, layout, GridTarget::of(resources))?;
        Ok(Self {
            pipeline,
            cube,
            constants: background_constants(background),
        })
    }
}

/// BackgroundCubeMapConstants: occProjectionWorldView, uYCoeff, uZCoeff.
fn background_constants(background: &crate::environment_frame::CubeBackground) -> [u8; 72] {
    let coefficients = background.source.coefficients();
    let words = background
        .projection_world_view
        .iter()
        .map(|value| value.to_ne_bytes())
        .chain(coefficients.iter().map(|value| value.to_ne_bytes()));
    super::super::environment_pipeline::push_constant_bytes(words)
}

/// Record the background as the frame's first rendering scope (it clears the attachments).
/// Example: `record_background(runtime, command, &resources, clear, &draw)`.
pub(in super::super) fn record_background(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    clear: [f32; 4],
    draw: &BackgroundFrameDraw,
) {
    super::super::record::begin_rendering(runtime, command, resources, clear, true, true, false);
    let device = runtime.device();
    draw.pipeline.bind(
        device,
        command,
        Some(draw.cube.set),
        vk::ShaderStageFlags::VERTEX,
        &draw.constants,
    );
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_draw(command, 36, 1, 0, 0);
        device.cmd_end_rendering(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment_frame::{CubeBackground, EnvironmentCubeSource, SkydomeParams};

    #[test]
    fn constants_hold_the_matrix_then_the_coefficients() {
        let mut matrix = [0f32; 16];
        matrix[15] = 2.;
        let background = CubeBackground {
            source: EnvironmentCubeSource::Skydome(SkydomeParams {
                sun_direction: [0., 1., 0.],
                cloudiness: 0.,
                time: 0.,
                fogginess: 0.,
                size: 8,
            }),
            projection_world_view: matrix,
        };
        let bytes = background_constants(&background);
        assert_eq!(f32::from_ne_bytes(bytes[60..64].try_into().unwrap()), 2.);
        assert_eq!(i32::from_ne_bytes(bytes[64..68].try_into().unwrap()), 1);
        assert_eq!(i32::from_ne_bytes(bytes[68..72].try_into().unwrap()), 1);
    }
}
