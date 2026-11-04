//! The queueing, transfer and per-frame camera virtuals of `AisViewController` dispatch through
//! the installed `AisViewControllerFrameHooks`, so an override intercepts the controller's own
//! calls: zoom from scrolling, reset from focus loss, transfer from `flush_view_events`, the
//! orientation keys, XR, redraw and fit-all from the frame, pan/rotation from camera actions, and
//! the XR turn, teleport, picking, highlight and move-to handlers from XR input.
//! Without an owner each entrypoint runs its `*_base` body. Native witness:
//! the recorded controller_frame_slots capture.
use super::{AisInteractiveContext, AisViewController, AisWalkDelta};
use crate::v3d::{V3dTypeOfOrientation, V3dView, V3dViewSource};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Axis1, Point3, Transform3};
use martensite_tkservice::aspect::AspectScrollDelta;

type Ctx = Handle<AisInteractiveContext>;
type View = Handle<dyn V3dViewSource>;

impl AisViewController {
    /// Virtual `UpdateZoom`; `let queued = controller.update_zoom(&delta)?;`.
    pub fn update_zoom(&mut self, delta: &AspectScrollDelta) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_zoom(self, delta),
            None => Ok(self.update_zoom_base(delta)),
        }
    }
    /// Virtual `UpdateZRotation`; `let queued = controller.update_z_rotation(0.25)?;`.
    pub fn update_z_rotation(&mut self, angle: f64) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_z_rotation(self, angle),
            None => Ok(self.update_z_rotation_base(angle)),
        }
    }
    /// Virtual `ResetViewInput`; `controller.reset_view_input()?`.
    pub fn reset_view_input(&mut self) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.reset_view_input(self),
            None => {
                self.reset_view_input_base();
                Ok(())
            }
        }
    }
    /// Virtual `UpdateViewOrientation`; `controller.update_view_orientation(orientation, true)?`.
    pub fn update_view_orientation(
        &mut self,
        orientation: V3dTypeOfOrientation,
        to_fit_all: bool,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.update_view_orientation(self, orientation, to_fit_all),
            None => {
                self.update_view_orientation_base(orientation, to_fit_all);
                Ok(())
            }
        }
    }
    /// Virtual `flushBuffers`; `controller.flush_buffers(&context, &view)?`.
    pub fn flush_buffers(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.flush_buffers(self, context, view),
            None => {
                self.flush_buffers_base(context, view);
                Ok(())
            }
        }
    }
    /// Virtual `flushGestures`; `controller.flush_gestures(&context, &view)?`.
    pub fn flush_gestures(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.flush_gestures(self, context, view),
            None => self.flush_gestures_base(context, view),
        }
    }
    /// Virtual `FitAllAuto`; `controller.fit_all_auto(&context, &view)?`.
    pub fn fit_all_auto(
        &mut self,
        context: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.fit_all_auto(self, context, view),
            None => self.fit_all_auto_base(context, view),
        }
    }
    /// Virtual `PickAxis`; `let hit = controller.pick_axis(&mut point, &context, &view, axis)?;`.
    pub fn pick_axis(
        &mut self,
        point: &mut Point3,
        context: &AisInteractiveContext,
        view: &V3dView,
        axis: Axis1,
    ) -> Result<bool, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.pick_axis(self, point, context, view, axis),
            None => self.pick_axis_base(point, context, view, axis),
        }
    }
}

