//! Actual view submissions use source culling, not empty draw counts, to defer depth clears.
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::{Graphic3dRenderDriver, RenderFrame};
use recording_device::RecordingDevice;

fn recorded_scene() -> (scene::ViewerScene, RecordingDevice) {
    let device = RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), 128).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    (scene, device)
}
fn submitted(scene: &scene::ViewerScene, device: &RecordingDevice) -> RenderFrame {
    scene.render_view().try_redraw().unwrap();
    device.last_frame()
}
fn additional_layer(scene: &scene::ViewerScene, clear: bool) -> i32 {
    let mut settings = Graphic3dZLayerSettings::default();
    settings.set_clear_depth(clear);
    scene.viewer.add_z_layer(&settings).unwrap().unwrap()
}
fn layer_triangle(
    scene: &scene::ViewerScene,
    layer: i32,
    color: [f32; 4],
) -> Handle<Graphic3dStructure> {
    let structure =
        Graphic3dStructure::new(&scene.viewer.structure_manager(), &Handle::default()).unwrap();
    let group = structure.new_group().unwrap();
    let style = scene::unlit_style(color);
    if color[3] < 1. {
        style.set_alpha_mode(Graphic3dAlphaMode::Blend, 0.5);
    }
    group.set_group_primitives_aspect(&style);
    group.add_primitive_array(&view_setup::triangle_vertices(), true);
    structure.set_z_layer(layer);
    structure.display();
    structure
}

#[test]
fn trailing_culled_default_layer_does_not_erase_submitted_scene_depth() {
    let (scene, device) = recorded_scene();
    let frame = submitted(&scene, &device);
    assert!(!frame.draws().is_empty());
    let last = frame
        .layers()
        .iter()
        .rposition(|layer| !layer.draw_range().is_empty())
        .unwrap();
    assert!(
        frame.layers()[last + 1..]
            .iter()
            .all(|layer| !layer.depth_policy().2)
    );
}

#[test]
fn empty_clear_is_applied_to_the_next_nonculled_source_layer() {
    let (scene, device) = recorded_scene();
    additional_layer(&scene, true);
    let next = additional_layer(&scene, false);
    let _later = layer_triangle(&scene, next, [0., 0., 1., 1.]);
    let frame = submitted(&scene, &device);
    let clearing: Vec<_> = frame
        .layers()
        .iter()
        .filter(|layer| layer.depth_policy().2)
        .collect();
    assert_eq!(clearing.len(), 1);
    assert!(!clearing[0].draw_range().is_empty());
    assert_eq!(clearing[0].draw_range().end, frame.draws().len());
}

#[test]
fn multiple_culled_clears_collapse_before_the_later_source_draw() {
    let (scene, device) = recorded_scene();
    additional_layer(&scene, true);
    additional_layer(&scene, true);
    let next = additional_layer(&scene, false);
    let _later = layer_triangle(&scene, next, [0., 0., 1., 1.]);
    let frame = submitted(&scene, &device);
    assert_eq!(
        frame
            .layers()
            .iter()
            .filter(|layer| layer.depth_policy().2)
            .count(),
        1
    );
}

#[test]
fn deferred_transparency_flushes_before_the_next_actual_depth_clear() {
    let (scene, device) = recorded_scene();
    let _blend = layer_triangle(&scene, GRAPHIC3D_ZLAYER_ID_DEFAULT, [0., 1., 0., 0.5]);
    additional_layer(&scene, true);
    let next = additional_layer(&scene, false);
    let _later = layer_triangle(&scene, next, [0., 0., 1., 1.]);
    let frame = submitted(&scene, &device);
    assert_blend_before_clear(&frame);
}
fn assert_blend_before_clear(frame: &RenderFrame) {
    let clear = frame
        .layers()
        .iter()
        .position(|layer| layer.depth_policy().2)
        .unwrap();
    assert!(clear > 0);
    let blend = &frame.layers()[clear - 1];
    assert!(!blend.draw_range().is_empty());
    assert!(!blend.depth_policy().1);
    assert!(
        frame.draws()[blend.draw_range()]
            .iter()
            .all(|draw| draw.vertices()[0].color[3] == 0.5)
    );
}

#[test]
fn blank_view_still_submits_initial_attachment_initialization() {
    let (scene, device) = recorded_scene();
    scene.structure.erase();
    let frame = submitted(&scene, &device);
    assert!(frame.draws().is_empty());
    assert!(!frame.layers().is_empty());
    assert_eq!(frame.layers()[0].draw_range(), 0..0);
    assert_eq!(frame.clear_color(), [0., 0., 1., 1.]);
}

#[test]
fn generic_explicit_empty_clear_remains_a_caller_policy() {
    let frame = RenderFrame::with_layers(
        8,
        8,
        [0.; 4],
        vec![martensite_vulkan::RenderLayer::new(
            Vec::new(),
            true,
            true,
            true,
        )],
    )
    .unwrap();
    assert_eq!(frame.layers().len(), 1);
    assert!(frame.layers()[0].depth_policy().2);
}
