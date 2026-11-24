use super::*;
use crate::immediate_redraw_fixture::{SceneExtractionOnly, lit_sphere_view};
use crate::{Graphic3dRenderView, RenderLayer};
use martensite_tkv3d::v3d::V3dViewer;

fn render_view() -> (Handle<V3dViewer>, Handle<Graphic3dRenderView>) {
    let driver = crate::Graphic3dRenderDriver::with_backend(SceneExtractionOnly, 4096).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = lit_sphere_view(&viewer).get().unwrap().view().cast();
    (viewer, view)
}

#[test]
fn inputs_follow_the_in_place_view_state() {
    let (_viewer, view) = render_view();
    let capture = || ImmediateViewInputs::capture(view.get().unwrap(), [128, 128]);
    let base = capture();
    assert!(base == capture(), "an unchanged view gives equal inputs");
    assert!(base != ImmediateViewInputs::capture(view.get().unwrap(), [64, 128]));
    let camera = crate::view_render_camera::owned_render_camera(view.get().unwrap());
    let scale = camera.scale();
    camera.set_scale(scale * 1.5);
    assert!(base != capture(), "the camera changed in place");
    camera.set_scale(scale);
    assert!(base == capture());
    view.change_rendering_params(&mut |params| params.line_feather += 1.);
    assert!(base != capture(), "the rendering parameters changed");
}

#[test]
fn statistics_parameters_stay_out_of_the_inputs() {
    let (_viewer, view) = render_view();
    let base = ImmediateViewInputs::capture(view.get().unwrap(), [128, 128]);
    view.change_rendering_params(&mut |params| params.stats.stats_text_height += 4);
    assert!(base == ImmediateViewInputs::capture(view.get().unwrap(), [128, 128]));
}

#[test]
fn shells_keep_the_main_spans_without_their_draws() {
    let (_viewer, view) = render_view();
    let layers = vec![
        RenderLayer::new(Vec::new(), true, false, true),
        RenderLayer::new(Vec::new(), false, true, false),
        RenderLayer::new(Vec::new(), true, true, true),
    ];
    let frame = RenderFrame::with_layers(8, 8, [0., 0., 0., 1.], layers).unwrap();
    let inputs = || ImmediateViewInputs::capture(view.get().unwrap(), [8, 8]);
    let elision = MainElision::capture(7, inputs(), &frame, 2);
    assert_eq!(elision.start(), 2);
    let shells = elision.shell_layers();
    assert_eq!(shells.len(), 2);
    for (shell, span) in shells.iter().zip(&frame.layers) {
        assert!(shell.draws.is_empty());
        assert_eq!(
            (shell.depth_test, shell.depth_write, shell.clear_depth),
            span.depth_policy()
        );
    }
    assert!(elision.holds(7, &inputs()));
    assert!(
        !elision.holds(8, &inputs()),
        "a later notice ends the elision"
    );
}
