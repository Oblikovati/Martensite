//! A redraw submits its frame and returns; the frame's owners wait here until its fence
//! signals. The maintainer's error-timing decision of 2026-11-22 (PLAN.md, phase 2) lets a
//! late GPU fault surface at the next redraw that needs the frame's slot (#594).
use super::{VulkanRenderer, frame::FrameResources, submission::CommandSubmission};
use crate::RenderError;
use std::collections::VecDeque;
use std::sync::{Mutex, PoisonError};

/// Frames the GPU may run while the host prepares the next one.
pub(super) const FRAMES_IN_FLIGHT: usize = 2;

/// One submitted frame. Its commands drop first, so their fence wait comes before the
/// owners are released.
pub(super) struct InFlightFrame {
    pub commands: CommandSubmission,
    pub resources: FrameResources,
}

/// The submitted frames of one device, oldest first. The mutex only keeps the renderer
/// `Sync`: every access holds `&mut` and takes no lock.
#[derive(Default)]
pub(super) struct FrameFlight(Mutex<VecDeque<InFlightFrame>>);

impl FrameFlight {
    fn frames(&mut self) -> &mut VecDeque<InFlightFrame> {
        self.0.get_mut().unwrap_or_else(PoisonError::into_inner)
    }
    /// The number of frames not yet settled; `flight.len()`.
    pub fn len(&mut self) -> usize {
        self.frames().len()
    }
}

impl VulkanRenderer {
    /// Wait for the oldest frame while the flight is full; `renderer.make_flight_room()?` returns
    /// the late fault of a settled frame. A finished frame is not settled early, so the
    /// resources a redraw creates do not depend on GPU timing.
    pub(super) fn make_flight_room(&mut self) -> Result<(), RenderError> {
        while self.flight.len() >= FRAMES_IN_FLIGHT {
            self.settle_oldest_frame()?;
        }
        Ok(())
    }
    /// Keep a submitted frame until its fence signals, and let later frames draw its
    /// uploads; `renderer.launch_frame(frame)`.
    pub(super) fn launch_frame(&mut self, mut frame: InFlightFrame) {
        frame.resources.geometry_transfers.residency.share();
        frame.resources.retain_submitted_draws();
        self.flight.frames().push_back(frame);
    }
    /// Wait for every submitted frame, as `glFinish` does; `renderer.finish_frames()?` settles
    /// all of them and returns the first late fault.
    pub(crate) fn finish_frames(&mut self) -> Result<(), RenderError> {
        let mut first = Ok(());
        while self.flight.len() > 0 {
            let outcome = self.settle_oldest_frame();
            first = first.and(outcome);
        }
        first
    }
    /// Release every frame once the owning close proved the device idle; `renderer.release_flight_after_idle()`.
    pub(super) fn release_flight_after_idle(&mut self) {
        for frame in self.flight.frames().drain(..) {
            frame.commands.qualify_device_idle();
        }
    }
    fn settle_oldest_frame(&mut self) -> Result<(), RenderError> {
        let Some(frame) = self.flight.frames().pop_front() else {
            return Ok(());
        };
        let outcome = frame.commands.complete();
        self.settle_owned_frame(frame.commands, frame.resources, outcome)
            .map(drop)
    }
}

#[cfg(test)]
#[path = "frame_flight_tests.rs"]
mod tests;
