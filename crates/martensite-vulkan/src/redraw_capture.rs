//! A published redraw keeps its frame on the device until a caller asks for pixels (#594).
//!
//! OCCT's `Redraw` neither waits for the GPU nor reads the frame back, so a Vulkan redraw
//! publishes the frame's images and the host steps of its capture. The first `pixels` call
//! reads the images once; later calls return the same pixels.
use crate::driver_backend::{RenderedFrame, SharedRenderDevice};
use crate::view_capture::HostCaptureSteps;
use crate::{FrameCapture, RenderError, backend::FrameReadback, error::rejected};
use std::sync::{Mutex, OnceLock, PoisonError};

/// One published redraw; `capture.pixels(&view.device)?` returns its frame.
pub(crate) struct RedrawCapture {
    size: [u32; 2],
    pixels: OnceLock<Result<FrameCapture, RenderError>>,
    deferred: Mutex<Option<DeferredPixels>>,
}

/// The device images of a redraw and the host steps its capture still applies.
struct DeferredPixels {
    readback: FrameReadback,
    steps: HostCaptureSteps,
}

impl RedrawCapture {
    /// The result of one redraw; `RedrawCapture::new(rendered, steps)` applies the host steps
    /// at once to pixels already on the host, and later to device images.
    pub fn new(rendered: RenderedFrame, steps: HostCaptureSteps) -> Self {
        // The size is the one the backend produced, which the redraw cursor checks.
        let (size, pixels, deferred) = match rendered {
            RenderedFrame::Ready(mut capture) => {
                steps.apply(&mut capture);
                (
                    [capture.width, capture.height],
                    OnceLock::from(Ok(capture)),
                    None,
                )
            }
            RenderedFrame::Deferred(readback) => {
                let size = readback.logical_size();
                (
                    size,
                    OnceLock::new(),
                    Some(DeferredPixels { readback, steps }),
                )
            }
        };
        Self {
            size,
            pixels,
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
        let mut capture = device.write().read_frame(&readback)?;
        let ledger = device.read().work_ledger();
        let _readback = ledger
            .as_ref()
            .map(|ledger| ledger.enter(crate::backend::WorkPhase::Readback));
        steps.apply(&mut capture);
        Ok(capture)
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
