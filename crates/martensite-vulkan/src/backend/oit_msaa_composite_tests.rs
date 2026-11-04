//! The depth-peeling blend and flush MSAA programs (Graphic3dShaderManager
//! getStdProgramOitDepthPeelingBlend/Flush with theMsaa) run on the device over four-sample
//! inputs. Depth peeling initialises its buffers single-sample and falls back to
//! zero samples (the design notes), so the view keeps that fallback; the reference here
//! is the programs' own arithmetic followed by the pipelines' blend equations, evaluated exactly
//! on dyadic half-float values.
use super::{
    buffer::DeviceBuffer,
    image::{AttachmentSpec, TargetImage},
    memory_selection::HostBufferAccess,
    oit_composite::{OitCompositePhase, OitCompositePipeline, OitCompositeTarget},
    oit_descriptor::OitPassDescriptors,
    runtime::{VulkanOptions, VulkanRuntime},
    submission::CommandSubmission,
};
use ash::vk;
use std::sync::Arc;

const FORMAT: vk::Format = vk::Format::R16G16B16A16_SFLOAT;
const SAMPLES: vk::SampleCountFlags = vk::SampleCountFlags::TYPE_4;
const EXTENT: vk::Extent2D = vk::Extent2D {
    width: 2,
    height: 2,
};
/// The destination's clear colour before the composite.
const DESTINATION: [f32; 4] = [0.125, 0.25, 0.75, 0.5];

/// Four-sample inputs, the four-sample output and its single-sample average resolve.
struct CompositeImages {
    front: TargetImage,
    back: TargetImage,
    output: TargetImage,
    resolved: TargetImage,
}
impl CompositeImages {
    fn new(runtime: &Arc<VulkanRuntime>) -> Self {
        let image = |usage, samples| {
            let spec = AttachmentSpec {
                format: FORMAT,
                usage: vk::ImageUsageFlags::COLOR_ATTACHMENT | usage,
                aspect: vk::ImageAspectFlags::COLOR,
                samples,
            };
            TargetImage::with_attachment(runtime, EXTENT, spec).unwrap()
        };
        Self {
            front: image(vk::ImageUsageFlags::SAMPLED, SAMPLES),
            back: image(vk::ImageUsageFlags::SAMPLED, SAMPLES),
            output: image(vk::ImageUsageFlags::empty(), SAMPLES),
            resolved: image(
                vk::ImageUsageFlags::TRANSFER_SRC,
                vk::SampleCountFlags::TYPE_1,
            ),
        }
    }
}

fn barrier(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    images: &[&TargetImage],
    layouts: (vk::ImageLayout, vk::ImageLayout),
    access: (vk::AccessFlags2, vk::AccessFlags2),
) {
    let stages = vk::PipelineStageFlags2::ALL_COMMANDS;
    let barriers: Vec<_> = images
        .iter()
        .map(|image| {
            vk::ImageMemoryBarrier2::default()
                .src_stage_mask(stages)
                .dst_stage_mask(stages)
                .src_access_mask(access.0)
                .dst_access_mask(access.1)
                .old_layout(layouts.0)
                .new_layout(layouts.1)
                .image(image.handle)
                .subresource_range(image.range())
        })
        .collect();
    let info = vk::DependencyInfo::default().image_memory_barriers(&barriers);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_pipeline_barrier2(command, &info) };
}

fn color_attachment(image: &TargetImage, clear: [f32; 4]) -> vk::RenderingAttachmentInfo<'_> {
    vk::RenderingAttachmentInfo::default()
        .image_view(image.view)
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(vk::ClearValue {
            color: vk::ClearColorValue { float32: clear },
        })
}

fn render_area() -> vk::Rect2D {
    vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent: EXTENT,
    }
}

