//! Copy a completed main color and depth pair before loading the immediate layers.
use super::{frame::FrameResources, image::TargetImage, runtime::VulkanRuntime};
use crate::RenderFrame;
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
    if let (Some(plan), Some(ticket)) = (&frame.immediate_main, ticket) {
        plan.seed().remember(ticket.clone());
    }
}

impl MainLayerImages {
    pub(super) fn new(color: Arc<TargetImage>, depth: Arc<TargetImage>) -> Self {
        Self { color, depth }
    }

    pub(super) fn allocation_bytes(&self) -> u64 {
        self.color.allocation_bytes() + self.depth.allocation_bytes()
    }

    pub(super) fn restore(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        target: &FrameResources,
    ) {
        let images = [target.color.as_ref(), target.depth.as_ref()];
        let before = images.map(to_copy_destination);
        apply_barriers(runtime, command, &before);
        copy_pair(
            runtime,
            command,
            [&self.color, &self.depth],
            images,
            target.attachment_extent,
        );
        let after = images.map(|image| {
            load_attachment(image, Layout::TRANSFER_DST_OPTIMAL, Access::TRANSFER_WRITE)
        });
        apply_barriers(runtime, command, &after);
    }
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
            super::record::image_barrier(rendered[0], true, false),
            super::record::image_barrier(rendered[1], true, false),
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
        .dst_stage_mask(
            Stage::COLOR_ATTACHMENT_OUTPUT
                | Stage::EARLY_FRAGMENT_TESTS
                | Stage::LATE_FRAGMENT_TESTS,
        )
        .dst_access_mask(
            Access::COLOR_ATTACHMENT_READ
                | Access::COLOR_ATTACHMENT_WRITE
                | Access::DEPTH_STENCIL_ATTACHMENT_READ
                | Access::DEPTH_STENCIL_ATTACHMENT_WRITE,
        )
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
    }
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
