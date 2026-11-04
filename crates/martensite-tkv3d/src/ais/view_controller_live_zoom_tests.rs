//! Independent expectations retain the observed 13-scenario live-queue contract.
use super::view_controller_live_zoom_fixture::*;
use super::*;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::AspectScrollDelta;
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::Arc,
};

fn assert_live_zoom(mutation: ScrollMutation, expected: &[f64]) {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Zoom, mutation);
    dispatch_live_scroll(&mut scene).unwrap();
    assert_eq!(recorded_tags(&hooks, ScrollRole::Zoom), expected);
    assert!(retained_tags(&scene).is_empty());
}
#[test]
fn live_zoom_observes_each_original_node() {
    let expected = [11.0, 22.0, 33.0];
    assert_live_zoom(ScrollMutation::Observe, &expected);
}
#[test]
fn live_zoom_visits_callback_appended_future_node() {
    let expected = [11.0, 22.0, 33.0, 44.0];
    assert_live_zoom(ScrollMutation::Append, &expected);
}
#[test]
fn live_zoom_skips_callback_removed_next_node() {
    let expected = [11.0, 33.0];
    assert_live_zoom(ScrollMutation::RemoveNext, &expected);
}
#[test]
fn live_zoom_retains_current_cursor_when_earlier_node_is_removed() {
    let expected = [11.0, 22.0, 99.0];
    assert_live_zoom(ScrollMutation::RemoveEarlierEdit, &expected);
}
#[test]
fn live_zoom_copies_current_argument_before_callback_edit() {
    let expected = [11.0, 22.0, 33.0];
    assert_live_zoom(ScrollMutation::CopyCurrent, &expected);
}
#[test]
fn camera_hook_owner_survives_nested_virtual_zoom() {
    let expected = [11.0, 77.0, 22.0, 33.0];
    assert_live_zoom(ScrollMutation::Reenter, &expected);
}
#[test]
fn camera_hook_owner_and_queue_survive_typed_failure_for_retry() {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Zoom, ScrollMutation::TypedFailure);
    let error = dispatch_live_scroll(&mut scene).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(
        error.message(),
        "zoom queue callback tag=22; expected success"
    );
    assert_failure_retry(&mut scene, &hooks);
}
fn assert_failure_retry(
    scene: &mut super::view_controller_action_scene_fixture::ActionScene,
    hooks: &LiveScrollHooks,
) {
    assert_eq!(retained_tags(scene), [11.0, 22.0, 33.0]);
    assert_eq!(recorded_tags(hooks, ScrollRole::Zoom), [11.0, 22.0]);
    hooks.calls.lock().unwrap().clear();
    dispatch_live_scroll(scene).unwrap();
    assert_eq!(recorded_tags(hooks, ScrollRole::Zoom), [11.0, 22.0, 33.0]);
    assert!(retained_tags(scene).is_empty());
}
#[test]
fn camera_hook_owner_survives_unrelated_unwind_for_retry() {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Zoom, ScrollMutation::Panic);
    let error = catch_unwind(AssertUnwindSafe(|| dispatch_live_scroll(&mut scene))).unwrap_err();
    assert_eq!(
        error.downcast_ref::<&str>(),
        Some(&"zoom queue callback tag=22; unrelated failure")
    );
    assert_failure_retry(&mut scene, &hooks);
}
#[test]
fn camera_loop_zoom_gate_clears_queue_without_calling_override() {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Zoom, ScrollMutation::Observe);
    scene.controller.set_allow_zooming(false);
    dispatch_live_scroll(&mut scene).unwrap();
    assert!(recorded_tags(&hooks, ScrollRole::Zoom).is_empty());
    assert!(retained_tags(&scene).is_empty());
}
#[test]
fn direct_virtual_zoom_reaches_override_with_base_zoom_disabled() {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Zoom, ScrollMutation::Observe);
    scene.controller.set_allow_zooming(false);
    scene
        .controller
        .handle_zoom(&scene.view, &AspectScrollDelta::with_delta(11.0, 0), None)
        .unwrap();
    assert_eq!(recorded_tags(&hooks, ScrollRole::Zoom), [11.0]);
}
#[test]
fn focus_dispatch_precedes_zoom_gate_and_visits_appended_node() {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Focus, ScrollMutation::Append);
    dispatch_live_scroll(&mut scene).unwrap();
    assert_eq!(
        recorded_tags(&hooks, ScrollRole::Focus),
        [11.0, 22.0, 33.0, 44.0]
    );
    assert!(recorded_tags(&hooks, ScrollRole::Zoom).is_empty());
}
#[test]
fn picker_removed_next_node_is_not_picked_or_zoomed() {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Picker, ScrollMutation::RemoveNext);
    dispatch_live_scroll(&mut scene).unwrap();
    assert_eq!(recorded_tags(&hooks, ScrollRole::Picker), [11.0, 33.0]);
    assert_eq!(recorded_tags(&hooks, ScrollRole::Zoom), [11.0, 33.0]);
    assert_picker_arguments(&hooks, &[11, 33]);
}
#[test]
fn picker_observes_future_point_but_zoom_reads_edited_future_delta() {
    let (mut scene, hooks) =
        live_scroll_scene(ScrollRole::Picker, ScrollMutation::RemoveEarlierEdit);
    dispatch_live_scroll(&mut scene).unwrap();
    assert_eq!(
        recorded_tags(&hooks, ScrollRole::Picker),
        [11.0, 22.0, 33.0]
    );
    assert_eq!(recorded_tags(&hooks, ScrollRole::Zoom), [11.0, 22.0, 99.0]);
    assert_picker_arguments(&hooks, &[11, 22, 33]);
}
fn assert_picker_arguments(hooks: &LiveScrollHooks, expected_x: &[i32]) {
    let calls = hooks.calls.lock().unwrap();
    let picks: Vec<Vec2<i32>> = calls
        .iter()
        .filter(|v| v.role == ScrollRole::Picker)
        .map(|v| v.point)
        .collect();
    let expected: Vec<Vec2<i32>> = expected_x.iter().map(|x| Vec2::new(*x, x + 100)).collect();
    assert_eq!(picks, expected);
    assert!(
        calls
            .iter()
            .filter(|v| v.role == ScrollRole::Zoom)
            .all(|v| v.hit == Some(Point3::new(7.0, 8.0, 9.0)))
    );
}
#[test]
fn taking_camera_hooks_returns_same_installed_owner() {
    let (mut scene, hooks) = live_scroll_scene(ScrollRole::Zoom, ScrollMutation::Observe);
    let erased: Arc<dyn AisViewControllerCameraHooks> = hooks;
    assert!(Arc::ptr_eq(
        &scene.controller.take_camera_hooks().unwrap(),
        &erased
    ));
    assert!(scene.controller.take_camera_hooks().is_none());
}

