//! Lighting revisions protect model frames independently of GUI target and composition epochs.
use super::*;
impl Graphic3dRenderView {
    pub(crate) fn invalidate_lighting_capture(&self) {
        *self.outcome.write() = Some(Err(rejected(
            "lighting selection changed",
            "a redraw under the current lighting selection",
        )));
        self.retained.invalidate();
    }
    pub(super) fn require_capture_lighting(&self) -> Result<(), RenderError> {
        let selection = self.lighting_selection.read();
        let target = self.redraw_target.read();
        let Some(target) = target.as_ref() else {
            return Ok(());
        };
        selection.require_current(&target.started.lighting)
    }
}
