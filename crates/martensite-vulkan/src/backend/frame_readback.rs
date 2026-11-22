//! A completed frame keeps its colour and depth images until a caller asks for pixels (#594).
//!
//! A redraw no longer copies its attachments to host memory. The view keeps this ticket,
//! which pins the attachment generation it names, and `capture` records the copy on demand.
use super::{
    VulkanRenderer, WorkPhase, attachment_coordinates::AttachmentCoordinates, buffer::DeviceBuffer,
    image::TargetImage, memory_selection::HostBufferAccess, runtime::VulkanRuntime,
    submission::CommandSubmission,
};
use crate::{FrameCapture, RenderError};
use ash::vk;
use std::sync::Arc;

/// The images of one completed frame; `renderer.read_frame(&readback)?` copies them out.
#[derive(Clone)]
pub(crate) struct FrameReadback {
    pub(super) color: Arc<TargetImage>,
    pub(super) depth: Arc<TargetImage>,
    /// The logical frame extent of the capture.
    extent: vk::Extent2D,
    /// The attachment extent the copies read (see `AttachmentCoordinates::leading_rows`).
    attachment_extent: vk::Extent2D,
    coordinates: AttachmentCoordinates,
}

/// The host buffers one readback copies the colour and depth attachments into.
pub(super) struct ReadbackBuffers {
    pub color: DeviceBuffer,
    pub depth: DeviceBuffer,
}

impl FrameReadback {
    /// The ticket of a frame whose images are left in `TRANSFER_SRC_OPTIMAL`.
    pub(super) fn new(
        [color, depth]: [Arc<TargetImage>; 2],
        [extent, attachment_extent]: [vk::Extent2D; 2],
        coordinates: AttachmentCoordinates,
    ) -> Self {
        Self {
            color,
            depth,
            extent,
            attachment_extent,
            coordinates,
        }
    }
    /// The width and height of the capture this ticket reads.
    pub(crate) fn logical_size(&self) -> [u32; 2] {
        [self.extent.width, self.extent.height]
    }
    /// The bytes of one attachment copy; both attachments use four bytes per texel.
    fn attachment_bytes(&self) -> usize {
        self.attachment_extent.width as usize * self.attachment_extent.height as usize * 4
    }
    fn buffers(&self, runtime: &Arc<VulkanRuntime>) -> Result<ReadbackBuffers, RenderError> {
        let bytes = self.attachment_bytes() as u64;
        let buffer = || {
            DeviceBuffer::new(
                runtime,
                bytes,
                vk::BufferUsageFlags::TRANSFER_DST,
                HostBufferAccess::Readback,
            )
        };
        Ok(ReadbackBuffers {
            color: buffer()?,
            depth: buffer()?,
        })
    }
    fn record(&self, runtime: &VulkanRuntime, command: vk::CommandBuffer, into: &ReadbackBuffers) {
        super::record::record_attachment_reads(
            runtime,
            command,
            [&self.color, &self.depth],
            [into.color.handle, into.depth.handle],
            self.attachment_extent,
        );
    }
    fn decode(&self, buffers: &ReadbackBuffers) -> Result<FrameCapture, RenderError> {
        let size = self.attachment_bytes();
        let rgba = self.logical_capture_rows(buffers.color.read(size)?)?;
        let depth_bytes = self.logical_capture_rows(buffers.depth.read(size)?)?;
        let depth = super::depth_equivalence::depth_plane(
            self.depth.format,
            depth_bytes.as_chunks::<4>().0,
        );
        let capture = FrameCapture {
            width: self.extent.width,
            height: self.extent.height,
            rgba,
            depth,
        };
        #[cfg(feature = "command-state-diagnostics")]
        self.observe_readback(&capture, buffers);
        Ok(capture)
    }
    /// Crop the leading attachment rows, then orient the logical rows top-left.
    fn logical_capture_rows(&self, bytes: Vec<u8>) -> Result<Vec<u8>, RenderError> {
        let bytes = self.coordinates.logical_rows(bytes, self.extent);
        self.coordinates.capture_rows(bytes, self.extent.width)
    }
    #[cfg(feature = "command-state-diagnostics")]
    fn observe_readback(&self, capture: &FrameCapture, buffers: &ReadbackBuffers) {
        use crate::scene::replay_diagnostic::{self, ReadbackArguments};
        use ash::vk::Handle;
        let color = [self.color.handle.as_raw(), self.color.view.as_raw()];
        let depth = [self.depth.handle.as_raw(), self.depth.view.as_raw()];
        let arguments = ReadbackArguments {
            color: [color[0], color[1], buffers.color.handle.as_raw()],
            depth: [depth[0], depth[1], buffers.depth.handle.as_raw()],
            formats: [self.color.format.as_raw(), self.depth.format.as_raw()],
            samples: [self.color.samples.as_raw(), self.depth.samples.as_raw()],
            attachment_extent: [self.attachment_extent.width, self.attachment_extent.height],
            leading_rows: self.coordinates.leading_rows(self.extent.height),
            coordinates: self.coordinates,
        };
        replay_diagnostic::readback(capture, arguments);
    }
}

impl VulkanRenderer {
    /// Copy a completed frame's images to the host; `renderer.read_frame(&readback)?`.
    pub(crate) fn read_frame(
        &mut self,
        readback: &FrameReadback,
    ) -> Result<FrameCapture, RenderError> {
        self.runtime.lifecycle.admit("read frame")?;
        let outcome = self.read_admitted_frame(readback);
        self.runtime.lifecycle.observe(outcome)
    }
    fn read_admitted_frame(&self, readback: &FrameReadback) -> Result<FrameCapture, RenderError> {
        let _readback = self.runtime.work.enter(WorkPhase::Readback);
        let buffers = readback.buffers(&self.runtime)?;
        let commands = CommandSubmission::new(&self.runtime)?;
        readback.record(&self.runtime, commands.command, &buffers);
        if let Err(primary) = commands.finish() {
            return Err(self.keep_failed_readback(commands, readback, buffers, primary));
        }
        readback.decode(&buffers)
    }
    /// Keep the copy's owners while its completion is unknown; returns the primary failure.
    fn keep_failed_readback(
        &self,
        commands: CommandSubmission,
        readback: &FrameReadback,
        buffers: ReadbackBuffers,
        primary: RenderError,
    ) -> RenderError {
        if !commands.retirement_incomplete() {
            return primary;
        }
        self.retirements
            .keep_readback(commands, readback.clone(), buffers, primary)
    }
}

#[cfg(test)]
#[path = "frame_readback_tests.rs"]
mod tests;
