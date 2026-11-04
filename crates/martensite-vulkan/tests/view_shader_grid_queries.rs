//! GlView::ShaderGridEcho (both overloads) and ShaderGridSnapPoint through a live view.
//!
//! The view answers from its displayed grid, its camera and its window size. The test view is
//! 64x64 pixels with a top orthographic camera of scale 20 (10 world units per half view), so
//! pixel (48, 16) lies near world (5.2, 4.8) and its nearest node of a unit grid is (5, 5).
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod viewer_scene;
use martensite_tkmath::gp::Axis3;
use martensite_tkservice::aspect::AspectGridParams;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::{Graphic3dRenderDriver, RenderGridPixel};
use viewer_scene::ViewerScene;

fn scene_with_unit_grid() -> ViewerScene {
    let driver =
        Graphic3dRenderDriver::with_backend(recording_device::RecordingDevice::default(), 128)
            .unwrap();
    let scene = ViewerScene::new(&driver);
    let mut unit = AspectGridParams::default();
    unit.set_scale(1.).unwrap();
    scene.cview().grid_display(&unit, &Axis3::default());
    scene
}

fn xyz(vertex: &Graphic3dVertex) -> [f64; 3] {
    let (x, y, z) = vertex.coord_f64();
    [x, y, z]
}

#[test]
fn view_echo_overloads_snap_the_pixel_to_the_grid_node() {
    let scene = scene_with_unit_grid();
    let view = scene.cview();
    let (mut point, mut display, mut alone) = Default::default();
    assert!(view.shader_grid_echo_display(48, 16, &mut point, &mut display));
    assert!(view.shader_grid_echo(48, 16, &mut alone));
    assert_eq!(xyz(&point), [5., 5., 0.]);
    assert_eq!(xyz(&alone), xyz(&point));
    // The display point is the same node moved along the view axis, as the grid computes it.
    let camera = scene.render_view().camera();
    let query = RenderGridPixel {
        size: [64, 64],
        pixel: [48, 16],
    };
    let echo = scene
        .render_view()
        .shader_grid()
        .echo(camera.get().unwrap(), query);
    let display_node = echo.unwrap().display;
    let expected = [display_node.x(), display_node.y(), display_node.z()];
    assert_eq!(xyz(&display), expected.map(|v| f64::from(v as f32)));
    assert_eq!(&xyz(&display)[..2], &[5., 5.]);
}

#[test]
fn view_snap_point_rounds_to_the_nearest_node() {
    let scene = scene_with_unit_grid();
    let mut node = Graphic3dVertex::default();
    let query = Graphic3dVertex::from_f64(2.4, -2.6, 0.);
    assert!(scene.cview().shader_grid_snap_point(&query, &mut node));
    assert_eq!(xyz(&node), [2., -3., 0.]);
}

#[test]
fn erased_grids_answer_no_echo_and_no_snap() {
    let scene = scene_with_unit_grid();
    let view = scene.cview();
    view.grid_erase();
    let (mut point, mut display) = Default::default();
    assert!(!view.shader_grid_echo_display(48, 16, &mut point, &mut display));
    assert!(!view.shader_grid_echo(48, 16, &mut point));
    let mut node = Graphic3dVertex::default();
    assert!(!view.shader_grid_snap_point(&Graphic3dVertex::from_f64(1., 1., 0.), &mut node));
}
