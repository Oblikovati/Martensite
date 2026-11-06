//! Shared-owner overrides of the queueing, frame, keyboard, touch and 3D-mouse virtuals, as in
//! the recorded controller_frame_slots capture and martensite_controller_input_slots.
//! Each witness records only its own slots; the others pass through to the base unrecorded.
use super::view_controller_slot_trace::{SlotTrace, Via, bits, dir_bits, point_bits};
use super::*;
use crate::v3d::{V3dTypeOfOrientation, V3dView, V3dViewSource};
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Axis1, Point3, Transform3};
use martensite_tkservice::aspect::{AspectScrollDelta, AspectVKey};
use martensite_tkservice::wnt::WntHidSpaceMouse;
use std::sync::Mutex;

/// Slots of the frame witness, in its `SLOT_NAMES` order.
pub(super) const FRAME_SLOTS: &[&str] = &[
    "zoom",
    "zrotation",
    "reset",
    "orientation",
    "buffers",
    "gestures",
    "fitauto",
    "axis",
    "panning",
    "zrotate",
    "orbit",
    "viewrotation",
    "orientkeys",
    "redraw",
    "xrinput",
    "xrpresentations",
];
/// Slots of the input witness, in its `SLOT_NAMES` order.
pub(super) const INPUT_SLOTS: &[&str] = &[
    "keydown",
    "keyup",
    "keyaxis",
    "addtouch",
    "removetouch",
    "updatetouch",
    "mouse3d",
    "translation",
    "rotation",
    "keys3d",
];
/// Slots of the XR handler witness, in its `SLOT_NAMES` order.
pub(super) const XR_SLOTS: &[&str] = &[
    "xrturnpad",
    "xrteleport",
    "xrpicking",
    "xrhighlight",
    "xrmoveto",
];
/// Slots of the camera-action witness, in its `SLOT_NAMES` order.
pub(super) const CAMERA_SLOTS: &[&str] = &[
    "cameraactions",
    "handlezoom",
    "focus",
    "pickpoint",
    "gravity",
    "navigation",
    "handleevents",
    "flushevents",
];
/// The witness text of a walk: defined/empty flags, then forward value and pressure bits.
pub(super) fn walk_text(walk: &AisWalkDelta) -> String {
    let forward = &walk[AisWalkTranslation::Forward];
    let flags = format!("{}{}", walk.is_defined() as i32, walk.is_empty() as i32);
    format!("{flags},{},{}", bits(forward.value), bits(forward.pressure))
}
/// Records the witness slots of one case against the scene context and view.
pub(super) struct FrameSlotRecorder {
    pub(super) trace: SlotTrace,
    pub(super) context: Handle<AisInteractiveContext>,
    pub(super) view: Handle<dyn V3dViewSource>,
    pub(super) axis_point: Mutex<Point3>,
}
impl FrameSlotRecorder {
    pub(super) fn new(
        trace: SlotTrace,
        context: Handle<AisInteractiveContext>,
        view: Handle<dyn V3dViewSource>,
    ) -> Self {
        let axis_point = Mutex::new(Point3::new(1.0, 2.0, 3.0));
        Self {
            trace,
            context,
            view,
            axis_point,
        }
    }
    pub(super) fn same(
        &self,
        context: &AisInteractiveContext,
        view: &Handle<dyn V3dViewSource>,
    ) -> String {
        let same = std::ptr::eq(context, &*self.context)
            && view.object_address() == self.view.object_address();
        (same as i32).to_string()
    }
    pub(super) fn same_view(&self, view: &V3dView) -> String {
        (std::ptr::eq(view, self.view.view_base()) as i32).to_string()
    }
    pub(super) fn same_actual(&self, context: &AisInteractiveContext, view: &V3dView) -> String {
        let same =
            std::ptr::eq(context, &*self.context) && std::ptr::eq(view, self.view.view_base());
        (same as i32).to_string()
    }
}
/// `all` for the clear-every-touch id, else the decimal id, as the input witness writes it.
fn touch_id(id: usize) -> String {
    if id == usize::MAX {
        "all".to_string()
    } else {
        id.to_string()
    }
}
fn touch_args(id: usize, point: Vec2<f64>) -> String {
    format!("{},{},{}", touch_id(id), bits(point.x), bits(point.y))
}
fn report_args(event: &WntHidSpaceMouse<'_>) -> String {
    let flags = [
        event.is_translation(),
        event.is_rotation(),
        event.is_key_state(),
    ];
    flags
        .iter()
        .map(|flag| (*flag as i32).to_string())
        .collect()
}
type Ctx = Handle<AisInteractiveContext>;
type View = Handle<dyn V3dViewSource>;
type A = AisViewController;
impl AisViewControllerFrameHooks for FrameSlotRecorder {
    fn update_zoom(&self, c: &mut A, delta: &AspectScrollDelta) -> Result<bool, Failure> {
        let args = || {
            format!(
                "{},{},{},{}",
                delta.point.x,
                delta.point.y,
                bits(delta.delta),
                delta.flags
            )
        };
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_zoom(delta),
            Via::Base => Ok(c.update_zoom_base(delta)),
        };
        self.trace.dispatch(c, "zoom", &args, true, &mut call)
    }
    fn update_z_rotation(&self, c: &mut A, angle: f64) -> Result<bool, Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_z_rotation(angle),
            Via::Base => Ok(c.update_z_rotation_base(angle)),
        };
        self.trace
            .dispatch(c, "zrotation", &|| bits(angle), true, &mut call)
    }
    fn reset_view_input(&self, c: &mut A) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.reset_view_input(),
            Via::Base => {
                c.reset_view_input_base();
                Ok(())
            }
        };
        self.trace.void(c, "reset", &String::new, &mut call)
    }
    fn update_view_orientation(
        &self,
        c: &mut A,
        orientation: V3dTypeOfOrientation,
        fit: bool,
    ) -> Result<(), Failure> {
        let args = || format!("{},{}", orientation as i32, fit as i32);
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_view_orientation(orientation, fit),
            Via::Base => {
                c.update_view_orientation_base(orientation, fit);
                Ok(())
            }
        };
        self.trace.void(c, "orientation", &args, &mut call)
    }
    fn flush_buffers(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.flush_buffers(ctx, view),
            Via::Base => {
                c.flush_buffers_base(ctx, view);
                Ok(())
            }
        };
        self.trace
            .void(c, "buffers", &|| self.same(ctx, view), &mut call)
    }
    fn flush_gestures(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.flush_gestures(ctx, view),
            Via::Base => c.flush_gestures_base(ctx, view),
        };
        self.trace
            .void(c, "gestures", &|| self.same(ctx, view), &mut call)
    }
    fn fit_all_auto(
        &self,
        c: &mut A,
        ctx: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.fit_all_auto(ctx, view),
            Via::Base => c.fit_all_auto_base(ctx, view),
        };
        self.trace
            .void(c, "fitauto", &|| self.same_actual(ctx, view), &mut call)
    }
    fn pick_axis(
        &self,
        c: &mut A,
        point: &mut Point3,
        ctx: &AisInteractiveContext,
        view: &V3dView,
        axis: Axis1,
    ) -> Result<bool, Failure> {
        let args = || {
            let (location, direction) = (axis.location(), axis.direction());
            format!(
                "{},{},{}",
                self.same_actual(ctx, view),
                point_bits(&location),
                dir_bits(&direction)
            )
        };
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.pick_axis(point, ctx, view, axis),
            Via::Base => c.pick_axis_base(point, ctx, view, axis),
        };
        self.trace.dispatch(c, "axis", &args, true, &mut call)
    }
    fn handle_panning(&self, c: &mut A, view: &V3dView) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_panning(view),
            Via::Base => c.handle_panning_base(view),
        };
        self.trace
            .void(c, "panning", &|| self.same_view(view), &mut call)
    }
    fn handle_z_rotate(&self, c: &mut A, view: &V3dView) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_z_rotate(view),
            Via::Base => c.handle_z_rotate_base(view),
        };
        self.trace
            .void(c, "zrotate", &|| self.same_view(view), &mut call)
    }
    fn handle_orbit_rotation(
        &self,
        c: &mut A,
        view: &V3dView,
        pivot: &Point3,
        lock: bool,
    ) -> Result<(), Failure> {
        let args = || {
            format!(
                "{},{},{}",
                self.same_view(view),
                point_bits(pivot),
                lock as i32
            )
        };
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_orbit_rotation(view, pivot, lock),
            Via::Base => c.handle_orbit_rotation_base(view, pivot, lock),
        };
        self.trace.void(c, "orbit", &args, &mut call)
    }
    fn handle_view_rotation(
        &self,
        c: &mut A,
        view: &V3dView,
        yaw: f64,
        pitch: f64,
        roll: f64,
        restart: bool,
    ) -> Result<(), Failure> {
        let args = || {
            let (same, flag) = (self.same_view(view), restart as i32);
            format!("{same},{},{},{},{flag}", bits(yaw), bits(pitch), bits(roll))
        };
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_view_rotation(view, yaw, pitch, roll, restart),
            Via::Base => c.handle_view_rotation_base(view, yaw, pitch, roll, restart),
        };
        self.trace.void(c, "viewrotation", &args, &mut call)
    }
    fn handle_view_orientation_keys(
        &self,
        c: &mut A,
        ctx: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_view_orientation_keys(ctx, view),
            Via::Base => c.handle_view_orientation_keys_base(ctx, view),
        };
        self.trace
            .void(c, "orientkeys", &|| self.same(ctx, view), &mut call)
    }
    fn handle_view_redraw(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_view_redraw(ctx, view),
            Via::Base => c.handle_view_redraw_base(ctx, view),
        };
        self.trace
            .void(c, "redraw", &|| self.same(ctx, view), &mut call)
    }
    fn handle_xr_input(
        &self,
        c: &mut A,
        ctx: &Ctx,
        view: &View,
        walk: &AisWalkDelta,
    ) -> Result<(), Failure> {
        let args = || format!("{},{}", self.same(ctx, view), walk.is_defined() as i32);
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_xr_input(ctx, view, walk),
            Via::Base => c.handle_xr_input_base(ctx, view, walk),
        };
        self.trace.void(c, "xrinput", &args, &mut call)
    }
    fn handle_xr_presentations(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_xr_presentations(ctx, view),
            Via::Base => c.handle_xr_presentations_base(ctx, view),
        };
        self.trace
            .void(c, "xrpresentations", &|| self.same(ctx, view), &mut call)
    }
    fn key_down(
        &self,
        c: &mut A,
        key: AspectVKey,
        time: f64,
        pressure: f64,
    ) -> Result<(), Failure> {
        let args = || format!("{key},{},{}", bits(time), bits(pressure));
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.key_down(key, time, pressure),
            Via::Base => c.key_down_base(key, time, pressure),
        };
        self.trace.void(c, "keydown", &args, &mut call)
    }
    fn key_up(&self, c: &mut A, key: AspectVKey, time: f64) -> Result<(), Failure> {
        let args = || format!("{key},{}", bits(time));
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.key_up(key, time),
            Via::Base => c.key_up_base(key, time),
        };
        self.trace.void(c, "keyup", &args, &mut call)
    }
    fn key_from_axis(
        &self,
        c: &mut A,
        neg: AspectVKey,
        pos: AspectVKey,
        time: f64,
        pressure: f64,
    ) -> Result<(), Failure> {
        let args = || format!("{neg},{pos},{},{}", bits(time), bits(pressure));
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.key_from_axis(neg, pos, time, pressure),
            Via::Base => c.key_from_axis_base(neg, pos, time, pressure),
        };
        self.trace.void(c, "keyaxis", &args, &mut call)
    }
    fn add_touch_point(
        &self,
        c: &mut A,
        id: usize,
        point: Vec2<f64>,
        clear: bool,
    ) -> Result<(), Failure> {
        let args = || format!("{},{}", touch_args(id, point), clear as i32);
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.add_touch_point(id, point, clear),
            Via::Base => c.add_touch_point_base(id, point, clear),
        };
        self.trace.void(c, "addtouch", &args, &mut call)
    }
    fn remove_touch_point(
        &self,
        c: &mut A,
        id: usize,
        clear_points: bool,
    ) -> Result<bool, Failure> {
        let args = || format!("{},{}", touch_id(id), clear_points as i32);
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.remove_touch_point(id, clear_points),
            Via::Base => c.remove_touch_point_base(id, clear_points),
        };
        self.trace
            .dispatch(c, "removetouch", &args, true, &mut call)
    }
    fn update_touch_point(&self, c: &mut A, id: usize, point: Vec2<f64>) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_touch_point(id, point),
            Via::Base => c.update_touch_point_base(id, point),
        };
        self.trace
            .void(c, "updatetouch", &|| touch_args(id, point), &mut call)
    }
    fn update_3d_mouse(&self, c: &mut A, event: &WntHidSpaceMouse<'_>) -> Result<bool, Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_3d_mouse(event),
            Via::Base => c.update_3d_mouse_base(event),
        };
        self.trace
            .dispatch(c, "mouse3d", &|| report_args(event), true, &mut call)
    }
    fn update_3d_mouse_translation(
        &self,
        c: &mut A,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_3d_mouse_translation(event),
            Via::Base => dispatch_space_mouse_translation(c, event),
        };
        self.trace
            .dispatch(c, "translation", &|| report_args(event), true, &mut call)
    }
    fn update_3d_mouse_rotation(
        &self,
        c: &mut A,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_3d_mouse_rotation(event),
            Via::Base => dispatch_space_mouse_rotation(c, event),
        };
        self.trace
            .dispatch(c, "rotation", &|| report_args(event), true, &mut call)
    }
    fn update_3d_mouse_keys(
        &self,
        c: &mut A,
        event: &WntHidSpaceMouse<'_>,
    ) -> Result<bool, Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.update_3d_mouse_keys(event),
            Via::Base => dispatch_space_mouse_keys(c, event),
        };
        self.trace
            .dispatch(c, "keys3d", &|| report_args(event), true, &mut call)
    }
    fn handle_camera_actions(
        &self,
        c: &mut A,
        ctx: &AisInteractiveContext,
        view: &View,
        walk: &AisWalkDelta,
    ) -> Result<(), Failure> {
        let args = || format!("{},{}", self.same(ctx, view), walk_text(walk));
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_camera_actions(ctx, view, walk).map(|_| -1),
            Via::Base => c.handle_camera_actions_base(ctx, view, walk).map(|_| -1),
        };
        let text = |_: &i32| "-1".to_string();
        self.trace
            .dispatch_value(c, "cameraactions", &args, (-1, &text), &mut call)
            .map(|_| ())
    }
    fn handle_xr_turn_pad(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_xr_turn_pad(ctx, view),
            Via::Base => c.handle_xr_turn_pad_base(ctx, view),
        };
        self.trace
            .void(c, "xrturnpad", &|| self.same(ctx, view), &mut call)
    }
    fn handle_xr_teleport(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_xr_teleport(ctx, view),
            Via::Base => c.handle_xr_teleport_base(ctx, view),
        };
        self.trace
            .void(c, "xrteleport", &|| self.same(ctx, view), &mut call)
    }
    fn handle_xr_picking(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_xr_picking(ctx, view),
            Via::Base => c.handle_xr_picking_base(ctx, view),
        };
        self.trace
            .void(c, "xrpicking", &|| self.same(ctx, view), &mut call)
    }
    fn handle_xr_highlight(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_xr_highlight(ctx, view),
            Via::Base => c.handle_xr_highlight_base(ctx, view),
        };
        self.trace
            .void(c, "xrhighlight", &|| self.same(ctx, view), &mut call)
    }
    fn handle_xr_move_to(
        &self,
        c: &mut A,
        ctx: &Ctx,
        view: &View,
        pose: &Transform3,
        highlight: bool,
    ) -> Result<i32, Failure> {
        let args = || {
            let t = pose.translation_part();
            format!(
                "{},{},{},{},{}",
                self.same(ctx, view),
                bits(t.x()),
                bits(t.y()),
                bits(t.z()),
                highlight as i32
            )
        };
        // The witness reports the int result through its bool dispatch: a hit count of 0 or 1.
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c
                .handle_xr_move_to(ctx, view, pose, highlight)
                .map(|hit| hit != 0),
            Via::Base => c
                .handle_xr_move_to_base(ctx, view, pose, highlight)
                .map(|hit| hit != 0),
        };
        self.trace
            .dispatch(c, "xrmoveto", &args, true, &mut call)
            .map(i32::from)
    }
}
use martensite_tkservice::aspect::{
    dispatch_space_mouse_keys, dispatch_space_mouse_rotation, dispatch_space_mouse_translation,
};
