//! Capture follows the actual generated subview window instead of its replaced standalone window.
use super::*;
use martensite_tkernel::ncollection::Vec2;
impl ViewerSession {
    /// Bind the active view and retain its actual generated window; `session.bind_subview(&parent, [0.5, 1.], [0., 0.])?`.
    pub fn bind_subview(
        &mut self,
        parent: &Handle<dyn V3dViewSource>,
        size: [f64; 2],
        offset: [f64; 2],
    ) -> Result<(), Failure> {
        let view = self.view().clone();
        attach_relative_subview(&view, parent, size, offset)?;
        self.views[self.active].0 = generated_subview_window(&view)?;
        view.redraw();
        Ok(())
    }
    /// Set the actual active controller and redraw; `session.set_camera_rotation_mode(AisRotationMode::PickLast)`.
    pub fn set_camera_rotation_mode(&mut self, mode: martensite_tkv3d::ais::AisRotationMode) {
        self.controller.set_rotation_mode(mode);
        self.view().redraw();
    }
    /// Observe the actual active controller mode; `session.camera_rotation_mode()`.
    pub fn camera_rotation_mode(&self) -> martensite_tkv3d::ais::AisRotationMode {
        self.controller.rotation_mode()
    }
}
fn attach_relative_subview(
    view: &Handle<dyn V3dViewSource>,
    parent: &Handle<dyn V3dViewSource>,
    size: [f64; 2],
    offset: [f64; 2],
) -> Result<(), Failure> {
    view.view_base().set_subview_window(
        parent,
        Vec2::new(size[0], size[1]),
        AspectTypeOfTriedronPosition::LEFT_UPPER,
        Vec2::new(offset[0], offset[1]),
        Vec2::new(0, 0),
    )
}
fn generated_subview_window(
    view: &Handle<dyn V3dViewSource>,
) -> Result<Handle<AspectNeutralWindow>, Failure> {
    let actual = view.window().cast::<AspectNeutralWindow>();
    if actual.is_null() {
        return Err(martensite_tkernel::standard::exceptions::program_error(
            &format!(
                "subview window {:?}; expected actual generated neutral window",
                view.window().object_address()
            ),
        ));
    }
    Ok(actual)
}
