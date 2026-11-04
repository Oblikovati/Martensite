//! GlView::runPathtrace and runPathtraceOut with adaptive sampling: clear
//! the output image at the first accumulated frame and the visual error every frame, upload the
//! tile samples (or offsets), run one pass per seed with an image barrier between passes
//! (glMemoryBarrier), draw the output program into the frame, and copy the visual error for
//! the next frame's GrabVarianceMap.
use super::{
    buffer::DeviceBuffer, frame::FrameResources, memory_selection::HostBufferAccess,
    raytrace_adaptive::AdaptiveTargets, raytrace_image::RaytraceImage, raytrace_pass::bind,
    raytrace_pipeline::RaytracePipeline, runtime::VulkanRuntime,
};
use crate::RenderError;
use crate::raytrace::{AdaptivePasses, PathtraceFrame};
use ash::vk;
use std::sync::Arc;

type Scope = (vk::PipelineStageFlags2, vk::AccessFlags2);
const STORAGE: Scope = (
    vk::PipelineStageFlags2::FRAGMENT_SHADER,
    vk::AccessFlags2::from_raw(
        vk::AccessFlags2::SHADER_STORAGE_READ.as_raw()
            | vk::AccessFlags2::SHADER_STORAGE_WRITE.as_raw(),
    ),
);
const TRANSFER: Scope = (
    vk::PipelineStageFlags2::ALL_TRANSFER,
    vk::AccessFlags2::from_raw(
        vk::AccessFlags2::TRANSFER_READ.as_raw() | vk::AccessFlags2::TRANSFER_WRITE.as_raw(),
    ),
);
const HOST_READ: Scope = (vk::PipelineStageFlags2::HOST, vk::AccessFlags2::HOST_READ);
const GENERAL: vk::ImageLayout = vk::ImageLayout::GENERAL;

/// The adaptive half of one path-traced frame.
pub(super) struct AdaptiveDraw {
    pub targets: Arc<AdaptiveTargets>,
    pub render: Arc<RaytracePipeline>,
    pub display: Arc<RaytracePipeline>,
    pub passes: AdaptivePasses,
    pub debug: bool,
    /// The frame's accumulation count and output parameters.
    pub frame: PathtraceFrame,
    /// The tile samples (or offsets) of this frame.
    upload: DeviceBuffer,
    /// The visual error after the output program, read once the frame completes.
    readback: DeviceBuffer,
}
impl AdaptiveDraw {
    /// Stage the map of `frame`, whose adaptive passes are filled;
    /// `AdaptiveDraw::new(rt, targets, (render, display), frame)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        targets: Arc<AdaptiveTargets>,
        (render, display): (Arc<RaytracePipeline>, Arc<RaytracePipeline>),
        frame: PathtraceFrame,
    ) -> Result<Self, RenderError> {
        let adaptive = frame.adaptive.as_ref();
        let debug = adaptive.is_some_and(|adaptive| adaptive.debug);
        let passes = adaptive
            .and_then(|adaptive| adaptive.passes.clone())
            .ok_or_else(|| {
                crate::error::rejected(frame.accum_frames, "prepared adaptive passes")
            })?;
        let bytes = tile_map_bytes(&passes, targets.key.atomic);
        let upload = DeviceBuffer::new(
            runtime,
            bytes.len() as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
            HostBufferAccess::Upload,
        )?;
        upload.write(&bytes)?;
        let errors = (passes.tiles[0] * passes.tiles[1]) as u64 * 4;
        let readback = DeviceBuffer::new(
            runtime,
            errors,
            vk::BufferUsageFlags::TRANSFER_DST,
            HostBufferAccess::Readback,
        )?;
        Ok(Self {
            targets,
            render,
            display,
            passes,
            debug,
            frame,
            upload,
            readback,
        })
    }
    /// Hand the completed frame's visual error to the view's tile sampler.
    /// Example: `draw.publish_visual_error()?` after the frame's fence.
    pub fn publish_visual_error(&self) -> Result<(), RenderError> {
        let count = (self.passes.tiles[0] * self.passes.tiles[1]) as usize;
        let bytes = self.readback.read(count * 4)?;
        let errors = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|value| i32::from_ne_bytes(*value))
            .collect();
        self.passes.feedback.publish(errors);
        Ok(())
    }
}