/// Clear every sample of the front and back inputs and make them shader-readable.
fn record_inputs(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    images: &CompositeImages,
    inputs: [[f32; 4]; 2],
) {
    let targets = [
        &images.front,
        &images.back,
        &images.output,
        &images.resolved,
    ];
    let write = vk::AccessFlags2::COLOR_ATTACHMENT_WRITE;
    let to_attachment = (
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    );
    barrier(
        runtime,
        command,
        &targets,
        to_attachment,
        (vk::AccessFlags2::NONE, write),
    );
    let attachments = [
        color_attachment(&images.front, inputs[0]),
        color_attachment(&images.back, inputs[1]),
    ];
    let info = vk::RenderingInfo::default()
        .render_area(render_area())
        .layer_count(1)
        .color_attachments(&attachments);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_begin_rendering(command, &info);
        runtime.device().cmd_end_rendering(command);
    }
    let to_read = (
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    );
    let read = vk::AccessFlags2::SHADER_SAMPLED_READ;
    barrier(
        runtime,
        command,
        &[&images.front, &images.back],
        to_read,
        (write, read),
    );
}

fn record_composite(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    images: &CompositeImages,
    pipeline: &OitCompositePipeline,
    descriptors: &OitPassDescriptors,
) {
    let attachments = [color_attachment(&images.output, DESTINATION)
        .resolve_mode(vk::ResolveModeFlags::AVERAGE)
        .resolve_image_view(images.resolved.view)
        .resolve_image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let info = vk::RenderingInfo::default()
        .render_area(render_area())
        .layer_count(1)
        .color_attachments(&attachments);
    let device = runtime.device();
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_begin_rendering(command, &info);
        super::render_target_record::set_viewport(runtime, command, EXTENT);
        device.cmd_set_stencil_test_enable(command, false);
        device.cmd_bind_pipeline(command, vk::PipelineBindPoint::GRAPHICS, pipeline.handle);
        let sets = [descriptors.set];
        let point = vk::PipelineBindPoint::GRAPHICS;
        device.cmd_bind_descriptor_sets(command, point, pipeline.layout, 0, &sets, &[]);
        device.cmd_draw(command, 3, 1, 0, 0);
        device.cmd_end_rendering(command);
    }
}

fn record_readback(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    image: &TargetImage,
    buffer: &DeviceBuffer,
) {
    let layouts = (
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
    );
    let access = (
        vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
        vk::AccessFlags2::TRANSFER_READ,
    );
    barrier(runtime, command, &[image], layouts, access);
    let region = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: EXTENT.width,
            height: EXTENT.height,
            depth: 1,
        });
    let layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_copy_image_to_buffer(
            command,
            image.handle,
            layout,
            buffer.handle,
            &[region],
        );
        let host = vk::MemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::ALL_TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::HOST)
            .dst_access_mask(vk::AccessFlags2::HOST_READ);
        let barriers = [host];
        let info = vk::DependencyInfo::default().memory_barriers(&barriers);
        runtime.device().cmd_pipeline_barrier2(command, &info);
    }
}

/// An IEEE binary16 value as f32 (finite values only).
fn half_to_f32(bits: u16) -> f32 {
    let sign = if bits & 0x8000 != 0 { -1. } else { 1. };
    let exponent = i32::from((bits >> 10) & 0x1f);
    let mantissa = f32::from(bits & 0x3ff);
    assert_ne!(exponent, 0x1f, "expected a finite half, got {bits:#06x}");
    if exponent == 0 {
        return sign * mantissa * (martensite_cmath::powi(f64::from(2f32), -24) as f32);
    }
    sign * (1. + mantissa / 1024.) * (martensite_cmath::powi(f64::from(2f32), exponent - 15) as f32)
}