impl AisViewController {
    /// Virtual `handlePanning`; `controller.handle_panning(&view)?`.
    pub fn handle_panning(&mut self, view: &V3dView) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_panning(self, view),
            None => self.handle_panning_base(view),
        }
    }
    /// Virtual `handleZRotate`; `controller.handle_z_rotate(&view)?`.
    pub fn handle_z_rotate(&mut self, view: &V3dView) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_z_rotate(self, view),
            None => self.handle_z_rotate_base(view),
        }
    }
    /// Virtual `handleOrbitRotation`; `controller.handle_orbit_rotation(&view, &pivot, false)?`.
    pub fn handle_orbit_rotation(
        &mut self,
        view: &V3dView,
        pivot: &Point3,
        lock_z_up: bool,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_orbit_rotation(self, view, pivot, lock_z_up),
            None => self.handle_orbit_rotation_base(view, pivot, lock_z_up),
        }
    }
    /// Virtual `handleViewRotation`; `controller.handle_view_rotation(&view, yaw, pitch, roll, false)?`.
    pub fn handle_view_rotation(
        &mut self,
        view: &V3dView,
        yaw: f64,
        pitch: f64,
        roll: f64,
        restart: bool,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_view_rotation(self, view, yaw, pitch, roll, restart),
            None => self.handle_view_rotation_base(view, yaw, pitch, roll, restart),
        }
    }
    /// Virtual `handleViewOrientationKeys`; `controller.handle_view_orientation_keys(&context, &view)?`.
    pub fn handle_view_orientation_keys(
        &mut self,
        context: &Ctx,
        view: &View,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_view_orientation_keys(self, context, view),
            None => self.handle_view_orientation_keys_base(context, view),
        }
    }
    /// Virtual `handleViewRedraw`; `controller.handle_view_redraw(&context, &view)?`.
    pub fn handle_view_redraw(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_view_redraw(self, context, view),
            None => self.handle_view_redraw_base(context, view),
        }
    }
    /// Virtual `handleXRInput`; `controller.handle_xr_input(&context, &view, &walk)?`.
    pub fn handle_xr_input(
        &mut self,
        context: &Ctx,
        view: &View,
        walk: &AisWalkDelta,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_xr_input(self, context, view, walk),
            None => self.handle_xr_input_base(context, view, walk),
        }
    }
    /// Virtual `handleXRPresentations`; `controller.handle_xr_presentations(&context, &view)?`.
    pub fn handle_xr_presentations(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_xr_presentations(self, context, view),
            None => self.handle_xr_presentations_base(context, view),
        }
    }
}

impl AisViewController {
    /// Virtual `handleCameraActions`; `controller.handle_camera_actions(&context, &view, &walk)?`.
    pub fn handle_camera_actions(
        &mut self,
        context: &AisInteractiveContext,
        view: &View,
        walk: &AisWalkDelta,
    ) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_camera_actions(self, context, view, walk),
            None => self.handle_camera_actions_base(context, view, walk),
        }
    }
    /// Virtual `handleXRTurnPad`; `controller.handle_xr_turn_pad(&context, &view)?`.
    pub fn handle_xr_turn_pad(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_xr_turn_pad(self, context, view),
            None => self.handle_xr_turn_pad_base(context, view),
        }
    }
    /// Virtual `handleXRTeleport`; `controller.handle_xr_teleport(&context, &view)?`.
    pub fn handle_xr_teleport(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_xr_teleport(self, context, view),
            None => self.handle_xr_teleport_base(context, view),
        }
    }
    /// Virtual `handleXRPicking`; `controller.handle_xr_picking(&context, &view)?`.
    pub fn handle_xr_picking(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_xr_picking(self, context, view),
            None => self.handle_xr_picking_base(context, view),
        }
    }
    /// Virtual `handleXRHighlight`; `controller.handle_xr_highlight(&context, &view)?`.
    pub fn handle_xr_highlight(&mut self, context: &Ctx, view: &View) -> Result<(), Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_xr_highlight(self, context, view),
            None => self.handle_xr_highlight_base(context, view),
        }
    }
    /// Virtual `handleXRMoveTo`; `let hit = controller.handle_xr_move_to(&context, &view, &pose, true)?;`.
    pub fn handle_xr_move_to(
        &mut self,
        context: &Ctx,
        view: &View,
        pose: &Transform3,
        to_highlight: bool,
    ) -> Result<i32, Failure> {
        match self.frame_hooks() {
            Some(hooks) => hooks.handle_xr_move_to(self, context, view, pose, to_highlight),
            None => self.handle_xr_move_to_base(context, view, pose, to_highlight),
        }
    }
}
