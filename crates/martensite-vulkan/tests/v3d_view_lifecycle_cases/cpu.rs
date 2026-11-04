//! Real retained selection and parameters establish scene setup without GPU pixels.
use super::*;
#[allow(dead_code)] // The shared probe has optional geometry diagnostics unused by this fixed scene.
#[path = "../support/selection_frame_probe.rs"]
mod recording_probe;

#[test]
fn dynamic_highlight_is_removed_after_the_actual_method_switch() {
    let mut probe = recording_probe::SelectionFrameProbe::new();
    let shape = box_sized([1., 2., 3.]).unwrap();
    highlight_scene(&mut probe.session, &shape).unwrap();
    probe.session.move_cursor(250, 250).unwrap();
    assert!(probe.session.context().has_detected());
    switch_to_raytrace(&probe.session).unwrap();
    probe.session.move_cursor(400, 400).unwrap();
    assert!(!probe.session.context().has_detected());
}

#[test]
fn method_switch_preserves_unrelated_rendering_parameters() {
    let probe = recording_probe::SelectionFrameProbe::new();
    let view = probe.session.view().view_base();
    view.change_rendering_params(&mut |params| {
        params.ray_tracing.is_reflection_enabled = true;
        params.ray_tracing.is_antialiasing_enabled = true;
    });
    switch_to_raytrace(&probe.session).unwrap();
    let actual = view.rendering_params();
    assert_eq!(actual.method, Graphic3dRenderingMode::Raytracing);
    assert!(actual.ray_tracing.is_reflection_enabled);
    assert!(actual.ray_tracing.is_antialiasing_enabled);
}

#[test]
fn texture_limit_scene_keeps_the_actual_top_view_and_shaded_box() {
    let mut probe = recording_probe::SelectionFrameProbe::new();
    let shape = box_sized([1.; 3]).unwrap();
    texture_limit_scene(&mut probe.session, &shape).unwrap();
    let object = probe.session.object("b").unwrap();
    assert!(probe.session.context().is_displayed_mode(object, 1));
    let (frame, _) = probe.snapshot();
    assert!(!frame.draws().is_empty());
    assert_eq!(frame.extent(), [409, 409]);
}
