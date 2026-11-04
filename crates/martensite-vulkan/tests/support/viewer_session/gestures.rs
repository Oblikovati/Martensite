//! Interactive viewer input settings: view rotation in a 2D view and mouse gestures.
use super::*;
use martensite_tkv3d::ais::AisMouseGesture;

impl ViewerSession {
    /// The viewer's input no longer rotates the view; `session.set_2d_input()`.
    pub fn set_2d_input(&mut self) {
        self.controller.set_allow_rotation(false);
    }

    /// Bind the mouse button mask
    /// to the gesture; `session.bind_mouse_gesture(ASPECT_VKEY_MOUSE_LEFT_BUTTON, AisMouseGesture::Pan)`.
    pub fn bind_mouse_gesture(&mut self, button: u32, gesture: AisMouseGesture) {
        self.controller
            .change_mouse_gesture_map()
            .bind(button, gesture);
    }
}
