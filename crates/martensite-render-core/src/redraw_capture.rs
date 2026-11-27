//! A published redraw keeps its frame on the device until a caller asks for pixels (#594).
//!
//! OCCT's `Redraw` neither waits for the GPU nor reads the frame back, so a Vulkan redraw
//! publishes the frame's images and the host steps of its capture. The first `pixels` call
//! reads the images once; later calls return the same pixels.
use crate::device_backend::{CapturePlanes, RenderDeferredFrame, RenderedFrame};
use crate::driver_backend::SharedRenderDevice;
use crate::frame_work_ledger::WorkPhase;
use crate::view_capture::HostCaptureSteps;
use crate::{FrameCapture, RenderError, error::rejected};
use std::sync::{Mutex, OnceLock, PoisonError};

/// One published redraw; `capture.pixels(&view.device)?` returns its frame.
pub struct RedrawCapture {
    size: [u32; 2],
    pub assembly_streaming: Option<crate::RenderAssemblyStreamingProgress>,
    pixels: OnceLock<Result<FrameCapture, RenderError>>,
    /// The colour of a frame whose depth no caller read yet; its depth stays empty.
    color: OnceLock<Result<FrameCapture, RenderError>>,
    deferred: Mutex<Option<DeferredPixels>>,
}

/// The device images of a redraw and the host steps its capture still applies.
struct DeferredPixels {
    readback: Box<dyn RenderDeferredFrame>,
    steps: HostCaptureSteps,
}

impl RedrawCapture {
    /// The result of one redraw; `RedrawCapture::new(rendered, steps)` applies the host steps
    /// at once to pixels already on the host, and later to device images.
    pub fn new(rendered: RenderedFrame, steps: HostCaptureSteps) -> Self {
        // The size is the one the backend produced, which the redraw cursor checks.
        let (size, pixels, deferred, assembly_streaming) = match rendered {
            RenderedFrame::Ready(mut capture) => {
                steps.apply(&mut capture);
                (
                    [capture.width, capture.height],
                    OnceLock::from(Ok(capture)),
                    None,
                    None,
                )
            }
            RenderedFrame::Deferred(readback) => {
                let size = readback.logical_size();
                let progress = readback.assembly_streaming();
                (
                    size,
                    OnceLock::new(),
                    Some(DeferredPixels { readback, steps }),
                    progress,
                )
            }
        };
        Self {
            size,
            assembly_streaming,
            pixels,
            color: OnceLock::new(),
            deferred: Mutex::new(deferred),
        }
    }
    /// The logical width and height of the frame, known without reading it.
    pub fn size(&self) -> [u32; 2] {
        self.size
    }
    /// The frame's pixels, read from the device on the first call; `capture.pixels(&device)?`.
    /// The caller must not hold the device lock.
    pub fn pixels(&self, device: &SharedRenderDevice) -> Result<&FrameCapture, RenderError> {
        let pixels = self.pixels.get_or_init(|| self.read(device));
        pixels.as_ref().map_err(Clone::clone)
    }
    /// The frame's pixels for `planes`; `capture.planes(CapturePlanes::Color, &device)?`.
    /// A colour read leaves the depth on the device and returns an empty depth plane, unless
    /// the full frame was already read (#594). The caller must not hold the device lock.
    pub fn planes(
        &self,
        planes: CapturePlanes,
        device: &SharedRenderDevice,
    ) -> Result<&FrameCapture, RenderError> {
        if planes == CapturePlanes::ColorAndDepth || self.pixels.get().is_some() {
            return self.pixels(device);
        }
        let color = self.color.get_or_init(|| self.read_color(device));
        color.as_ref().map_err(Clone::clone)
    }
    /// Gives unread device images back to the next frame; their pixels can no longer be read.
    /// A view calls this only when a new redraw starts, because that redraw always replaces
    /// this publication; `capture.release_unread()`.
    pub fn release_unread(&self) {
        if self.pixels.get().is_none() {
            let unread = self
                .deferred
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            drop(unread);
        }
    }
    fn read(&self, device: &SharedRenderDevice) -> Result<FrameCapture, RenderError> {
        let mut deferred = self.deferred.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(DeferredPixels { readback, steps }) = deferred.take() else {
            return Err(rejected(
                "a frame released for a later redraw",
                "the frame of the last published redraw",
            ));
        };
        let mut capture = device
            .write()
            .read_frame(readback.as_ref(), CapturePlanes::ColorAndDepth)?;
        let ledger = device.read().work_ledger();
        let _readback = ledger
            .as_ref()
            .map(|ledger| ledger.enter(WorkPhase::Readback));
        steps.apply(&mut capture);
        Ok(capture)
    }
}

