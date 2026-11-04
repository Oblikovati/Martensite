//! Public viewer contracts retain gradient snapshots and submit the selected background.
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::{aspect::AspectGradientFillMethod, graphic3d::*};
use martensite_vulkan::Graphic3dRenderDriver;
use recording_device::RecordingDevice;

fn gradient_scene() -> (scene::ViewerScene, RecordingDevice) {
    let device = RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), 128).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    scene.structure.erase();
    (scene, device)
}

fn case_vertical_colors(scene: &scene::ViewerScene) {
    scene.view.set_bg_gradient_colors(
        Color::named(NamedColor::RED),
        Color::named(NamedColor::BLUE),
        AspectGradientFillMethod::Vertical,
        false,
    );
}

#[test]
fn public_gradient_getter_is_an_owned_snapshot_of_the_case_colors() {
    let (scene, _) = gradient_scene();
    case_vertical_colors(&scene);
    let mut snapshot = scene.view.gradient_background();
    assert_eq!(
        snapshot.colors(),
        (
            Color::named(NamedColor::RED),
            Color::named(NamedColor::BLUE)
        )
    );
    change_snapshot(&mut snapshot);
    let retained = scene.view.gradient_background();
    assert_eq!(
        retained.bg_gradient_fill_method(),
        AspectGradientFillMethod::Vertical
    );
    assert_ne!(retained.colors(), snapshot.colors());
}

#[test]
fn public_vertical_gradient_setter_selects_the_active_background_kind() {
    let (scene, _) = gradient_scene();
    case_vertical_colors(&scene);
    assert_eq!(
        scene.cview().background_type(),
        Graphic3dTypeOfBackground::Gradient
    );
    assert_eq!(
        scene.view.gradient_background().bg_gradient_fill_method(),
        AspectGradientFillMethod::Vertical
    );
}

#[test]
fn public_gradient_style_drives_the_existing_frame_without_direct_state_writes() {
    let (scene, device) = gradient_scene();
    case_vertical_colors(&scene);
    scene.render_view().try_redraw().unwrap();
    let frame = device.last_frame();
    assert_eq!(
        frame.draws().len(),
        1,
        "frame {frame:?}, expected one existing background draw"
    );
    scene
        .view
        .set_bg_gradient_style(AspectGradientFillMethod::None, false);
    scene.render_view().try_redraw().unwrap();
    assert!(device.last_frame().draws().is_empty());
}

fn change_snapshot(snapshot: &mut martensite_tkservice::aspect::AspectGradientBackground) {
    snapshot.set_colors(
        Color::named(NamedColor::GREEN),
        Color::named(NamedColor::BLACK),
        AspectGradientFillMethod::Horizontal,
    );
}
