//! The depth range of the active view: automatic z-fit and an explicit camera z range.
use super::*;

impl ViewerSession {
    /// The active view's automatic depth fit, keeping its scale factor, then
    /// redraw; `session.set_auto_z_fit(false)?`.
    pub fn set_auto_z_fit(&self, enabled: bool) -> Result<(), Failure> {
        let view = self.view().view_base();
        view.set_auto_z_fit_mode(enabled, view.auto_z_fit_scale_factor())?;
        view.redraw();
        Ok(())
    }

    /// The camera's depth range, then redraw; the caller keeps
    /// `near < far`; `session.set_z_range(1., 512.)?`.
    pub fn set_z_range(&self, near: f64, far: f64) -> Result<(), Failure> {
        let view = self.view().view_base();
        let camera = view.camera();
        let camera = camera.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(
                "the active view has no camera; expected a live V3d view",
            )
        })?;
        camera.set_z_range(near, far)?;
        view.redraw();
        Ok(())
    }
}
