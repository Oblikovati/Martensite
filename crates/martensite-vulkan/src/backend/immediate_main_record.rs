//! Save a completed main color and depth pair, and draw it back before the immediate layers.
use super::{
    frame::FrameResources, image::TargetImage, main_restore_pipeline::MainRestorePipeline,
    runtime::VulkanRuntime,
};
use crate::{RenderError, RenderFrame, immediate_main::ImmediateMainPlan};
use ash::vk::{self, AccessFlags2 as Access, ImageLayout as Layout, PipelineStageFlags2 as Stage};
use std::sync::{Arc, Weak};

/// View-side proof of identity only. The frame target pool owns the published images.
#[derive(Clone, Default)]
pub(crate) struct MainLayerTicket(Arc<()>);

impl MainLayerTicket {
    pub(super) fn witness(&self) -> Weak<()> {
        Arc::downgrade(&self.0)
    }

    pub(super) fn identifies(&self, witness: &Weak<()>) -> bool {
        Arc::as_ptr(&self.0) == witness.as_ptr()
    }

    /// Whether both tickets name the same published images.
    pub(crate) fn same(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone)]
pub(crate) struct MainLayerImages {
    color: Arc<TargetImage>,
    depth: Arc<TargetImage>,
}

pub(super) struct MainLayerSnapshot {
    pub start: usize,
    pub ticket: MainLayerTicket,
    pub images: MainLayerImages,
}

pub(super) fn remember(frame: &RenderFrame, resources: &FrameResources) {
    let ticket = resources.main_publication.as_ref().or_else(|| {
        resources
            .main_snapshot
            .as_ref()
            .map(|snapshot| &snapshot.ticket)
    });
    if let (Some(seed), Some(ticket)) = (
        frame
            .immediate_main
            .as_ref()
            .and_then(ImmediateMainPlan::seed),
        ticket,
    ) {
        seed.remember(ticket.clone());
    }
}

impl MainLayerImages {
    pub(super) fn new(color: Arc<TargetImage>, depth: Arc<TargetImage>) -> Self {
        Self { color, depth }
    }

    pub(super) fn allocation_bytes(&self) -> u64 {
        self.color.allocation_bytes() + self.depth.allocation_bytes()
    }

    /// Prepare the chosen restore without changing the saved attachment representation.
    pub(super) fn into_source(
        self,
        pipeline: Option<Arc<MainRestorePipeline>>,
    ) -> Result<MainRestoreSource, RenderError> {
        let draw = pipeline
            .map(|pipeline| self.bind_draw(pipeline))
            .transpose()?;
        Ok(MainRestoreSource {
            color: self.color,
            depth: self.depth,
            draw,
        })
    }

    fn bind_draw(
        &self,
        pipeline: Arc<MainRestorePipeline>,
    ) -> Result<MainRestoreDraw, RenderError> {
        let sets = [
            self.color.restore_set(pipeline.set_layout)?,
            self.depth.restore_set(pipeline.set_layout)?,
        ];
        Ok(MainRestoreDraw { pipeline, sets })
    }
}

/// Published main images with optional bindings for a lossless restore draw.
pub(crate) struct MainRestoreSource {
    color: Arc<TargetImage>,
    depth: Arc<TargetImage>,
    draw: Option<MainRestoreDraw>,
}

struct MainRestoreDraw {
    pipeline: Arc<MainRestorePipeline>,
    sets: [vk::DescriptorSet; 2],
}

impl MainRestoreSource {
    pub(super) fn allocation_bytes(&self) -> u64 {
        self.color.allocation_bytes() + self.depth.allocation_bytes()
    }

    /// Populate the targets before the immediate layers load them.
    pub(super) fn restore(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        target: &FrameResources,
    ) {
        let images = [target.color.as_ref(), target.depth.as_ref()];
        let saved = [self.color.as_ref(), self.depth.as_ref()];
        if let Some(draw) = &self.draw {
            draw.restore(runtime, command, target, (saved, images));
        } else {
            restore_attachments(runtime, command, (saved, images), target.attachment_extent);
        }
    }
}

fn restore_attachments(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    (saved, images): ([&TargetImage; 2], [&TargetImage; 2]),
    extent: vk::Extent2D,
) {
    apply_barriers(runtime, command, &images.map(to_copy_destination));
    copy_pair(runtime, command, saved, images, extent);
    let ready = images
        .map(|image| load_attachment(image, Layout::TRANSFER_DST_OPTIMAL, Access::TRANSFER_WRITE));
    apply_barriers(runtime, command, &ready);
}

