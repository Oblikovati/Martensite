//! Camera overrides retain their owner during nested dispatch and failure unwinding.
use super::AisSelectionScheme;
use super::{AisInteractiveContext, AisViewController, AisWalkDelta};
use crate::v3d::{V3dView, V3dViewSource};
use martensite_tkernel::{
    ncollection::{Sequence, Vec2},
    standard::{Failure, Handle},
};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::{AspectScrollDelta, AspectVKeyFlags, AspectVKeyMouse};
use std::sync::Arc;

/// Override camera actions without removing the installed owner during callbacks.
/// Example: `controller.set_camera_hooks(Arc::new(MyCameraHooks));`.
/// Callbacks borrow themselves immutably so nested dispatch can use the same owner;
/// applications can keep callback state in their own synchronized fields.
pub trait AisViewControllerCameraHooks: Send + Sync {
    /// Retain existing Box callbacks by default; `hooks.on_selection_changed(c, ctx, view)?`.
    fn on_selection_changed(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        controller.on_selection_changed_legacy(context, view)
    }
    /// Retain existing drag callbacks/default body; `hooks.on_object_dragged(c, ctx, view, action)?`.
    fn on_object_dragged(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        action: super::AisDragAction,
    ) -> Result<(), Failure> {
        controller.on_object_dragged_legacy(context, view, action)
    }
    /// Retain existing subview callbacks; `hooks.on_subview_changed(c, ctx, old, new)?`.
    fn on_subview_changed(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        old_view: &Handle<dyn V3dViewSource>,
        new_view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        controller.on_subview_changed_legacy(context, old_view, new_view)
    }
    /// Queue one pick with the base body by default; `hooks.select_in_viewer(c, point, scheme)?`.
    fn select_in_viewer(
        &self,
        controller: &mut AisViewController,
        point: Vec2<i32>,
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        controller.select_in_viewer_base(point, scheme);
        Ok(())
    }
    /// Queue a point sequence with the base body by default; `hooks.select_points_in_viewer(c, &points, scheme)?`.
    fn select_points_in_viewer(
        &self,
        controller: &mut AisViewController,
        points: &Sequence<Vec2<i32>>,
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        controller.select_points_in_viewer_base(points, scheme);
        Ok(())
    }
    /// Replace rectangle corners with the base body by default; `hooks.update_rubber_band(c, from, to)?`.
    fn update_rubber_band(
        &self,
        controller: &mut AisViewController,
        first: Vec2<i32>,
        second: Vec2<i32>,
    ) -> Result<(), Failure> {
        controller.update_rubber_band_base(first, second);
        Ok(())
    }
    /// Edit the lasso polygon with the base body by default; `hooks.update_poly_selection(c, point, true)?`.
    fn update_poly_selection(
        &self,
        controller: &mut AisViewController,
        point: Vec2<i32>,
        append: bool,
    ) -> Result<(), Failure> {
        controller.update_poly_selection_base(point, append);
        Ok(())
    }
    /// Run the native `update_mouse_click` base body by default; `hooks.update_mouse_click(c, ...)?`.
    fn update_mouse_click(
        &self,
        controller: &mut AisViewController,
        point: Vec2<i32>,
        buttons: AspectVKeyMouse,
        modifiers: AspectVKeyFlags,
        emulated: bool,
    ) -> Result<bool, Failure> {
        controller.update_mouse_click_base(point, buttons, modifiers, emulated)
    }
    /// Run the native `update_mouse_buttons` base body by default; `hooks.update_mouse_buttons(c, ...)?`.
    fn update_mouse_buttons(
        &self,
        controller: &mut AisViewController,
        point: Vec2<i32>,
        buttons: AspectVKeyMouse,
        modifiers: AspectVKeyFlags,
        emulated: bool,
    ) -> Result<bool, Failure> {
        controller.update_mouse_buttons_base(point, buttons, modifiers, emulated)
    }
    /// Run the native `update_mouse_position` base body by default; `hooks.update_mouse_position(c, ...)?`.
    fn update_mouse_position(
        &self,
        controller: &mut AisViewController,
        point: Vec2<i32>,
        buttons: AspectVKeyMouse,
        modifiers: AspectVKeyFlags,
        emulated: bool,
    ) -> Result<bool, Failure> {
        controller.update_mouse_position_base(point, buttons, modifiers, emulated)
    }
    /// Run the native `handle_move_to` base body by default; `hooks.handle_move_to(c, ...)?`.
    fn handle_move_to(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        controller.handle_move_to_base(context, view)
    }
    /// Run the native `handle_selection_pick` base body by default; `hooks.handle_selection_pick(c, ...)?`.
    fn handle_selection_pick(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        controller.handle_selection_pick_base(context, view)
    }
    /// Run the native `handle_dynamic_highlight` base body by default; `hooks.handle_dynamic_highlight(c, ...)?`.
    fn handle_dynamic_highlight(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        controller.handle_dynamic_highlight_base(context, view)
    }
    /// Run the native `handle_selection_poly` base body by default; `hooks.handle_selection_poly(c, ...)?`.
    fn handle_selection_poly(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        controller.handle_selection_poly_base(context, view)
    }
    /// Run the native `context_lazy_move_to` base body by default; `hooks.context_lazy_move_to(c, ...)?`.
    fn context_lazy_move_to(
        &self,
        controller: &mut AisViewController,
        context: &AisInteractiveContext,
        view: &Handle<dyn V3dViewSource>,
        pixel: &Vec2<i32>,
    ) -> Result<(), Failure> {
        controller.context_lazy_move_to_base(context, view, pixel)
    }
    /// Keep the original orbit-pivot policy by default; `hooks.gravity_point(c, ctx, view)?`.
    fn gravity_point(
        &self,
        controller: &mut AisViewController,
        context: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<Point3, Failure> {
        controller.gravity_point_base(context, view)
    }
    /// Delegate real navigation by default; `hooks.handle_navigation_keys(c, ctx, view)?`.
    fn handle_navigation_keys(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<AisWalkDelta, Failure> {
        controller.handle_navigation_keys_base(context, view)
    }
    /// Delegate one complete frame by default; `hooks.handle_view_events(c, ctx, view)?`.
    fn handle_view_events(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        controller.handle_view_events_base(context, view)
    }
    /// Delegate UI transfer and its actual flag; `hooks.flush_view_events(c, ctx, view, false)?`.
    fn flush_view_events(
        &self,
        controller: &mut AisViewController,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        to_handle: bool,
    ) -> Result<(), Failure> {
        controller.flush_view_events_base(context, view, to_handle)
    }
    /// Delegate to the original zoom by default; `hooks.handle_zoom(c, view, scroll, None)?`.
    fn handle_zoom(
        &self,
        controller: &mut AisViewController,
        view: &V3dView,
        scroll: &AspectScrollDelta,
        hit: Option<&Point3>,
    ) -> Result<(), Failure> {
        controller.handle_zoom_base(view, scroll, hit)
    }
    /// Delegate to the immutable focus primitive; `hooks.handle_z_focus_scroll(c, view, scroll)?`.
    fn handle_z_focus_scroll(
        &self,
        controller: &mut AisViewController,
        view: &V3dView,
        scroll: &AspectScrollDelta,
    ) -> Result<(), Failure> {
        controller.handle_z_focus_scroll(view, scroll)
    }
    /// Delegate to actual selector picking; `hooks.pick_point(c, out, ctx, view, pixel, true)?`.
    fn pick_point(
        &self,
        controller: &mut AisViewController,
        point: &mut Point3,
        context: &AisInteractiveContext,
        view: &V3dView,
        cursor: &Vec2<i32>,
        stick_to_pick_ray: bool,
    ) -> Result<bool, Failure> {
        controller.pick_point_base(point, context, view, cursor, stick_to_pick_ray)
    }
}

impl AisViewController {
    /// Install shared camera overrides; `controller.set_camera_hooks(Arc::new(MyCameraHooks));`.
    pub fn set_camera_hooks(&mut self, hooks: Arc<dyn AisViewControllerCameraHooks>) {
        // Keep the owner installed: a callback may reenter or unwind before returning.
        self.dispatch.camera_hooks = Some(hooks);
    }
    /// Restore base camera dispatch and return its owner; `let prior = controller.take_camera_hooks();`.
    pub fn take_camera_hooks(&mut self) -> Option<Arc<dyn AisViewControllerCameraHooks>> {
        // An already running callback retains its own Arc until its frame unwinds.
        self.dispatch.camera_hooks.take()
    }
    /// Dispatch mutable focus callbacks; `controller.dispatch_z_focus_scroll(view, scroll)?`.
    /// The existing immutable `handle_z_focus_scroll` remains the base primitive.
    pub fn dispatch_z_focus_scroll(
        &mut self,
        view: &V3dView,
        scroll: &AspectScrollDelta,
    ) -> Result<(), Failure> {
        if let Some(hooks) = self.dispatch.camera_hooks.clone() {
            return hooks.handle_z_focus_scroll(self, view, scroll);
        }
        self.handle_z_focus_scroll(view, scroll)
    }
}
