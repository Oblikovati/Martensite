//! Overrides of the input-queueing (zoom, rotation, keyboard, touch, 3D mouse), buffer-transfer
//! and per-frame camera virtuals of `AisViewController`. The trait is separate from `AisViewControllerCameraHooks` only to keep
//! each module small; one object may implement both and be installed in both owners.
use super::{AisInteractiveContext, AisViewController, AisWalkDelta};
use crate::v3d::{V3dTypeOfOrientation, V3dView, V3dViewSource};
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Axis1, Point3, Transform3};
use martensite_tkservice::aspect::{
    AspectScrollDelta, AspectVKey, dispatch_space_mouse_keys, dispatch_space_mouse_rotation,
    dispatch_space_mouse_translation,
};
use martensite_tkservice::wnt::WntHidSpaceMouse;
use std::sync::Arc;

type Ctx = Handle<AisInteractiveContext>;
type View = Handle<dyn V3dViewSource>;

/// Override queueing and frame virtuals; `controller.set_frame_hooks(Arc::new(MyFrameHooks));`.
/// Each default runs the matching `*_base` body, so an override that calls it delegates.
/// The controller's own internal callers (scroll, focus loss, flush, frame and camera actions)
/// dispatch through the installed owner, which stays installed during nested calls.
pub trait AisViewControllerFrameHooks: Send + Sync {
    /// `UpdateZoom`; `hooks.update_zoom(c, &delta)?` queues or merges a zoom by default.
    fn update_zoom(
        &self,
        c: &mut AisViewController,
        delta: &AspectScrollDelta,
    ) -> Result<bool, Failure> {
        Ok(c.update_zoom_base(delta))
    }
    /// `UpdateZRotation`; `hooks.update_z_rotation(c, 0.25)?` queues a touch rotation by default.
    fn update_z_rotation(&self, c: &mut AisViewController, angle: f64) -> Result<bool, Failure> {
        Ok(c.update_z_rotation_base(angle))
    }
    /// `ResetViewInput`; `hooks.reset_view_input(c)?` clears keys and buttons by default.
    fn reset_view_input(&self, c: &mut AisViewController) -> Result<(), Failure> {
        c.reset_view_input_base();
        Ok(())
    }
    /// `UpdateViewOrientation`; `hooks.update_view_orientation(c, orientation, true)?`.
    fn update_view_orientation(
        &self,
        c: &mut AisViewController,
        orientation: V3dTypeOfOrientation,
        to_fit_all: bool,
    ) -> Result<(), Failure> {
        c.update_view_orientation_base(orientation, to_fit_all);
        Ok(())
    }
    /// `flushBuffers`; `hooks.flush_buffers(c, &ctx, &view)?` moves the UI queue by default.
    fn flush_buffers(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.flush_buffers_base(ctx, view);
        Ok(())
    }
    /// `flushGestures`; `hooks.flush_gestures(c, &ctx, &view)?` synthesizes touch gestures by default.
    fn flush_gestures(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.flush_gestures_base(ctx, view)
    }
    /// `FitAllAuto`; `hooks.fit_all_auto(c, &ctx, &view)?` fits selection or scene by default.
    fn fit_all_auto(
        &self,
        c: &mut AisViewController,
        ctx: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<(), Failure> {
        c.fit_all_auto_base(ctx, view)
    }
    /// `PickAxis`; `hooks.pick_axis(c, &mut point, &ctx, &view, axis)?` picks along the axis by default.
    fn pick_axis(
        &self,
        c: &mut AisViewController,
        point: &mut Point3,
        ctx: &AisInteractiveContext,
        view: &V3dView,
        axis: Axis1,
    ) -> Result<bool, Failure> {
        c.pick_axis_base(point, ctx, view, axis)
    }
    /// `handlePanning`; `hooks.handle_panning(c, &view)?` applies the queued pan by default.
    fn handle_panning(&self, c: &mut AisViewController, view: &V3dView) -> Result<(), Failure> {
        c.handle_panning_base(view)
    }
    /// `handleZRotate`; `hooks.handle_z_rotate(c, &view)?` applies the queued Z rotation by default.
    fn handle_z_rotate(&self, c: &mut AisViewController, view: &V3dView) -> Result<(), Failure> {
        c.handle_z_rotate_base(view)
    }
    /// `handleOrbitRotation`; `hooks.handle_orbit_rotation(c, &view, &pivot, false)?`.
    fn handle_orbit_rotation(
        &self,
        c: &mut AisViewController,
        view: &V3dView,
        pivot: &Point3,
        lock_z_up: bool,
    ) -> Result<(), Failure> {
        c.handle_orbit_rotation_base(view, pivot, lock_z_up)
    }
    /// `handleViewRotation`; `hooks.handle_view_rotation(c, &view, yaw, pitch, roll, false)?`.
    fn handle_view_rotation(
        &self,
        c: &mut AisViewController,
        view: &V3dView,
        yaw: f64,
        pitch: f64,
        roll: f64,
        restart: bool,
    ) -> Result<(), Failure> {
        c.handle_view_rotation_base(view, yaw, pitch, roll, restart)
    }
    /// `handleViewOrientationKeys`; `hooks.handle_view_orientation_keys(c, &ctx, &view)?`.
    fn handle_view_orientation_keys(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.handle_view_orientation_keys_base(ctx, view)
    }
    /// `handleViewRedraw`; `hooks.handle_view_redraw(c, &ctx, &view)?` redraws by default.
    fn handle_view_redraw(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.handle_view_redraw_base(ctx, view)
    }
    /// `handleXRInput`; `hooks.handle_xr_input(c, &ctx, &view, &walk)?`.
    fn handle_xr_input(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
        walk: &AisWalkDelta,
    ) -> Result<(), Failure> {
        c.handle_xr_input_base(ctx, view, walk)
    }
    /// `handleXRPresentations`; `hooks.handle_xr_presentations(c, &ctx, &view)?`.
    fn handle_xr_presentations(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.handle_xr_presentations_base(ctx, view)
    }
    /// `KeyDown`; `hooks.key_down(c, key, time, pressure)?` presses the cached key by default.
    fn key_down(
        &self,
        c: &mut AisViewController,
        key: AspectVKey,
        time: f64,
        pressure: f64,
    ) -> Result<(), Failure> {
        c.key_down_base(key, time, pressure)
    }
    /// `KeyUp`; `hooks.key_up(c, key, time)?` releases the cached key by default.
    fn key_up(&self, c: &mut AisViewController, key: AspectVKey, time: f64) -> Result<(), Failure> {
        c.key_up_base(key, time)
    }
    /// `KeyFromAxis`; `hooks.key_from_axis(c, negative, positive, time, pressure)?`.
    fn key_from_axis(
        &self,
        c: &mut AisViewController,
        negative: AspectVKey,
        positive: AspectVKey,
        time: f64,
        pressure: f64,
    ) -> Result<(), Failure> {
        c.key_from_axis_base(negative, positive, time, pressure)
    }
    /// `AddTouchPoint`; `hooks.add_touch_point(c, id, point, clear_before)?`.
    fn add_touch_point(
        &self,
        c: &mut AisViewController,
        id: usize,
        point: Vec2<f64>,
        clear_before: bool,
    ) -> Result<(), Failure> {
        c.add_touch_point_base(id, point, clear_before)
    }
    /// `RemoveTouchPoint`; `hooks.remove_touch_point(c, id, false)?` returns whether one was removed.
    fn remove_touch_point(
        &self,
        c: &mut AisViewController,
        id: usize,
        clear_points: bool,
    ) -> Result<bool, Failure> {
        c.remove_touch_point_base(id, clear_points)
    }
    /// `UpdateTouchPoint`; `hooks.update_touch_point(c, id, point)?` adds an unknown id by default.
    fn update_touch_point(
        &self,
        c: &mut AisViewController,
        id: usize,
        point: Vec2<f64>,
    ) -> Result<(), Failure> {
        c.update_touch_point_base(id, point)
    }
    /// `Update3dMouse`; `hooks.update_3d_mouse(c, &report)?` runs the three handlers by default.
    fn update_3d_mouse(
        &self,
        c: &mut AisViewController,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        c.update_3d_mouse_base(event)
    }
    /// Inherited `update3dMouseTranslation`; `hooks.update_3d_mouse_translation(c, &report)?`.
    fn update_3d_mouse_translation(
        &self,
        c: &mut AisViewController,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        dispatch_space_mouse_translation(c, event)
    }
    /// Inherited `update3dMouseRotation`; `hooks.update_3d_mouse_rotation(c, &report)?`.
    fn update_3d_mouse_rotation(
        &self,
        c: &mut AisViewController,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        dispatch_space_mouse_rotation(c, event)
    }
    /// Inherited `update3dMouseKeys`; `hooks.update_3d_mouse_keys(c, &report)?`.
    fn update_3d_mouse_keys(
        &self,
        c: &mut AisViewController,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        dispatch_space_mouse_keys(c, event)
    }
    /// `handleCameraActions`; `hooks.handle_camera_actions(c, &ctx, &view, &walk)?` runs every
    /// queued camera request (orientation, pan, rotations, zoom, focus) by default.
    fn handle_camera_actions(
        &self,
        c: &mut AisViewController,
        ctx: &AisInteractiveContext,
        view: &View,
        walk: &AisWalkDelta,
    ) -> Result<(), Failure> {
        c.handle_camera_actions_base(ctx, view, walk)
    }
    /// `handleXRTurnPad`; `hooks.handle_xr_turn_pad(c, &ctx, &view)?` turns on a side trackpad click.
    fn handle_xr_turn_pad(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.handle_xr_turn_pad_base(ctx, view)
    }
    /// `handleXRTeleport`; `hooks.handle_xr_teleport(c, &ctx, &view)?` aims and teleports by default.
    fn handle_xr_teleport(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.handle_xr_teleport_base(ctx, view)
    }
    /// `handleXRPicking`; `hooks.handle_xr_picking(c, &ctx, &view)?` highlights and selects by default.
    fn handle_xr_picking(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.handle_xr_picking_base(ctx, view)
    }
    /// `handleXRHighlight`; `hooks.handle_xr_highlight(c, &ctx, &view)?` highlights the picking hand ray.
    fn handle_xr_highlight(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        c.handle_xr_highlight_base(ctx, view)
    }
    /// `handleXRMoveTo`; `hooks.handle_xr_move_to(c, &ctx, &view, &pose, true)?` returns 1 on a hit.
    fn handle_xr_move_to(
        &self,
        c: &mut AisViewController,
        ctx: &Ctx,
        view: &View,
        pose: &Transform3,
        to_highlight: bool,
    ) -> Result<i32, Failure> {
        c.handle_xr_move_to_base(ctx, view, pose, to_highlight)
    }
}

impl AisViewController {
    /// Install queueing/frame overrides; `controller.set_frame_hooks(Arc::new(MyFrameHooks));`.
    pub fn set_frame_hooks(&mut self, hooks: Arc<dyn AisViewControllerFrameHooks>) {
        // Keep the owner installed: a callback may reenter or unwind before returning.
        self.dispatch.frame_hooks = Some(hooks);
    }
    /// Restore base dispatch and return the owner; `let prior = controller.take_frame_hooks();`.
    pub fn take_frame_hooks(&mut self) -> Option<Arc<dyn AisViewControllerFrameHooks>> {
        self.dispatch.frame_hooks.take()
    }
    /// Clone the installed owner for one dispatch; a running callback keeps its own `Arc`.
    pub(super) fn frame_hooks(&self) -> Option<Arc<dyn AisViewControllerFrameHooks>> {
        self.dispatch.frame_hooks.clone()
    }
}
