//! The explicit wait for the frames that redraws left on the GPU, like `glFinish` (#594).
use crate::{Graphic3dRenderDriver, RenderError};

impl Graphic3dRenderDriver {
    /// Waits until every frame submitted on this device has finished, as `glFinish` does,
    /// and returns the first GPU fault that one of them reported. A redraw returns before
    /// its frame finishes, so call this to end a timed loop or to see a late fault now.
    /// Example: `view.redraw(); driver.finish_frames()?;`.
    pub fn finish_frames(&self) -> Result<(), RenderError> {
        self.device.write().finish_frames()
    }
}
