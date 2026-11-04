//! The same recorder as the frame witness, installed as the camera-hook owner too, as in
//! the recorded controller_camera_slots capture: one object overrides both hook traits.
use super::view_controller_frame_slot_fixture::{FrameSlotRecorder, walk_text};
use super::view_controller_slot_trace::{Via, bits, point_bits};
use super::*;
use crate::v3d::{V3dView, V3dViewSource};
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::AspectScrollDelta;

type Ctx = Handle<AisInteractiveContext>;
type View = Handle<dyn V3dViewSource>;
type A = AisViewController;
fn scroll_args(scroll: &AspectScrollDelta) -> String {
    format!(
        "{},{},{},{}",
        scroll.point.x,
        scroll.point.y,
        bits(scroll.delta),
        scroll.flags
    )
}
/// Void virtuals of this witness write their result as the quoted text "-1".
fn void_text(_: &i32) -> String {
    "-1".to_string()
}
impl FrameSlotRecorder {
    fn camera_void(
        &self,
        c: &mut A,
        slot: &str,
        args: &dyn Fn() -> String,
        call: &mut dyn FnMut(&mut A, Via) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| call(c, via).map(|_| -1);
        self.trace
            .dispatch_value(c, slot, args, (-1, &void_text), &mut call)
            .map(|_| ())
    }
}
impl AisViewControllerCameraHooks for FrameSlotRecorder {
    fn flush_view_events(
        &self,
        c: &mut A,
        ctx: &Ctx,
        view: &View,
        to_handle: bool,
    ) -> Result<(), Failure> {
        let args = || format!("{},{}", self.same(ctx, view), to_handle as i32);
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.flush_view_events(ctx, view, to_handle),
            Via::Base => c.flush_view_events_base(ctx, view, to_handle),
        };
        self.camera_void(c, "flushevents", &args, &mut call)
    }
    fn handle_view_events(&self, c: &mut A, ctx: &Ctx, view: &View) -> Result<(), Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_view_events(ctx, view),
            Via::Base => c.handle_view_events_base(ctx, view),
        };
        self.camera_void(c, "handleevents", &|| self.same(ctx, view), &mut call)
    }
    fn handle_navigation_keys(
        &self,
        c: &mut A,
        ctx: &Ctx,
        view: &View,
    ) -> Result<AisWalkDelta, Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_navigation_keys(ctx, view),
            Via::Base => c.handle_navigation_keys_base(ctx, view),
        };
        let direct = (
            AisWalkDelta::default(),
            &walk_text as &dyn Fn(&AisWalkDelta) -> String,
        );
        self.trace
            .dispatch_value(c, "navigation", &|| self.same(ctx, view), direct, &mut call)
    }
    fn handle_zoom(
        &self,
        c: &mut A,
        view: &V3dView,
        scroll: &AspectScrollDelta,
        hit: Option<&Point3>,
    ) -> Result<(), Failure> {
        let args = || {
            let hit = hit.map(point_bits).unwrap_or_else(|| "none".to_string());
            format!("{},{},{hit}", self.same_view(view), scroll_args(scroll))
        };
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.handle_zoom(view, scroll, hit),
            Via::Base => c.handle_zoom_base(view, scroll, hit),
        };
        self.camera_void(c, "handlezoom", &args, &mut call)
    }
    fn handle_z_focus_scroll(
        &self,
        c: &mut A,
        view: &V3dView,
        scroll: &AspectScrollDelta,
    ) -> Result<(), Failure> {
        let args = || format!("{},{}", self.same_view(view), scroll_args(scroll));
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.dispatch_z_focus_scroll(view, scroll),
            Via::Base => c.handle_z_focus_scroll(view, scroll),
        };
        self.camera_void(c, "focus", &args, &mut call)
    }
    fn pick_point(
        &self,
        c: &mut A,
        point: &mut Point3,
        ctx: &AisInteractiveContext,
        view: &V3dView,
        cursor: &Vec2<i32>,
        stick: bool,
    ) -> Result<bool, Failure> {
        let args = || {
            format!(
                "{},{},{},{}",
                self.same_actual(ctx, view),
                cursor.x,
                cursor.y,
                stick as i32
            )
        };
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.pick_point(point, ctx, view, cursor, stick),
            Via::Base => c.pick_point_base(point, ctx, view, cursor, stick),
        };
        let text = |hit: &bool| (*hit as i32).to_string();
        self.trace
            .dispatch_value(c, "pickpoint", &args, (true, &text), &mut call)
    }
    fn gravity_point(
        &self,
        c: &mut A,
        ctx: &AisInteractiveContext,
        view: &V3dView,
    ) -> Result<Point3, Failure> {
        let mut call = |c: &mut A, via| match via {
            Via::Reenter => c.gravity_point(ctx, view),
            Via::Base => c.gravity_point_base(ctx, view),
        };
        let direct = (
            Point3::new(0.5, 0.25, 0.125),
            &point_bits as &dyn Fn(&Point3) -> String,
        );
        self.trace.dispatch_value(
            c,
            "gravity",
            &|| self.same_actual(ctx, view),
            direct,
            &mut call,
        )
    }
}
