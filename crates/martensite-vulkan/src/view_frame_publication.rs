//! Publication owns the last result and checks the accepted composition epoch.
use super::redraw_completion::{RedrawPublicationTarget, RedrawTargetStamp, RetiredRedraw};
use super::*;
use crate::backend::CapturePlanes;
impl Graphic3dRenderView {
    /// Read the last frame (color and depth) or its error; `render_view.capture()?` never conceals a failed redraw.
    pub fn capture(&self) -> Result<FrameCapture, RenderError> {
        self.with_capture(CapturePlanes::ColorAndDepth, FrameCapture::clone)
    }
    /// Read only the color plane of the last frame on demand, skipping depth readback (#501);
    /// `render_view.capture_color()?`.
    pub fn capture_color(&self) -> Result<FrameCapture, RenderError> {
        self.with_capture(CapturePlanes::Color, FrameCapture::clone)
    }
    /// Borrow the last frame's `planes` without copying them;
    /// `view.with_capture(CapturePlanes::Color, |frame| frame.width)?`.
    /// A dump copies straight from this borrow instead of cloning the colour and depth (#594).
    pub(crate) fn with_capture<R>(
        &self,
        planes: CapturePlanes,
        read: impl FnOnce(&FrameCapture) -> R,
    ) -> Result<R, RenderError> {
        let publication = self.capture_publication(Some(planes))?;
        publication.planes(planes, &self.device).map(read)
    }
    /// Dump the last frame into `image`: an RGB dump of a deferred frame reads packed rows
    /// from the device (#594), every other dump copies from the borrowed capture;
    /// `view.dump_publication(&image, &Graphic3dBufferType::Rgb)?`.
    pub(crate) fn dump_publication(
        &self,
        image: &martensite_tkservice::image::ImagePixMap,
        kind: &martensite_tkservice::graphic3d::Graphic3dBufferType,
    ) -> Result<bool, RenderError> {
        let rgb = crate::view_capture::is_direct_rgb_dump(image, kind);
        let planes = (!rgb).then(|| crate::view_capture::dump_planes(kind));
        let publication = self.capture_publication(planes)?;
        if rgb
            && let Some(copied) =
                crate::view_capture::dump_rgb_rows_from(image, &publication, &self.device)
        {
            return copied;
        }
        let planes = crate::view_capture::dump_planes(kind);
        let capture = publication.planes(planes, &self.device)?;
        Ok(crate::view_capture::dump_capture(image, kind, capture))
    }
    pub(crate) fn capture_publication(&self, planes: Option<CapturePlanes>) -> FramePublication {
        self.require_capture_lighting()?;
        if self.checked_gui_policy() && self.gui_layers.read().capture_stamp().extent.is_some() {
            return self.checked_gui_publication();
        }
        self.legacy_gui_publication(planes)
    }
    fn legacy_gui_publication(&self, planes: Option<CapturePlanes>) -> FramePublication {
        self.require_capture_target()?;
        let extent = self.gui_capture_extent()?;
        let gui = self.gui_layers.read();
        if let Some(extent) = extent {
            gui.snapshot(extent)?;
        }
        let outcome = self
            .read_capture_outcome(planes)
            .unwrap_or_else(|| Err(rejected("no redraw", "a successfully rendered view")));
        self.require_capture_target()?;
        outcome
    }
    fn read_capture_outcome(&self, planes: Option<CapturePlanes>) -> Option<FramePublication> {
        #[cfg(feature = "command-state-diagnostics")]
        use std::sync::atomic::Ordering;
        let source = self.outcome.read();
        #[cfg(feature = "command-state-diagnostics")]
        let serial = self.redraw_serial.load(Ordering::Relaxed);
        let snapshot = source.clone();
        drop(source);
        // The first capture of a redraw reads its frame from the device (#594), before the
        // second target check, as the cloning capture did; the result stays cached. An RGB dump
        // passes no planes, because it may read the device rows itself.
        if let (Some(Ok(capture)), Some(planes)) = (&snapshot, planes) {
            let _ = capture.planes(planes, &self.device);
        }
        #[cfg(feature = "command-state-diagnostics")]
        {
            let output = snapshot.as_ref().map(|result| {
                let capture = result.as_ref().map_err(Clone::clone)?;
                capture.pixels(&self.device).cloned()
            });
            crate::scene::replay_diagnostic::cloned(
                self as *const Self as usize,
                serial,
                output.as_ref().and_then(|result| result.as_ref().ok()),
                &output,
            );
        }
        snapshot
    }
    #[cfg(any(
        feature = "raytrace-input-diagnostics",
        feature = "command-state-diagnostics"
    ))]
    /// Dump an owned copy of the frame so the diagnostics can observe it after the copy.
    pub(super) fn observed_buffer_dump(
        &self,
        image: &martensite_tkservice::image::ImagePixMap,
        kind: &martensite_tkservice::graphic3d::Graphic3dBufferType,
    ) -> Result<bool, RenderError> {
        let observed_serial = self
            .redraw_serial
            .load(std::sync::atomic::Ordering::Relaxed);
        let capture = self.capture()?;
        // Diagnostic builds read each frame as it renders, so this dump copies from the
        // capture just taken, as before; it shares the dump path of other builds (#594).
        let copied = self.dump_publication(image, kind)?;
        #[cfg(feature = "raytrace-input-diagnostics")]
        self.observe_raytrace_buffer_dump(observed_serial, kind, &capture, copied);
        #[cfg(feature = "command-state-diagnostics")]
        crate::scene::replay_diagnostic::dumped(
            self as *const Self as usize,
            [
                observed_serial,
                self.redraw_serial
                    .load(std::sync::atomic::Ordering::Relaxed),
            ],
            &format!("{kind:?}"),
            copied,
            &capture,
        );
        Ok(copied)
    }
    /// Render the current view and retain errors for void Graphic3d callbacks; `render_view.try_redraw()?`.
    /// Ordinary window frames use deferred readback tickets (`#594`, `#501`) and do not perform
    /// CPU color/depth capture unless [`Self::capture`] or [`Self::capture_color`] is called.
    pub fn try_redraw(&self) -> Result<(), RenderError> {
        self.try_redraw_mode(false)
    }
    /// Present one checked interactive frame without mandatory CPU color/depth readback (#501);
    /// `render_view.try_present()?`.
    ///
    /// Preserves `RenderGuiSurfacePolicy::CheckedV2` surface/GUI generation validation and
    /// error reporting; call [`Self::capture_color`] or [`Self::capture`] when a fresh-frame
    /// screenshot or depth readback is explicitly requested.
    pub fn try_present(&self) -> Result<(), RenderError> {
        self.try_redraw()
    }
    pub(super) fn try_redraw_mode(&self, immediate: bool) -> Result<(), RenderError> {
        if !immediate {
            self.immediate_main.discard();
        }
        let target = self.redraw_target_stamp();
        let stamp = self.gui_layers.read().target_stamp();
        if self.checked_gui_policy() && stamp.extent.is_some() {
            return self.checked_gui_redraw(stamp, target);
        }
        let epoch = self.gui_layers.read().epoch.clone();
        let outcome = self.render_current(immediate);
        let result = self.publish_gui_outcome(&epoch, outcome, target);
        if result.is_err() {
            self.immediate_main.discard();
        }
        result
    }
    pub(super) fn publish_gui_outcome(
        &self,
        epoch: &std::sync::Arc<()>,
        outcome: Result<RedrawCapture, RenderError>,
        target: RedrawTargetStamp,
    ) -> Result<(), RenderError> {
        let publication = RedrawPublicationTarget {
            started: target,
            published: self.redraw_target_stamp(),
        };
        let mut retired = RetiredRedraw::default();
        let result = self.commit_gui_outcome(epoch, outcome, &publication, &mut retired);
        drop(retired);
        result
    }
    fn commit_gui_outcome(
        &self,
        epoch: &std::sync::Arc<()>,
        outcome: Result<RedrawCapture, RenderError>,
        publication: &RedrawPublicationTarget,
        retired: &mut RetiredRedraw,
    ) -> Result<(), RenderError> {
        let mut gui = self.gui_layers.write();
        gui.last_draw = None;
        let outcome = self.check_publication_target(&gui.epoch, epoch, outcome);
        let result = self.publish_redraw_observation(outcome, publication, retired);
        self.update_gui_redraw_status(result.is_ok());
        result
    }
    pub(super) fn update_gui_redraw_status(&self, succeeded: bool) {
        if succeeded {
            self.retained.redraw();
        } else {
            self.retained.invalidate();
        }
    }
}