/// The map uploaded before the passes: tile samples (R32I) or tile offsets (RG32I), row-major.
fn tile_map_bytes(passes: &AdaptivePasses, atomic: bool) -> Vec<u8> {
    match atomic {
        true => passes
            .offsets
            .iter()
            .flatten()
            .flat_map(|v| v.to_ne_bytes())
            .collect(),
        false => passes
            .tile_samples
            .iter()
            .flat_map(|v| v.to_ne_bytes())
            .collect(),
    }
}

fn memory_barrier(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    (source, target): (Scope, Scope),
) {
    let barrier = vk::MemoryBarrier2::default()
        .src_stage_mask(source.0)
        .src_access_mask(source.1)
        .dst_stage_mask(target.0)
        .dst_access_mask(target.1);
    let dependency = vk::DependencyInfo::default().memory_barriers(std::slice::from_ref(&barrier));
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_pipeline_barrier2(command, &dependency) };
}

/// New targets leave UNDEFINED for GENERAL; later frames wait for the earlier passes, output
/// program and copies before clearing and uploading.
fn begin_images(runtime: &VulkanRuntime, command: vk::CommandBuffer, targets: &AdaptiveTargets) {
    let initialized = std::sync::atomic::Ordering::Relaxed;
    if targets.initialized.swap(true, initialized) {
        memory_barrier(runtime, command, (merge(STORAGE, TRANSFER), TRANSFER));
        return;
    }
    let none = (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE);
    let undefined = vk::ImageLayout::UNDEFINED;
    let barriers = targets
        .images()
        .map(|image| image.barrier((undefined, GENERAL), none, TRANSFER));
    let dependency = vk::DependencyInfo::default().image_memory_barriers(&barriers);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_pipeline_barrier2(command, &dependency) };
    // A new tiles image holds no stale map beyond the uploaded region.
    clear(runtime, command, &targets.tiles);
    memory_barrier(runtime, command, (TRANSFER, TRANSFER));
}

fn merge(first: Scope, second: Scope) -> Scope {
    (first.0 | second.0, first.1 | second.1)
}

fn clear(runtime: &VulkanRuntime, command: vk::CommandBuffer, image: &RaytraceImage) {
    // glClearTexImage with nullptr: zero, whether the texels are float or integer.
    let zero = vk::ClearColorValue { uint32: [0; 4] };
    let range = [image.range()];
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime
            .device()
            .cmd_clear_color_image(command, image.handle, GENERAL, &zero, &range);
    }
}

fn upload_tile_map(runtime: &VulkanRuntime, command: vk::CommandBuffer, draw: &AdaptiveDraw) {
    let size = match draw.targets.key.atomic {
        true => draw.passes.offset_tiles,
        false => draw.passes.tiles,
    };
    let region = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: size[0] as u32,
            height: size[1] as u32,
            depth: 1,
        });
    let tiles = draw.targets.tiles.handle;
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_copy_buffer_to_image(
            command,
            draw.upload.handle,
            tiles,
            GENERAL,
            &[region],
        );
    }
}

/// Clear, upload and run the passes of program 0 (runPathtrace).
fn record_passes(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    (scene, frame): (vk::DescriptorSet, &PathtraceFrame),
    draw: &AdaptiveDraw,
) {
    let targets = &draw.targets;
    let fresh = !targets
        .initialized
        .load(std::sync::atomic::Ordering::Relaxed);
    begin_images(runtime, command, targets);
    if frame.accum_frames == 0 || fresh {
        clear(runtime, command, &targets.render);
    }
    clear(runtime, command, &targets.variance);
    upload_tile_map(runtime, command, draw);
    memory_barrier(runtime, command, (TRANSFER, STORAGE));
    for seed in &draw.passes.seeds {
        begin_without_attachments(runtime, command, draw.passes.viewport);
        let push = pass_constants(*seed, draw.passes.viewport);
        bind(
            runtime,
            command,
            &draw.render,
            &[scene, targets.render_set],
            &push,
        );
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe { runtime.device().cmd_end_rendering(command) };
        memory_barrier(runtime, command, (STORAGE, STORAGE));
    }
}