struct DefaultCameraHooks;
impl AisViewControllerCameraHooks for DefaultCameraHooks {}

#[test]
fn installed_default_zoom_and_focus_delegate_to_same_base_camera() {
    let (mut scene, _) = live_scroll_scene(ScrollRole::Focus, ScrollMutation::Observe);
    let (mut baseline, _) = live_scroll_scene(ScrollRole::Focus, ScrollMutation::Observe);
    scene
        .controller
        .set_camera_hooks(Arc::new(DefaultCameraHooks));
    baseline.controller.take_camera_hooks();
    dispatch_live_scroll(&mut scene).unwrap();
    dispatch_live_scroll(&mut baseline).unwrap();
    assert_eq!(
        scene.view.camera().z_focus(),
        baseline.view.camera().z_focus()
    );
    compare_default_zoom(&mut scene, &mut baseline);
}
fn compare_default_zoom(
    scene: &mut super::view_controller_action_scene_fixture::ActionScene,
    baseline: &mut super::view_controller_action_scene_fixture::ActionScene,
) {
    scene.controller.set_allow_zooming(true);
    baseline.controller.set_allow_zooming(true);
    let scroll = AspectScrollDelta::with_delta(11.0, 0);
    scene
        .controller
        .handle_zoom(&scene.view, &scroll, None)
        .unwrap();
    baseline
        .controller
        .handle_zoom(&baseline.view, &scroll, None)
        .unwrap();
    assert_eq!(scene.view.camera().scale(), baseline.view.camera().scale());
}
#[test]
fn installed_default_picker_retains_selector_output_and_status() {
    let (mut scene, _) = live_scroll_scene(ScrollRole::Picker, ScrollMutation::Observe);
    scene
        .controller
        .set_camera_hooks(Arc::new(DefaultCameraHooks));
    let pixel = Vec2::new(320, 240);
    let mut via_hook = Point3::new(71.0, 82.0, 93.0);
    let mut via_base = via_hook;
    let hooked = scene
        .controller
        .pick_point(&mut via_hook, &scene.context, &scene.view, &pixel, true)
        .unwrap();
    scene.controller.take_camera_hooks();
    let unhooked = scene
        .controller
        .pick_point(&mut via_base, &scene.context, &scene.view, &pixel, true)
        .unwrap();
    assert_eq!((hooked, via_hook), (unhooked, via_base));
}
