//! Publication owns the last result and checks the accepted composition epoch.
use super::redraw_completion::{RedrawPublicationTarget, RedrawTargetStamp};
use super::*;
impl Graphic3dRenderView {
    /// Read the last frame or its error; `render_view.capture()?` never conceals a failed redraw.
    pub fn capture(&self) -> Result<FrameCapture, RenderError> {
        self.require_capture_lighting()?;
        if self.checked_gui_policy() && self.gui_layers.read().capture_stamp().extent.is_some() {
            return self.checked_gui_capture();
        }
        self.legacy_gui_capture()
    }
    fn legacy_gui_capture(&self) -> Result<FrameCapture, RenderError> {
        self.require_capture_target()?;
        let extent = self.gui_capture_extent()?;
        let gui = self.gui_layers.read();
        if let Some(extent) = extent {
            gui.snapshot(extent)?;
        }
        let outcome = self
            .clone_capture_outcome()
            .unwrap_or_else(|| Err(rejected("no redraw", "a successfully rendered view")));
        self.require_capture_target()?;
        outcome
    }
    fn clone_capture_outcome(&self) -> Option<Result<FrameCapture, RenderError>> {
        let source = self.outcome.read();
        let output = source.clone();
        #[cfg(feature = "command-state-diagnostics")]
        crate::scene::replay_diagnostic::cloned(
            self as *const Self as usize,
            self.redraw_serial
                .load(std::sync::atomic::Ordering::Relaxed),
            &source,
            &output,
        );
        output
    }
    /// Render the current view and retain errors for void Graphic3d callbacks; `render_view.try_redraw()?`.
    pub fn try_redraw(&self) -> Result<(), RenderError> {
        let target = self.redraw_target_stamp();
        let stamp = self.gui_layers.read().target_stamp();
        if self.checked_gui_policy() && stamp.extent.is_some() {
            return self.checked_gui_redraw(stamp, target);
        }
        let epoch = self.gui_layers.read().epoch.clone();
        let outcome = self.render_current();
        self.publish_gui_outcome(&epoch, outcome, target)
    }
    pub(super) fn publish_gui_outcome(
        &self,
        epoch: &std::sync::Arc<()>,
        outcome: Result<FrameCapture, RenderError>,
        target: RedrawTargetStamp,
    ) -> Result<(), RenderError> {
        let publication = RedrawPublicationTarget {
            started: target,
            published: self.redraw_target_stamp(),
        };
        let mut retired = None;
        let result = self.commit_gui_outcome(epoch, outcome, &publication, &mut retired);
        drop(retired);
        result
    }
    fn commit_gui_outcome(
        &self,
        epoch: &std::sync::Arc<()>,
        outcome: Result<FrameCapture, RenderError>,
        publication: &RedrawPublicationTarget,
        retired: &mut Option<RedrawPublicationTarget>,
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