impl MainRestoreDraw {
    fn restore(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        target: &FrameResources,
        (saved, images): ([&TargetImage; 2], [&TargetImage; 2]),
    ) {
        let attach = images.map(|image| super::record::image_barrier(image, false));
        apply_barriers(
            runtime,
            command,
            &[attach, saved.map(lend_to_restore)].concat(),
        );
        begin_restore_rendering(runtime, command, target);
        self.pipeline
            .record(command, self.sets, target.attachment_extent);
        super::record::finish_rendering(runtime, command);
        let reload = images.map(reload_attachment);
        apply_barriers(
            runtime,
            command,
            &[reload, saved.map(return_from_restore)].concat(),
        );
    }
}

/// The colour through its UNORM alias, as the pipeline expects; both targets are overwritten.
fn begin_restore_rendering(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    target: &FrameResources,
) {
    let overwritten = |view| {
        super::record::render_attachment(view, super::record::depth_clear(), false)
            .load_op(vk::AttachmentLoadOp::DONT_CARE)
    };
    let colors = [overwritten(target.color.xor_view)];
    let depth = overwritten(target.depth.view);
    let info = vk::RenderingInfo::default()
        .render_area(vk::Rect2D {
            offset: vk::Offset2D::default(),
            extent: target.attachment_extent,
        })
        .layer_count(1)
        .color_attachments(&colors)
        .depth_attachment(&depth);
    // SAFETY: the command buffer is recording on this thread, outside any rendering scope, and
    // both target views are in `ATTACHMENT_OPTIMAL` for this frame.
    unsafe { runtime.device().cmd_begin_rendering(command, &info) };
}

/// Make the restored attachment writes visible to the immediate layers that load them.
fn reload_attachment(image: &TargetImage) -> vk::ImageMemoryBarrier2<'static> {
    let (stage, access) = super::record::attachment_write_scope(image.aspect);
    barrier(image)
        .old_layout(Layout::ATTACHMENT_OPTIMAL)
        .new_layout(Layout::ATTACHMENT_OPTIMAL)
        .src_stage_mask(stage)
        .src_access_mask(access)
        .dst_stage_mask(ATTACHMENT_STAGES)
        .dst_access_mask(ATTACHMENT_ACCESS)
}

impl MainLayerSnapshot {
    pub(super) fn record(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        frame: &FrameResources,
    ) {
        let rendered = [frame.color.as_ref(), frame.depth.as_ref()];
        let saved = [self.images.color.as_ref(), self.images.depth.as_ref()];
        let before = [
            super::record::image_barrier(rendered[0], true),
            super::record::image_barrier(rendered[1], true),
            to_copy_destination(saved[0]),
            to_copy_destination(saved[1]),
        ];
        apply_barriers(runtime, command, &before);
        copy_pair(runtime, command, rendered, saved, frame.attachment_extent);
        let after = [
            load_attachment(
                rendered[0],
                Layout::TRANSFER_SRC_OPTIMAL,
                Access::TRANSFER_READ,
            ),
            load_attachment(
                rendered[1],
                Layout::TRANSFER_SRC_OPTIMAL,
                Access::TRANSFER_READ,
            ),
            publish_copy(saved[0]),
            publish_copy(saved[1]),
        ];
        apply_barriers(runtime, command, &after);
    }
}

fn copy_pair(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    source: [&TargetImage; 2],
    destination: [&TargetImage; 2],
    extent: vk::Extent2D,
) {
    for (source, destination) in source.into_iter().zip(destination) {
        copy(runtime, command, source, destination, extent);
    }
}

fn to_copy_destination(image: &TargetImage) -> vk::ImageMemoryBarrier2<'static> {
    barrier(image)
        .old_layout(Layout::UNDEFINED)
        .new_layout(Layout::TRANSFER_DST_OPTIMAL)
        .dst_stage_mask(Stage::COPY)
        .dst_access_mask(Access::TRANSFER_WRITE)
}

const ATTACHMENT_STAGES: Stage = Stage::from_raw(
    Stage::COLOR_ATTACHMENT_OUTPUT.as_raw()
        | Stage::EARLY_FRAGMENT_TESTS.as_raw()
        | Stage::LATE_FRAGMENT_TESTS.as_raw(),
);
const ATTACHMENT_ACCESS: Access = Access::from_raw(
    Access::COLOR_ATTACHMENT_READ.as_raw()
        | Access::COLOR_ATTACHMENT_WRITE.as_raw()
        | Access::DEPTH_STENCIL_ATTACHMENT_READ.as_raw()
        | Access::DEPTH_STENCIL_ATTACHMENT_WRITE.as_raw(),
);