/// The resolved RGBA of every pixel after one composite of `phase` over `inputs` (front, back).
fn composite(
    runtime: &Arc<VulkanRuntime>,
    phase: OitCompositePhase,
    inputs: [[f32; 4]; 2],
) -> Vec<[f32; 4]> {
    let images = CompositeImages::new(runtime);
    let target = OitCompositeTarget {
        phase,
        format: FORMAT,
        depth: vk::Format::UNDEFINED,
        stencil: false,
        samples: SAMPLES,
    };
    let pipeline = OitCompositePipeline::new(runtime, target).unwrap();
    let sources = match phase {
        OitCompositePhase::Back => [images.back.view, images.back.view],
        _ => [images.front.view, images.back.view],
    };
    let descriptors = OitPassDescriptors::new(&pipeline.descriptors, sources, 0.).unwrap();
    let size = u64::from(EXTENT.width * EXTENT.height) * 8;
    let buffer = DeviceBuffer::new(
        runtime,
        size,
        vk::BufferUsageFlags::TRANSFER_DST,
        HostBufferAccess::Readback,
    )
    .unwrap();
    let submission = CommandSubmission::new(runtime).unwrap();
    record_inputs(runtime, submission.command, &images, inputs);
    record_composite(
        runtime,
        submission.command,
        &images,
        &pipeline,
        &descriptors,
    );
    record_readback(runtime, submission.command, &images.resolved, &buffer);
    submission.finish().unwrap();
    let bytes = buffer.read(size as usize).unwrap();
    let halves: Vec<f32> = bytes
        .chunks(2)
        .map(|pair| half_to_f32(u16::from_le_bytes([pair[0], pair[1]])))
        .collect();
    halves
        .chunks(4)
        .map(|pixel| [pixel[0], pixel[1], pixel[2], pixel[3]])
        .collect()
}

/// Flush: `vec4(front.rgb + (1 - front.a) * back.rgb, front.a + back.a)`, blended ONE /
/// ONE_MINUS_SRC_ALPHA on colour with alpha writes masked.
fn expected_flush(front: [f32; 4], back: [f32; 4]) -> [f32; 4] {
    let alpha = front[3] + back[3];
    let source = |c: usize| front[c] + (1. - front[3]) * back[c];
    let blend = |c: usize| source(c) + DESTINATION[c] * (1. - alpha);
    [blend(0), blend(1), blend(2), DESTINATION[3]]
}

/// Back blend: the back colour, discarded at zero alpha, blended SRC_ALPHA / ONE_MINUS_SRC_ALPHA
/// on colour and ONE / ONE_MINUS_SRC_ALPHA on alpha.
fn expected_back(back: [f32; 4]) -> [f32; 4] {
    if back[3] == 0. {
        return DESTINATION;
    }
    let colour = |c: usize| back[c] * back[3] + DESTINATION[c] * (1. - back[3]);
    [
        colour(0),
        colour(1),
        colour(2),
        back[3] + DESTINATION[3] * (1. - back[3]),
    ]
}

fn four_sample_runtime() -> Option<Arc<VulkanRuntime>> {
    let runtime = VulkanRuntime::create(VulkanOptions {
        validation: true,
        device_index: Some(0),
    })
    .unwrap();
    let limits = runtime.limits;
    let supported = limits.framebuffer_color_sample_counts.contains(SAMPLES)
        && limits.sampled_image_color_sample_counts.contains(SAMPLES)
        && runtime.sample_rate_shading;
    supported.then_some(runtime)
}

#[test]
fn depth_peeling_msaa_blend_and_flush_programs_composite_four_sample_inputs() {
    let Some(runtime) = four_sample_runtime() else {
        panic!("expected a device with four-sample float attachments and sample-rate shading");
    };
    let front = [0.25, 0.5, 0., 0.5];
    let back = [0.5, 0.25, 1., 0.5];
    let flushed = composite(&runtime, OitCompositePhase::Peeling, [front, back]);
    assert!(
        flushed
            .iter()
            .all(|pixel| *pixel == expected_flush(front, back)),
        "{flushed:?}"
    );
    let blended = composite(&runtime, OitCompositePhase::Back, [front, back]);
    assert!(
        blended.iter().all(|pixel| *pixel == expected_back(back)),
        "{blended:?}"
    );
    let empty = [0.75, 0.5, 0.25, 0.];
    let discarded = composite(&runtime, OitCompositePhase::Back, [front, empty]);
    assert!(
        discarded.iter().all(|pixel| *pixel == DESTINATION),
        "{discarded:?}"
    );
    let errors = runtime.validation.errors().unwrap();
    assert!(errors.is_empty(), "{errors:?}");
}