impl RedrawCapture {
    /// Read the colour only and keep the images for a later full read.
    fn read_color(&self, device: &SharedRenderDevice) -> Result<FrameCapture, RenderError> {
        let deferred = self.deferred.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(DeferredPixels { readback, steps }) = deferred.as_ref() else {
            // The full read holds this lock while it reads, so its pixels are already set.
            return self.color_of_full_read();
        };
        let mut capture = device
            .write()
            .read_frame(readback.as_ref(), CapturePlanes::Color)?;
        let ledger = device.read().work_ledger();
        let _readback = ledger
            .as_ref()
            .map(|ledger| ledger.enter(WorkPhase::Readback));
        steps.apply_color(&mut capture);
        Ok(capture)
    }
    fn color_of_full_read(&self) -> Result<FrameCapture, RenderError> {
        let Some(full) = self.pixels.get() else {
            return Err(rejected(
                "a frame released for a later redraw",
                "the frame of the last published redraw",
            ));
        };
        let full = full.as_ref().map_err(Clone::clone)?;
        Ok(FrameCapture {
            depth: Vec::new(),
            ..full.clone()
        })
    }
}

impl RedrawCapture {
    /// Write the frame's RGB rows through `write_row` straight from the device, or `None`
    /// when the frame is already on the host, was released, or has subview blits to compose;
    /// the caller then dumps from `planes`; `capture.dump_rgb(&device, &mut write_row)`.
    pub fn dump_rgb(
        &self,
        device: &SharedRenderDevice,
        write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
    ) -> Option<Result<bool, RenderError>> {
        if self.pixels.get().is_some() || self.color.get().is_some() {
            return None;
        }
        let deferred = self.deferred.lock().unwrap_or_else(PoisonError::into_inner);
        let DeferredPixels { readback, steps } = deferred.as_ref()?;
        if !steps.subview_blits.is_empty() {
            return None;
        }
        let flip = steps.flip_output;
        Some(
            device
                .write()
                .read_frame_rgb(readback.as_ref(), flip, write_row),
        )
    }
}

/// Pending deferred readback payload returned by [`RedrawCapture::take_for_async_readback`].
#[doc(hidden)]
pub enum AsyncReadbackState {
    /// Host capture already materialized.
    Ready(FrameCapture),
    /// Device-resident deferred frame plus host post-processing steps.
    Deferred(
        Box<dyn RenderDeferredFrame>,
        crate::view_capture::HostCaptureSteps,
    ),
}

impl RedrawCapture {
    /// Access the cached host capture, or take the pending deferred ticket plus host steps
    /// for asynchronous readback (used by `martensite-webgpu` `capture_async`).
    #[doc(hidden)]
    pub fn take_for_async_readback(&self) -> Result<AsyncReadbackState, RenderError> {
        if let Some(cached) = self.pixels.get() {
            return cached.clone().map(AsyncReadbackState::Ready);
        }
        let mut deferred = self.deferred.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(DeferredPixels { readback, steps }) = deferred.take() else {
            return Err(rejected(
                "a frame released for a later redraw",
                "the frame of the last published redraw",
            ));
        };
        Ok(AsyncReadbackState::Deferred(readback, steps))
    }

    /// Store the completed async capture so subsequent synchronous/async reads reuse it.
    #[doc(hidden)]
    pub fn complete_async_readback(&self, result: Result<FrameCapture, RenderError>) {
        let _ = self.pixels.set(result);
    }
}

impl std::fmt::Debug for RedrawCapture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RedrawCapture")
            .field("size", &self.size)
            .field("read", &self.pixels.get().is_some())
            .finish()
    }
}

/// Tests publish host pixels as an injected backend would; `Ok(capture.into())`.
#[cfg(test)]
impl From<FrameCapture> for RedrawCapture {
    fn from(capture: FrameCapture) -> Self {
        let steps = HostCaptureSteps {
            size: [capture.width, capture.height],
            scaled: None,
            subview_blits: Vec::new(),
            flip_output: false,
        };
        Self::new(RenderedFrame::Ready(capture), steps)
    }
}

#[cfg(test)]
#[path = "redraw_capture_tests.rs"]
mod tests;
