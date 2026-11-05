//! Named window/report fakes and actual-window observations keep the fixture boundary testable.
mod live;
mod no_window;
mod observed_windows;
use crate::document_presentation::DocumentScene;
use martensite_tkbrep::topods::Shape;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use no_window::{NoWindowSystem, QuietReports};
use std::rc::Rc;

fn control_shape() -> Shape {
    let mut builder = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    builder.shape().unwrap().clone()
}
fn unviewed_scene() -> (DocumentScene, Rc<NoWindowSystem>) {
    let system = Rc::new(NoWindowSystem::default());
    let scene = DocumentScene::new(system.clone(), Rc::new(QuietReports), None).unwrap();
    (scene, system)
}

#[test]
fn named_replacement_erase_and_redisplay_preserve_literal_identity() {
    let (mut scene, _) = unviewed_scene();
    scene.publish("a", &control_shape()).unwrap();
    let first = scene.control_binding("a").unwrap();
    scene.erase(&["a", "missing"]).unwrap();
    assert!(!first.visible());
    scene.display(&["a"]).unwrap();
    assert!(first.visible());
    assert!(Rc::ptr_eq(&first, &scene.control_binding("a").unwrap()));
    scene.publish("a", &control_shape()).unwrap();
    let replacement = scene.control_binding("a").unwrap();
    assert!(!first.visible() && replacement.visible());
    assert!(!Rc::ptr_eq(&first, &replacement));
}

#[test]
fn defaults_change_only_later_real_shape_drawables() {
    let (mut scene, _) = unviewed_scene();
    let first = scene.control_shape(&control_shape());
    assert_eq!(first.rendering().nb_isos(), 2);
    assert_eq!(first.rendering().discret(), 30);
    assert_eq!(first.rendering().infinity_size(), 100.);
    scene.set_isos(12);
    let next = scene.control_shape(&control_shape());
    assert_eq!(next.rendering().nb_isos(), 12);
    assert_eq!(first.rendering().nb_isos(), 2);
}

#[test]
fn zero_window_fit_and_clear_do_not_allocate_or_lose_named_shapes() {
    let (mut scene, system) = unviewed_scene();
    scene.publish("a", &control_shape()).unwrap();
    let original = scene.control_binding("a").unwrap();
    scene.fit_existing().unwrap();
    assert!(!scene.control_viewer().has_view(1));
    scene.erase_all().unwrap();
    assert_eq!(system.flushes.get(), 1);
    assert_eq!(scene.control_viewer().drawables().len(), 2);
    assert!(!original.visible());
    scene.display(&["a"]).unwrap();
    assert!(Rc::ptr_eq(&original, &scene.control_binding("a").unwrap()));
}

#[test]
fn unknown_name_refusal_retains_the_registry_after_clear() {
    let (mut scene, _) = unviewed_scene();
    scene.publish("a", &control_shape()).unwrap();
    let failure = scene.only("missing").unwrap_err();
    assert!(failure.message().contains("name=\"missing\""));
    assert!(failure.message().contains("expected existing named shape"));
    assert!(!scene.control_binding("a").unwrap().visible());
    assert!(scene.control_viewer().drawables().is_empty());
}

#[test]
fn malformed_frames_and_io_causes_keep_value_and_expected_contract() {
    // No filesystem side effect is needed to exercise these explicit failure inputs.
    DocumentScene::control_capture_refusals();
}
