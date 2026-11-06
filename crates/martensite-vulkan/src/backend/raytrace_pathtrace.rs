//! GlView::runPathtrace and runPathtraceOut without adaptive sampling: frame
//! `n` of an accumulation reads the image of target `n % 2` (uAccumTexture), renders program 0
//! with PATH_TRACING into the other target without depth test, and the output program writes it
//! into the frame with its averaged depth (alpha), depth-tested.
use super::{
    frame::FrameResources, raytrace_fsaa::FsaaTargets, raytrace_fsaa_record::begin_pass,
    raytrace_image::RaytraceImage, raytrace_pass::bind, raytrace_pipeline::RaytracePipeline,
    runtime::VulkanRuntime,
};
use crate::raytrace::PathtraceFrame;
use ash::vk;
use std::sync::Arc;

/// The accumulation pass of one frame: the shared targets and the two path-tracing programs.
pub(super) struct PathtraceDraw {
    pub targets: Arc<FsaaTargets>,
    pub render: Arc<RaytracePipeline>,
    pub display: Arc<RaytracePipeline>,
    pub frame: PathtraceFrame,
}

/// The accumulated image (`myAccumFrames % 2 ? FBO2 : FBO1`) and the rendered one.
fn accumulation_slots(accum_frames: i32) -> (usize, usize) {
    let source = (accum_frames % 2) as usize;
    (source, 1 - source)
}

/// The Display.fs push constants: uAccumFrames, uDebugAdaptive, uExposure, uWhitePoint.
pub(super) fn display_constants(frame: &PathtraceFrame) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..4].copy_from_slice(&frame.accum_frames.to_ne_bytes());
    bytes[8..12].copy_from_slice(&frame.exposure.to_ne_bytes());
    bytes[12..].copy_from_slice(&frame.white_point.to_ne_bytes());
    bytes
}

fn transition(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    image: &RaytraceImage,
    to_attachment: bool,
) {
    let read = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
    let attachment = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
    let sampled = (
        vk::PipelineStageFlags2::FRAGMENT_SHADER,
        vk::AccessFlags2::SHADER_SAMPLED_READ,
    );
    let written = (
        vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
        vk::AccessFlags2::from_raw(
            vk::AccessFlags2::COLOR_ATTACHMENT_READ.as_raw()
                | vk::AccessFlags2::COLOR_ATTACHMENT_WRITE.as_raw(),
        ),
    );
    let barrier = match to_attachment {
        true => image.barrier((read, attachment), sampled, written),
        false => image.barrier((attachment, read), written, sampled),
    };
    let dependency =
        vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_pipeline_barrier2(command, &dependency) };
}

/// Record one accumulated frame and its output before the span at the pass index.
pub(super) fn record_pathtrace(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    (resources, clear, first): (&FrameResources, [f32; 4], bool),
    scene: vk::DescriptorSet,
    draw: &PathtraceDraw,
) {
    let targets = &draw.targets;
    super::raytrace_fsaa_record::begin_accumulation(runtime, command, targets);
    let (source, target) = accumulation_slots(draw.frame.accum_frames);
    transition(runtime, command, &targets.color[target], true);
    begin_pass(runtime, command, resources, &targets.color[target], None);
    bind(
        runtime,
        command,
        &draw.render,
        &[scene, targets.inputs[source]],
        &[],
    );
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_end_rendering(command) };
    transition(runtime, command, &targets.color[target], false);
    if !first {
        super::record::layer_dependency(runtime, command);
    }
    super::record::begin_rendering(runtime, command, resources, clear, first, false, false);
    let push = display_constants(&draw.frame);
    bind(
        runtime,
        command,
        &draw.display,
        &[targets.outputs[target]],
        &push,
    );
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_end_rendering(command) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn frames_alternate_between_the_two_targets() {
        assert_eq!(super::accumulation_slots(0), (0, 1));
        assert_eq!(super::accumulation_slots(3), (1, 0));
    }
}
