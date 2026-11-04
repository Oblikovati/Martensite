//! Valid other-node mutations follow the retained Native controller witness.
use super::view_controller_action_scene_fixture::{ActionScene, action_scene};
use super::*;
use crate::v3d::V3dView;
use martensite_tkernel::{ncollection::Vec2, standard::Failure};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{aspect::*, graphic3d::Graphic3dCameraProjection};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ScrollMutation {
    Observe,
    Append,
    RemoveNext,
    RemoveEarlierEdit,
    CopyCurrent,
    TypedFailure,
    Reenter,
    Panic,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ScrollRole {
    Zoom,
    Focus,
    Picker,
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct ScrollInvocation {
    pub(super) role: ScrollRole,
    pub(super) delta: f64,
    pub(super) point: Vec2<i32>,
    pub(super) hit: Option<Point3>,
}
pub(super) struct LiveScrollHooks {
    pub(super) calls: Mutex<Vec<ScrollInvocation>>,
    mutation: ScrollMutation,
    role: ScrollRole,
    armed: AtomicBool,
}
impl LiveScrollHooks {
    /// Retain a shared recorder across reentry; `LiveScrollHooks::new(ScrollMutation::Observe, ScrollRole::Zoom)`.
    pub(super) fn new(mutation: ScrollMutation, role: ScrollRole) -> Arc<Self> {
        Arc::new(Self {
            calls: Mutex::new(Vec::new()),
            mutation,
            role,
            armed: AtomicBool::new(true),
        })
    }
    fn record(&self, role: ScrollRole, scroll: &AspectScrollDelta, hit: Option<&Point3>) {
        self.calls.lock().unwrap().push(ScrollInvocation {
            role,
            delta: scroll.delta,
            point: scroll.point,
            hit: hit.copied(),
        });
    }
    fn mutate(&self, c: &mut AisViewController, view: &V3dView, tag: f64) -> Result<(), Failure> {
        let trigger = mutation_trigger(self.mutation);
        if tag != trigger || !self.armed.swap(false, Ordering::SeqCst) {
            return Ok(());
        }
        self.apply_mutation(c, view)
    }
    fn apply_mutation(&self, c: &mut AisViewController, view: &V3dView) -> Result<(), Failure> {
        match self.mutation {
            ScrollMutation::Observe => Ok(()),
            ScrollMutation::Reenter => {
                c.handle_zoom(view, &AspectScrollDelta::with_delta(77.0, 0), None)
            }
            ScrollMutation::TypedFailure => Err(scroll_callback_failure()),
            ScrollMutation::Panic => panic!("zoom queue callback tag=22; unrelated failure"),
            mutation => mutate_live_scroll_queue(c, mutation, self.role),
        }
    }
}
impl AisViewControllerCameraHooks for LiveScrollHooks {
    fn handle_zoom(
        &self,
        c: &mut AisViewController,
        view: &V3dView,
        scroll: &AspectScrollDelta,
        hit: Option<&Point3>,
    ) -> Result<(), Failure> {
        self.record(ScrollRole::Zoom, scroll, hit);
        if self.role != ScrollRole::Zoom {
            return Ok(());
        }
        self.mutate(c, view, scroll.delta)
    }
    fn handle_z_focus_scroll(
        &self,
        c: &mut AisViewController,
        view: &V3dView,
        scroll: &AspectScrollDelta,
    ) -> Result<(), Failure> {
        self.record(ScrollRole::Focus, scroll, None);
        self.mutate(c, view, scroll.delta)
    }
    fn pick_point(
        &self,
        c: &mut AisViewController,
        point: &mut Point3,
        _context: &AisInteractiveContext,
        view: &V3dView,
        cursor: &Vec2<i32>,
        _stick: bool,
    ) -> Result<bool, Failure> {
        self.record(
            ScrollRole::Picker,
            &AspectScrollDelta::new(*cursor, f64::from(cursor.x), 0),
            None,
        );
        self.mutate(c, view, f64::from(cursor.x))?;
        *point = Point3::new(7.0, 8.0, 9.0);
        Ok(true)
    }
}
fn mutate_live_scroll_queue(
    c: &mut AisViewController,
    mutation: ScrollMutation,
    role: ScrollRole,
) -> Result<(), Failure> {
    let queue = &mut c.render_input.zoom_actions;
    match mutation {
        ScrollMutation::Append => {
            queue.append(scroll_for_role(role, 44.0));
        }
        ScrollMutation::RemoveNext => queue.remove(2)?,
        ScrollMutation::RemoveEarlierEdit => {
            queue.remove(1)?;
            queue.last_mut().unwrap().delta = 99.0;
        }
        ScrollMutation::CopyCurrent => queue.change_value(1)?.delta = 99.0,
        _ => unreachable!("queue mutation={mutation:?}; expected a node edit"),
    }
    Ok(())
}
/// Reuse a real CPU scene with literal scroll inputs; `live_scroll_scene(ScrollRole::Zoom, ScrollMutation::Append)`.
pub(super) fn live_scroll_scene(
    role: ScrollRole,
    mutation: ScrollMutation,
) -> (ActionScene, Arc<LiveScrollHooks>) {
    let mut scene = action_scene(0);
    let hooks = LiveScrollHooks::new(mutation, role);
    scene.controller.set_camera_hooks(hooks.clone());
    configure_live_scroll_scene(&mut scene, role);
    (scene, hooks)
}
fn configure_live_scroll_scene(scene: &mut ActionScene, role: ScrollRole) {
    let projection = match role {
        ScrollRole::Zoom => Graphic3dCameraProjection::Orthographic,
        ScrollRole::Focus => Graphic3dCameraProjection::Stereo,
        ScrollRole::Picker => Graphic3dCameraProjection::Perspective,
    };
    scene.view.camera().set_projection_type(projection);
    scene
        .controller
        .set_allow_zooming(role != ScrollRole::Focus);
    for delta in [11.0, 22.0, 33.0] {
        scene
            .controller
            .render_input
            .zoom_actions
            .append(scroll_for_role(role, delta));
    }
}
fn mutation_trigger(mutation: ScrollMutation) -> f64 {
    match mutation {
        ScrollMutation::RemoveEarlierEdit
        | ScrollMutation::TypedFailure
        | ScrollMutation::Panic => 22.0,
        _ => 11.0,
    }
}
fn scroll_callback_failure() -> Failure {
    Failure::with_type(
        "Standard_ProgramError",
        "zoom queue callback tag=22; expected success",
        "",
    )
}
fn scroll_for_role(role: ScrollRole, delta: f64) -> AspectScrollDelta {
    match role {
        ScrollRole::Zoom => AspectScrollDelta::with_delta(delta, 0),
        ScrollRole::Focus => AspectScrollDelta::with_delta(delta, ASPECT_VKEY_FLAGS_CTRL),
        ScrollRole::Picker => {
            AspectScrollDelta::new(Vec2::new(delta as i32, delta as i32 + 100), delta, 0)
        }
    }
}
/// Enter the actual controller pipeline; `dispatch_live_scroll(&mut scene)?`.
pub(super) fn dispatch_live_scroll(scene: &mut ActionScene) -> Result<(), Failure> {
    scene
        .controller
        .handle_camera_actions(&scene.context, &scene.view, &AisWalkDelta::default())
}
/// Inspect callback order without altering it; `recorded_tags(&hooks, ScrollRole::Zoom)`.
pub(super) fn recorded_tags(hooks: &LiveScrollHooks, role: ScrollRole) -> Vec<f64> {
    hooks
        .calls
        .lock()
        .unwrap()
        .iter()
        .filter(|call| call.role == role)
        .map(|call| call.delta)
        .collect()
}
/// Inspect failure retention or successful clearing; `retained_tags(&scene)`.
pub(super) fn retained_tags(scene: &ActionScene) -> Vec<f64> {
    scene
        .controller
        .render_input
        .zoom_actions
        .iter()
        .map(|scroll| scroll.delta)
        .collect()
}
