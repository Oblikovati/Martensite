//! The mouse and frame-selection virtuals of `AisViewController` dispatch through the installed
//! shared hooks, so an override intercepts the controller's own internal calls (click from a
//! button release, buttons from a modifier change, move-to from the frame, lazy move-to from
//! picking and highlighting). Each hook default runs the matching `_base` body. Native witness:
//! the recorded controller_event_slots capture.
use super::{AisInteractiveContext, AisViewController};
use crate::v3d::V3dViewSource;
use martensite_tkernel::{
    ncollection::Vec2,
    standard::{Failure, Handle},
};
use martensite_tkservice::aspect::{AspectVKeyFlags, AspectVKeyMouse};

impl AisViewController {
    /// Virtual `UpdateMouseClick`; `controller.update_mouse_click(point, button, flags, false)?`.
    pub fn update_mouse_click(
        &mut self,
        point: Vec2<i32>,
        button: AspectVKeyMouse,
        modifiers: AspectVKeyFlags,
        is_double_click: bool,
    ) -> Result<bool, Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.update_mouse_click_base(point, button, modifiers, is_double_click);
        };
        hooks.update_mouse_click(self, point, button, modifiers, is_double_click)
    }
    /// Virtual `UpdateMouseButtons`; `controller.update_mouse_buttons(point, buttons, flags, false)?`.
    pub fn update_mouse_buttons(
        &mut self,
        point: Vec2<i32>,
        buttons: AspectVKeyMouse,
        modifiers: AspectVKeyFlags,
        emulated: bool,
    ) -> Result<bool, Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.update_mouse_buttons_base(point, buttons, modifiers, emulated);
        };
        hooks.update_mouse_buttons(self, point, buttons, modifiers, emulated)
    }
    /// Virtual `UpdateMousePosition`; `controller.update_mouse_position(point, buttons, flags, false)?`.
    pub fn update_mouse_position(
        &mut self,
        point: Vec2<i32>,
        buttons: AspectVKeyMouse,
        modifiers: AspectVKeyFlags,
        emulated: bool,
    ) -> Result<bool, Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.update_mouse_position_base(point, buttons, modifiers, emulated);
        };
        hooks.update_mouse_position(self, point, buttons, modifiers, emulated)
    }
    /// Virtual `handleMoveTo`; `controller.handle_move_to(&context, &view)?`.
    pub fn handle_move_to(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.handle_move_to_base(context, view);
        };
        hooks.handle_move_to(self, context, view)
    }
    /// Virtual `handleSelectionPick`; `controller.handle_selection_pick(&context, &view)?`.
    pub fn handle_selection_pick(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.handle_selection_pick_base(context, view);
        };
        hooks.handle_selection_pick(self, context, view)
    }
    /// Virtual `handleDynamicHighlight`; `controller.handle_dynamic_highlight(&context, &view)?`.
    pub fn handle_dynamic_highlight(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.handle_dynamic_highlight_base(context, view);
        };
        hooks.handle_dynamic_highlight(self, context, view)
    }
    /// Virtual `handleSelectionPoly`; `controller.handle_selection_poly(&context, &view)?`.
    pub fn handle_selection_poly(
        &mut self,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.handle_selection_poly_base(context, view);
        };
        hooks.handle_selection_poly(self, context, view)
    }
    /// Virtual `contextLazyMoveTo`; `controller.context_lazy_move_to(&context, &view, &pixel)?`.
    pub fn context_lazy_move_to(
        &mut self,
        context: &AisInteractiveContext,
        view: &Handle<dyn V3dViewSource>,
        pixel: &Vec2<i32>,
    ) -> Result<(), Failure> {
        let Some(hooks) = self.dispatch.camera_hooks.clone() else {
            return self.context_lazy_move_to_base(context, view, pixel);
        };
        hooks.context_lazy_move_to(self, context, view, pixel)
    }
}
