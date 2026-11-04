//! GlView::updateSkydomeBg: the SkydomBackground.fs program renders each side of an RGB8
//! (non-colour, no mipmaps) cube map of AspectSkydomeBackground::Size texels with one
//! full-screen triangle; the texture is then sampled bilinearly.
use super::super::environment_pipeline::{
    EnvironmentPipeline, EnvironmentPipelineSpec, begin_texture_pass,
};
use super::super::{runtime::VulkanRuntime, submission::CommandSubmission};
use super::upload::{cube_barrier, finish_owned};
use super::{
    CubeFilter, CubeSamplerLayout, EnvironmentCube, EnvironmentImage, EnvironmentImageSpec,
};
use crate::environment_frame::SkydomeParams;
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;

/// ImageFormat_RGB without sRGB; Vulkan has no portable RGB8 attachment, RGBA8 holds the
/// same unorm channels.
const SKYDOME_FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;

const SKYDOME_SHADERS: [&[u8]; 2] = [
    include_bytes!("../../../shaders/skydome.vert.spv"),
    include_bytes!("../../../shaders/skydome.frag.spv"),
];

/// The skydome generator; `SkydomePipeline::new(&runtime)?`.
pub(in super::super) struct SkydomePipeline(EnvironmentPipeline);
impl SkydomePipeline {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        EnvironmentPipeline::new(
            runtime,
            EnvironmentPipelineSpec {
                shaders: SKYDOME_SHADERS,
                topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                color: SKYDOME_FORMAT,
                depth: vk::Format::UNDEFINED,
                stencil: vk::Format::UNDEFINED,
                samples: vk::SampleCountFlags::TYPE_1,
                push: push_range(),
                set_layout: None,
            },
        )
        .map(Self)
    }
}

fn push_range() -> vk::PushConstantRange {
    vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        .size(28)
}

/// The SkydomeConstants block of side `side`: uSunDir, uTime, uCloudy, uFog, uSide.
/// Example: `skydome_constants(&params, 0)`.
pub(super) fn skydome_constants(params: &SkydomeParams, side: i32) -> [u8; 28] {
    let words = [
        params.sun_direction[0].to_ne_bytes(),
        params.sun_direction[1].to_ne_bytes(),
        params.sun_direction[2].to_ne_bytes(),
        params.time.to_ne_bytes(),
        params.cloudiness.to_ne_bytes(),
        params.fogginess.to_ne_bytes(),
        side.to_ne_bytes(),
    ];
    super::super::environment_pipeline::push_constant_bytes(words)
}

impl EnvironmentCube {
    /// Generate the skydome cube; `EnvironmentCube::skydome(rt, layout, pipeline, &params)?`.
    pub fn skydome(
        runtime: &Arc<VulkanRuntime>,
        layout: &CubeSamplerLayout,
        pipeline: &SkydomePipeline,
        params: &SkydomeParams,
    ) -> Result<Arc<Self>, RenderError> {
        if params.size == 0 {
            return Err(rejected(params.size, "a positive skydome size"));
        }
        let usage = vk::ImageUsageFlags::SAMPLED
            | vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::COLOR_ATTACHMENT;
        let spec = EnvironmentImageSpec::cube(SKYDOME_FORMAT, params.size, 1, usage);
        let image = EnvironmentImage::new(runtime, spec)?;
        let faces = (0..6)
            .map(|side| image.face_view(0, side))
            .collect::<Result<Vec<_>, _>>()?;
        let submission = CommandSubmission::new(runtime)?;
        record_skydome(
            runtime.device(),
            submission.command,
            &pipeline.0,
            &image,
            &faces,
            params,
        );
        let (image, _) = finish_owned(submission, (image, faces))?;
        Self::sampled(runtime, layout, image, CubeFilter::Bilinear).map(Arc::new)
    }
}

fn record_skydome(
    device: &ash::Device,
    command: vk::CommandBuffer,
    pipeline: &EnvironmentPipeline,
    image: &EnvironmentImage,
    faces: &[super::super::pbr_environment::image::FaceView],
    params: &SkydomeParams,
) {
    let attachment = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
    cube_barrier(
        device,
        command,
        image,
        0..1,
        vk::ImageLayout::UNDEFINED,
        attachment,
    );
    for (side, face) in faces.iter().enumerate() {
        begin_texture_pass(device, command, face.handle, [face.extent; 2]);
        let constants = skydome_constants(params, side as i32);
        pipeline.bind(
            device,
            command,
            None,
            vk::ShaderStageFlags::FRAGMENT,
            &constants,
        );
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_draw(command, 3, 1, 0, 0);
            device.cmd_end_rendering(command);
        }
    }
    let read = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
    cube_barrier(device, command, image, 0..1, attachment, read);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_follow_the_skydome_block_layout() {
        let params = SkydomeParams {
            sun_direction: [0., 1., 0.5],
            cloudiness: 0.2,
            time: 3.,
            fogginess: 0.1,
            size: 64,
        };
        let bytes = skydome_constants(&params, 5);
        let word = |i: usize| f32::from_ne_bytes(bytes[4 * i..4 * i + 4].try_into().unwrap());
        assert_eq!([word(0), word(1), word(2)], [0., 1., 0.5]);
        assert_eq!([word(3), word(4), word(5)], [3., 0.2, 0.1]);
        assert_eq!(i32::from_ne_bytes(bytes[24..28].try_into().unwrap()), 5);
    }
}
