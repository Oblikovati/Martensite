use super::culling_cpu_baseline::recorded_scene;
use martensite_vulkan::{RenderRedrawCursor, RenderRedrawFailure};
use std::sync::atomic::Ordering;

#[test]
fn actual_v3d_redraw_publishes_one_fresh_status_without_changing_pixels() {
    let (scene, _, _driver) = recorded_scene();
    let render = scene.render_view();
    let before = render.redraw_cursor().unwrap();
    scene.view.redraw();
    let fresh = render.check_redraw_since(&before).unwrap();
    assert_eq!(fresh.publication(), before.publication() + 1);
    let pixels = render.capture().unwrap().rgba;
    assert!(render.check_redraw_since(&fresh).is_err());
    assert_eq!(pixels, render.capture().unwrap().rgba);
}

#[test]
fn actual_failed_middle_publication_cannot_be_hidden_by_later_success() {
    let (scene, device, _driver) = recorded_scene();
    let render = scene.render_view();
    let before = render.redraw_cursor().unwrap();
    device.fail.store(true, Ordering::Relaxed);
    scene.view.redraw();
    let failure = render.check_redraw_since(&before).unwrap_err();
    assert!(matches!(
        failure,
        RenderRedrawFailure::FailedPublication { .. }
    ));
    assert!(failure.to_string().contains("injected failure"));
    device.fail.store(false, Ordering::Relaxed);
    scene.view.redraw();
    assert!(matches!(
        render.check_redraw_since(&before),
        Err(RenderRedrawFailure::UnexpectedPublications { .. })
    ));
}

#[test]
fn actual_target_resize_is_refused_without_rewriting_the_capture() {
    let (scene, _, _driver) = recorded_scene();
    let render = scene.render_view();
    let before = render.redraw_cursor().unwrap();
    scene.view.redraw();
    let pixels = render.capture().unwrap().rgba;
    scene.window.set_size(32, 48);
    assert!(matches!(
        render.check_redraw_since(&before),
        Err(RenderRedrawFailure::TargetChanged { .. })
    ));
    assert_eq!(pixels, render.capture().unwrap().rgba);
}

#[test]
fn a_cursor_keeps_its_owner_and_cannot_certify_a_different_view() {
    let (first, _, _first_driver) = recorded_scene();
    let (second, _, _second_driver) = recorded_scene();
    let cursor = first.render_view().redraw_cursor().unwrap();
    second.view.redraw();
    assert!(matches!(
        second.render_view().check_redraw_since(&cursor),
        Err(RenderRedrawFailure::WrongView { .. })
    ));
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<RenderRedrawCursor>();
    require_send_sync::<RenderRedrawFailure>();
}

#[test]
fn a_noop_callback_cannot_reuse_an_earlier_success() {
    let (scene, _, _driver) = recorded_scene();
    let render = scene.render_view();
    scene.view.redraw();
    let before = render.redraw_cursor().unwrap();
    assert!(matches!(
        render.check_redraw_since(&before),
        Err(RenderRedrawFailure::NoFreshPublication { .. })
    ));
}
