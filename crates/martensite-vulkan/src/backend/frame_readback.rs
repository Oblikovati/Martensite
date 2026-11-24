//! A completed frame keeps its colour and depth images until a caller asks for pixels (#594).
//!
//! A redraw no longer copies its attachments to host memory. The view keeps this ticket,
//! which pins the attachment generation it names, and `capture` records the copy on demand.
use super::{
    VulkanRenderer, WorkPhase, attachment_coordinates::AttachmentCoordinates, buffer::DeviceBuffer,
    image::TargetImage, runtime::VulkanRuntime, submission::CommandSubmission,
};
use crate::{FrameCapture, RenderError};
use ash::vk;
use std::sync::Arc;

/// The images of one completed frame; `renderer.read_frame(&readback)?` copies them out.
#[derive(Clone)]
pub(crate) struct FrameReadback {
    pub(crate) assembly_streaming: Option<crate::RenderAssemblyStreamingProgress>,
    pub(super) color: Arc<TargetImage>,
    pub(super) depth: Arc<TargetImage>,
    /// The logical frame extent of the capture.
    extent: vk::Extent2D,
    /// The attachment extent the copies read (see `AttachmentCoordinates::leading_rows`).
    attachment_extent: vk::Extent2D,
    coordinates: AttachmentCoordinates,
}

/// The attachments one readback copies; a colour dump skips the depth copy (#594).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CapturePlanes {
    /// Colour and depth, as `capture` returns them.
    ColorAndDepth,
    /// Colour only; the capture's depth stays empty.
    Color,
}

/// The host buffers one readback copies the colour and, when read, depth attachments into.
pub(super) struct ReadbackBuffers {
    pub color: DeviceBuffer,
    pub depth: Option<DeviceBuffer>,
}

impl FrameReadback {
    /// The ticket of a frame whose images are left in `TRANSFER_SRC_OPTIMAL`.
    pub(super) fn new(
        [color, depth]: [Arc<TargetImage>; 2],
        [extent, attachment_extent]: [vk::Extent2D; 2],
        coordinates: AttachmentCoordinates,
    ) -> Self {
        Self {
            assembly_streaming: None,
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
    /// The pack rows of an RGB dump of this frame, top-left first unless `flip` (the view's
    /// flipped output); `readback.rgb_rows(false)`.
    pub(super) fn rgb_rows(&self, flip: bool) -> super::rgb_pack::RgbPackRows {
        let bottom_up = self.coordinates.bottom_up_capture_rows();
        super::rgb_pack::RgbPackRows {
            width: self.extent.width,
            height: self.extent.height,
            first_row: self.coordinates.leading_rows(self.extent.height),
            reversed: u32::from(bottom_up != flip),
        }
    }
    /// The attachment extent the copies read; `readback.attachment_extent()`.
    pub(super) fn attachment_extent(&self) -> vk::Extent2D {
        self.attachment_extent
    }
    /// The bytes of one attachment copy; both attachments use four bytes per texel.
    pub(super) fn attachment_bytes(&self) -> usize {
        self.attachment_extent.width as usize * self.attachment_extent.height as usize * 4
    }
    fn buffers(
        &self,
        runtime: &Arc<VulkanRuntime>,
        planes: CapturePlanes,
        cache: &super::readback_buffer_cache::ReadbackBufferCache,
    ) -> Result<ReadbackBuffers, RenderError> {
        let bytes = self.attachment_bytes() as u64;
        Ok(ReadbackBuffers {
            color: cache.take(runtime, bytes)?,
            depth: match planes {
                CapturePlanes::ColorAndDepth => Some(
                    super::readback_buffer_cache::new_readback_buffer(runtime, bytes)?,
                ),
                CapturePlanes::Color => None,
            },
        })
    }
    fn record(&self, runtime: &VulkanRuntime, command: vk::CommandBuffer, into: &ReadbackBuffers) {
        let color = (&*self.color, into.color.handle);
        let depth = into
            .depth
            .as_ref()
            .map(|buffer| (&*self.depth, buffer.handle));
        super::record::record_attachment_reads(
            runtime,
            command,
            [Some(color), depth],
            self.attachment_extent,
        );
    }
    fn decode(&self, buffers: &ReadbackBuffers) -> Result<FrameCapture, RenderError> {
        let size = self.attachment_bytes();
        let rgba = self.logical_capture_rows(buffers.color.read(size)?)?;
        let depth = match &buffers.depth {
            Some(buffer) => self.decode_depth(buffer, size)?,
            None => Vec::new(),
        };
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
    fn decode_depth(&self, buffer: &DeviceBuffer, size: usize) -> Result<Vec<f32>, RenderError> {
        let bytes = self.logical_capture_rows(buffer.read(size)?)?;
        Ok(super::depth_equivalence::depth_plane(
            self.depth.format,
            bytes.as_chunks::<4>().0,
        ))
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
            depth: [
                depth[0],
                depth[1],
                buffers
                    .depth
                    .as_ref()
                    .map_or(0, |buffer| buffer.handle.as_raw()),
            ],
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
        self.read_frame_planes(readback, CapturePlanes::ColorAndDepth)
    }
    /// Copy the chosen images of a completed frame to the host;
    /// `renderer.read_frame_planes(&readback, CapturePlanes::Color)?` leaves depth empty.
    pub(crate) fn read_frame_planes(
        &mut self,
        readback: &FrameReadback,
        planes: CapturePlanes,
    ) -> Result<FrameCapture, RenderError> {
        self.runtime.lifecycle.admit("read frame")?;
        // The thread may hold the frame being read; settling also reports its error (#594).
        self.settle_presentation()?;
        let outcome = self.read_admitted_frame(readback, planes);
        self.runtime.lifecycle.observe(outcome)
    }
    fn read_admitted_frame(
        &self,
        readback: &FrameReadback,
        planes: CapturePlanes,
    ) -> Result<FrameCapture, RenderError> {
        let _readback = self.runtime.work.enter(WorkPhase::Readback);
        let buffers = readback.buffers(&self.runtime, planes, &self.readback_buffers)?;
        let commands = CommandSubmission::new(&self.runtime)?;
        readback.record(&self.runtime, commands.command, &buffers);
        if let Err(primary) = commands.finish() {
            return Err(self.keep_failed_readback(commands, readback, buffers, primary));
        }
        let capture = readback.decode(&buffers);
        // The copy completed, so the colour buffer can serve the next readback.
        self.readback_buffers.give(buffers.color);
        capture
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
