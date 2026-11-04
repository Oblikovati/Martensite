//! The keyboard, touch and 3D-mouse virtuals of `AisViewController` (and the inherited
//! `AspectWindowInputListener` 3D-mouse handlers) dispatch through the installed
//! `AisViewControllerFrameHooks`, so an override intercepts the controller's own calls:
//! `update_3d_mouse` -> translation/rotation/keys handlers -> `key_from_axis`, `key_down` and
//! `key_up`; `add_touch_point(.., true)` -> `remove_touch_point`; `update_touch_point` of a new
//! id -> `add_touch_point`. Without an owner each runs its `*_base` body. Native witness:
//! the recorded controller_input_slots capture.
use super::AisViewController;
use martensite_tkernel::{ncollection::Vec2, standard::Failure};
use martensite_tkservice::{
    aspect::{
        AspectVKey, dispatch_space_mouse_keys, dispatch_space_mouse_rotation,
        dispatch_space_mouse_translation,
    },
    wnt::WntHidSpaceMouse,
};

impl AisViewController {
    /// Virtual `KeyDown`; `controller.key_down(key, 0.5, 1.0)?`.
    pub fn key_down(&mut self, key: AspectVKey, time: f64, pressure: f64) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.key_down(self, key, time, pressure),
            None => self.key_down_base(key, time, pressure),
        }
    }
    /// Virtual `KeyUp`; `controller.key_up(key, 1.0)?`.
    pub fn key_up(&mut self, key: AspectVKey, time: f64) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.key_up(self, key, time),
            None => self.key_up_base(key, time),
        }
    }
    /// Virtual `KeyFromAxis`; `controller.key_from_axis(left, right, 0.5, -0.25)?`.
    pub fn key_from_axis(
        &mut self,
        negative: AspectVKey,
        positive: AspectVKey,
        time: f64,
        pressure: f64,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.key_from_axis(self, negative, positive, time, pressure),
            None => self.key_from_axis_base(negative, positive, time, pressure),
        }
    }
    /// Virtual `AddTouchPoint`; `controller.add_touch_point(1, point, false)?`.
    pub fn add_touch_point(
        &mut self,
        id: usize,
        point: Vec2<f64>,
        clear_before: bool,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.add_touch_point(self, id, point, clear_before),
            None => self.add_touch_point_base(id, point, clear_before),
        }
    }
    /// Virtual `RemoveTouchPoint`; `let removed = controller.remove_touch_point(1, false)?;`.
    pub fn remove_touch_point(
        &mut self,
        id: usize,
        clear_select_points: bool,
    ) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.remove_touch_point(self, id, clear_select_points),
            None => self.remove_touch_point_base(id, clear_select_points),
        }
    }
    /// Virtual `UpdateTouchPoint`; `controller.update_touch_point(1, point)?`.
    pub fn update_touch_point(&mut self, id: usize, point: Vec2<f64>) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_touch_point(self, id, point),
            None => self.update_touch_point_base(id, point),
        }
    }
}

impl AisViewController {
    /// Virtual `Update3dMouse`; `let changed = controller.update_3d_mouse(&report)?;`.
    pub fn update_3d_mouse(&mut self, event: &WntHidSpaceMouse<'_>) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_3d_mouse(self, event),
            None => self.update_3d_mouse_base(event),
        }
    }
    /// Inherited virtual `update3dMouseTranslation`; `controller.update_3d_mouse_translation(&report)?`.
    pub fn update_3d_mouse_translation(
        &mut self,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_3d_mouse_translation(self, event),
            None => dispatch_space_mouse_translation(self, event),
        }
    }
    /// Inherited virtual `update3dMouseRotation`; `controller.update_3d_mouse_rotation(&report)?`.
    pub fn update_3d_mouse_rotation(
        &mut self,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_3d_mouse_rotation(self, event),
            None => dispatch_space_mouse_rotation(self, event),
        }
    }
    /// Inherited virtual `update3dMouseKeys`; `controller.update_3d_mouse_keys(&report)?`.
    pub fn update_3d_mouse_keys(&mut self, event: &WntHidSpaceMouse<'_>) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_3d_mouse_keys(self, event),
            None => dispatch_space_mouse_keys(self, event),
        }
    }
}
