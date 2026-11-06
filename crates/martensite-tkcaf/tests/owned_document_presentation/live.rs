//! Actual owned windows expose repaint, title and capture order without an implicit repair.
use super::no_window::QuietReports;
use super::{
    control_shape,
    observed_windows::{ObservedWindows, WindowAction},
};
use crate::document_presentation::DocumentScene;
use std::{path::PathBuf, rc::Rc};

fn native_scene(destination: Option<PathBuf>) -> (DocumentScene, Rc<ObservedWindows>) {
    let system = ObservedWindows::connected();
    let scene = DocumentScene::new(system.clone(), Rc::new(QuietReports), destination).unwrap();
    (scene, system)
}
fn direct_single_view() -> (DocumentScene, Rc<ObservedWindows>) {
    let (mut scene, system) = native_scene(None);
    scene.publish("a", &control_shape()).unwrap();
    scene
        .control_viewer()
        .make_view(1, "AXON", 0, 0, 400, 400)
        .unwrap();
    scene.only("a").unwrap();
    (scene, system)
}

#[test]
fn single_fit_updates_live_pixels_then_the_native_title() {
    let (mut scene, system) = direct_single_view();
    let before = system.pixels(0);
    system.actions.borrow_mut().clear();
    scene.fit_existing().unwrap();
    let after = system.pixels(0);
    assert!(
        before != after,
        "fit must update actual pixels before a capture helper"
    );
    let expected = format!("1 : AXON - Zoom {:.6}", scene.control_viewer().zoom(1));
    assert_single_fit_actions(&system.actions.borrow(), expected);
}

fn assert_single_fit_actions(actions: &[WindowAction], expected: String) {
    let clears = actions
        .iter()
        .filter(|a| **a == WindowAction::Clear(0))
        .count();
    assert_eq!(clears, 1);
    assert_eq!(actions.last(), Some(&WindowAction::Title(0, expected)));
}

#[test]
fn erase_flushes_after_actual_spatial_repaint() {
    let (mut scene, system) = direct_single_view();
    let before = system.pixels(0);
    system.actions.borrow_mut().clear();
    scene.erase(&["a"]).unwrap();
    assert!(
        before != system.pixels(0),
        "erase must update the live owned window"
    );
    assert_eq!(
        system.actions.borrow().as_slice(),
        &[WindowAction::Clear(0), WindowAction::Flush]
    );
}

#[test]
fn unsupported_extra_or_planar_views_return_typed_refusals() {
    let (mut scene, _) = native_scene(None);
    assert_extra_view_refusal(&mut scene);
    assert_planar_view_refusal(&mut scene);
    scene.close().unwrap();
}

fn assert_extra_view_refusal(scene: &mut DocumentScene) {
    scene
        .control_viewer()
        .make_view(2, "AXON", 0, 0, 60, 60)
        .unwrap();
    let failure = scene.fit_existing().unwrap_err();
    assert_eq!(failure.exception_type(), "DocumentPresentationUnsupported");
    assert!(failure.message().contains("views=[2]"));
    scene.control_viewer().delete_view(2);
}
fn assert_planar_view_refusal(scene: &mut DocumentScene) {
    scene
        .control_viewer()
        .make_view(1, "-2D-", 0, 0, 60, 60)
        .unwrap();
    let failure = scene.result_view("unsupported").unwrap_err();
    assert!(
        failure
            .message()
            .contains("expected zero views or sole spatial AXON view1")
    );
    scene.control_viewer().delete_view(1);
}

#[test]
fn result_capture_observes_fit_repaint_title_then_explicit_capture_repaint() {
    let (mut scene, system) = native_scene(None);
    scene.publish("result", &control_shape()).unwrap();
    system.observe_saves();
    system.actions.borrow_mut().clear();
    scene.result_view("owned-window-order").unwrap();
    assert_capture_order(&system.actions.borrow());
    scene.close().unwrap();
}

fn assert_capture_order(actions: &[WindowAction]) {
    let title = actions
        .iter()
        .position(|a| matches!(a, WindowAction::Title(0, _)))
        .unwrap();
    let save = actions
        .iter()
        .position(|a| *a == WindowAction::Save(0))
        .unwrap();
    assert!(actions[..title].contains(&WindowAction::Clear(0)));
    assert!(title < save);
    // RasterViewer flushes SaveView before saving, after xwd repaint.
    assert_eq!(
        &actions[title + 1..save],
        &[WindowAction::Clear(0), WindowAction::Flush]
    );
}

#[test]
fn native_constructor_keeps_the_real_backend_unviewed_until_requested() {
    let mut scene = DocumentScene::connect(None).unwrap();
    scene.publish("a", &control_shape()).unwrap();
    scene.fit_existing().unwrap();
    assert!(!scene.control_viewer().has_view(1));
    assert!(scene.control_binding("a").unwrap().visible());
    scene.close().unwrap();
}