fn load_attachment(
    image: &TargetImage,
    from: Layout,
    access: Access,
) -> vk::ImageMemoryBarrier2<'static> {
    barrier(image)
        .old_layout(from)
        .new_layout(Layout::ATTACHMENT_OPTIMAL)
        .src_stage_mask(Stage::COPY)
        .src_access_mask(access)
        .dst_stage_mask(ATTACHMENT_STAGES)
        .dst_access_mask(ATTACHMENT_ACCESS)
}

fn publish_copy(image: &TargetImage) -> vk::ImageMemoryBarrier2<'static> {
    barrier(image)
        .old_layout(Layout::TRANSFER_DST_OPTIMAL)
        .new_layout(Layout::TRANSFER_SRC_OPTIMAL)
        .src_stage_mask(Stage::COPY)
        .src_access_mask(Access::TRANSFER_WRITE)
        .dst_stage_mask(Stage::COPY)
        .dst_access_mask(Access::TRANSFER_READ)
}

/// A published image rests in `TRANSFER_SRC_OPTIMAL`, where captures read it; a restore
/// borrows it for the fetch and gives it back. Earlier users only read it.
fn lend_to_restore(image: &TargetImage) -> vk::ImageMemoryBarrier2<'static> {
    barrier(image)
        .old_layout(Layout::TRANSFER_SRC_OPTIMAL)
        .new_layout(Layout::SHADER_READ_ONLY_OPTIMAL)
        .src_stage_mask(Stage::ALL_TRANSFER | Stage::FRAGMENT_SHADER)
        .dst_stage_mask(Stage::FRAGMENT_SHADER)
        .dst_access_mask(Access::SHADER_SAMPLED_READ)
}

fn return_from_restore(image: &TargetImage) -> vk::ImageMemoryBarrier2<'static> {
    barrier(image)
        .old_layout(Layout::SHADER_READ_ONLY_OPTIMAL)
        .new_layout(Layout::TRANSFER_SRC_OPTIMAL)
        .src_stage_mask(Stage::FRAGMENT_SHADER)
        .dst_stage_mask(Stage::ALL_TRANSFER)
        .dst_access_mask(Access::TRANSFER_READ)
}

fn barrier(image: &TargetImage) -> vk::ImageMemoryBarrier2<'static> {
    vk::ImageMemoryBarrier2::default()
        .image(image.handle)
        .subresource_range(image.range())
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
}

fn apply_barriers(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    barriers: &[vk::ImageMemoryBarrier2<'_>],
) {
    let dependency = vk::DependencyInfo::default().image_memory_barriers(barriers);
    // SAFETY: the recording frame owns each image and these transitions occur outside rendering.
    unsafe { runtime.device().cmd_pipeline_barrier2(command, &dependency) };
}

fn copy(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    source: &TargetImage,
    destination: &TargetImage,
    extent: vk::Extent2D,
) {
    let layers = vk::ImageSubresourceLayers::default()
        .aspect_mask(source.aspect)
        .layer_count(1);
    let region = vk::ImageCopy::default()
        .src_subresource(layers)
        .dst_subresource(layers)
        .extent(vk::Extent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        });
    // SAFETY: the completed source and fresh destination have distinct Arc owners, identical
    // formats and extents, the correct transfer usages and the declared transfer layouts.
    unsafe {
        runtime.device().cmd_copy_image(
            command,
            source.handle,
            Layout::TRANSFER_SRC_OPTIMAL,
            destination.handle,
            Layout::TRANSFER_DST_OPTIMAL,
            &[region],
        );
    }
}

impl super::VulkanRenderer {
    pub(crate) fn allows_main_copy(&self) -> bool {
        self.runtime.window_depth_behavior == Default::default()
            && self.runtime.supports_main_copy(vk::Format::R8G8B8A8_SRGB)
            && self
                .runtime
                .supports_main_copy(self.runtime.depth.formats.plain)
            && (!uses_main_restore_draw(self.runtime.depth.formats.plain)
                || (super::image::supports_sampling(
                    &self.runtime,
                    super::main_restore_pipeline::RESTORE_COLOR_FORMAT,
                ) && super::image::supports_sampling(
                    &self.runtime,
                    self.runtime.depth.formats.plain,
                )))
    }
}

/// D24 can round again when a fetched UNORM value is written to the depth attachment (#596).
pub(super) fn uses_main_restore_draw(depth: vk::Format) -> bool {
    depth == vk::Format::D32_SFLOAT
}

impl VulkanRuntime {
    fn supports_main_copy(&self, format: vk::Format) -> bool {
        // SAFETY: the selected physical device and instance are live for the renderer lifetime.
        let features = unsafe {
            self.instance
                .get_physical_device_format_properties(self.physical, format)
        };
        features
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::TRANSFER_DST)
    }
}

#[cfg(all(test, not(feature = "command-state-diagnostics")))]
#[path = "immediate_main_record_tests.rs"]
mod tests;