fn begin_without_attachments(runtime: &VulkanRuntime, command: vk::CommandBuffer, size: [i32; 2]) {
    let extent = vk::Extent2D {
        width: size[0] as u32,
        height: size[1] as u32,
    };
    let info = vk::RenderingInfo::default()
        .render_area(vk::Rect2D {
            offset: vk::Offset2D::default(),
            extent,
        })
        .layer_count(1);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_begin_rendering(command, &info) };
    super::record::set_viewport(runtime, command, extent);
}

/// `uFrameRndSeed` and the pass viewport (raytrace_render.frag AdaptivePass).
fn pass_constants(seed: i32, viewport: [i32; 2]) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[..4].copy_from_slice(&seed.to_ne_bytes());
    bytes[8..12].copy_from_slice(&viewport[0].to_ne_bytes());
    bytes[12..].copy_from_slice(&viewport[1].to_ne_bytes());
    bytes
}

/// The adaptive Display.fs constants: the 16 bytes of the output program, then uTileSize and
/// uVarianceScaleFactor.
fn display_constants(frame: &PathtraceFrame, draw: &AdaptiveDraw) -> [u8; 28] {
    let mut bytes = [0u8; 28];
    bytes[..16].copy_from_slice(&super::raytrace_pathtrace::display_constants(frame));
    bytes[4..8].copy_from_slice(&i32::from(draw.debug).to_ne_bytes());
    let tile = draw.passes.tile_size.to_ne_bytes();
    bytes[16..20].copy_from_slice(&tile);
    bytes[20..24].copy_from_slice(&tile);
    bytes[24..].copy_from_slice(&draw.passes.variance_scale.to_ne_bytes());
    bytes
}

/// Copy the visual error for GrabVarianceMap once the output program wrote it.
fn read_visual_error(runtime: &VulkanRuntime, command: vk::CommandBuffer, draw: &AdaptiveDraw) {
    memory_barrier(runtime, command, (STORAGE, TRANSFER));
    let tiles = draw.passes.tiles;
    let region = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: tiles[0] as u32,
            height: tiles[1] as u32,
            depth: 1,
        });
    let variance = draw.targets.variance.handle;
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_copy_image_to_buffer(
            command,
            variance,
            GENERAL,
            draw.readback.handle,
            &[region],
        );
    }
    memory_barrier(runtime, command, (TRANSFER, HOST_READ));
}

/// Record one adaptive frame: the passes, the output program into the frame before the span at
/// the pass index, and the visual-error copy.
pub(super) fn record_adaptive(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    (resources, clear, first): (&FrameResources, [f32; 4], bool),
    scene: vk::DescriptorSet,
    draw: &AdaptiveDraw,
) {
    let frame = &draw.frame;
    record_passes(runtime, command, (scene, frame), draw);
    if !first {
        super::record::layer_dependency(runtime, command);
    }
    super::record::begin_rendering(runtime, command, resources, clear, first, false, false);
    let push = display_constants(frame, draw);
    let sets = [draw.targets.display_set];
    bind(runtime, command, &draw.display, &sets, &push);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_end_rendering(command) };
    read_visual_error(runtime, command, draw);
}

#[cfg(test)]
mod tests {
    #[test]
    fn pass_constants_hold_the_seed_and_the_viewport() {
        let bytes = super::pass_constants(-7, [512, 256]);
        let word = |i: usize| i32::from_ne_bytes(bytes[4 * i..][..4].try_into().unwrap());
        assert_eq!([word(0), word(1), word(2), word(3)], [-7, 0, 512, 256]);
    }
}
