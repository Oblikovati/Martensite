//! Layer limits reuse the actual native traversal; visual culling does not deactivate picking.
use super::view_setup;
use super::{RecordingDevice, assert_draws, setup};
#[path = "support/ais_scene.rs"]
mod selectable_scene;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Point3, Xyz};
use martensite_tkservice::{aspect::AspectWindow, graphic3d::*};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::Graphic3dRenderDriver;

fn layer_limits(viewer: &V3dViewer, distance: f64, size: f64) {
    let mut settings = viewer
        .z_layer_settings(GRAPHIC3D_ZLAYER_ID_DEFAULT)
        .unwrap();
    settings.set_culling_distance(distance);
    settings.set_culling_size(size);
    viewer
        .set_z_layer_settings(GRAPHIC3D_ZLAYER_ID_DEFAULT, &settings)
        .unwrap();
}

#[test]
fn distance_limits_reach_perspective_but_not_orthographic_draws() {
    let (scene, device) = setup();
    let camera = scene.view.camera();
    camera.set_projection_type(Graphic3dCameraProjection::Perspective);
    for (distance, count) in [(11., 0), (12., 1), (0., 1)] {
        layer_limits(&scene.viewer, distance, 0.);
        assert_draws(&scene, &device, count);
    }
    camera.set_projection_type(Graphic3dCameraProjection::Orthographic);
    layer_limits(&scene.viewer, 11., 0.);
    assert_draws(&scene, &device, 1);
}

#[test]
fn size_limits_preserve_native_on_off_and_no_update_state() {
    use Graphic3dRenderingParamsFrustumCulling::{NoUpdate, Off, On};
    let (scene, device) = setup();
    for (size, state, count) in [
        (16., On, 1),
        (128., On, 0),
        (0., NoUpdate, 0),
        (128., Off, 1),
        (128., NoUpdate, 1),
        (128., On, 0),
    ] {
        layer_limits(&scene.viewer, 0., size);
        scene.view.change_rendering_params(&mut |params| {
            params.ray_tracing.frustum_culling_state = state;
        });
        assert_draws(&scene, &device, count);
    }
}

#[test]
fn size_culling_tracks_actual_viewport_and_camera_scale_without_stale_frames() {
    let (scene, device) = setup();
    layer_limits(&scene.viewer, 0., 80.);
    assert_draws(&scene, &device, 0);
    scene.window.set_size(128, 128);
    scene.view.must_be_resized();
    assert_draws(&scene, &device, 1);
    let retained = device.last_frame();
    scene.window.set_size(128, 64);
    scene.view.must_be_resized();
    assert_draws(&scene, &device, 0);
    scene.view.camera().set_scale(10.);
    assert_draws(&scene, &device, 1);
    assert_eq!(retained.draws().len(), 1);
}

#[test]
fn persistent_structure_still_uses_native_layer_size_policy() {
    let (scene, device) = setup();
    let persistence =
        Graphic3dTransformPers::with_point(Graphic3dTransModeFlags::ZOOM_PERS, &Point3::default())
            .unwrap();
    scene
        .structure
        .set_transform_persistence(&Handle::new(persistence).cast());
    layer_limits(&scene.viewer, 0., 128.);
    assert_draws(&scene, &device, 0);
    layer_limits(&scene.viewer, 0., 16.);
    assert_draws(&scene, &device, 1);
}

#[test]
fn layer_origin_reaches_the_frame_without_changing_the_clip_transform() {
    // Native SetOrigin only moves translation between world-view and model for float precision;
    // the double-precision clip transform is unchanged (the recorded layer_origin capture).
    let (scene, device) = setup();
    scene.render_view().try_redraw().unwrap();
    let plain = *device.last_frame().draws()[0].clip_transform();
    let mut settings = Graphic3dZLayerSettings::default();
    settings.set_origin(Xyz::new(1.0e6, 0., 0.));
    scene
        .viewer
        .set_z_layer_settings(GRAPHIC3D_ZLAYER_ID_DEFAULT, &settings)
        .unwrap();
    scene.render_view().try_redraw().unwrap();
    assert_eq!(*device.last_frame().draws()[0].clip_transform(), plain);
}

fn selectable_setup() -> (selectable_scene::AisViewerScene, RecordingDevice) {
    let device = RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device.clone(), 128).unwrap();
    (selectable_scene::AisViewerScene::new(&driver), device)
}

#[test]
fn visual_size_culling_keeps_display_membership_and_actual_picking() {
    let (scene, device) = selectable_setup();
    assert_eq!(scene.window.size(), (64, 64));
    for (size, count) in [(0., 1), (128., 0)] {
        layer_limits(&scene.viewer, 0., size);
        scene.render_view().try_redraw().unwrap();
        assert_eq!(device.last_frame().draws().len(), count);
        scene.context.move_to(32, 32, &scene.view, false).unwrap();
        assert!(scene.context.is_displayed(&scene.object.cast()));
        assert!(scene.context.has_detected());
        scene.context.clear_detected(false);
    }
}

#[path = "layer_culling_capture.rs"]
mod capture;

#[path = "layer_culling_predicates.rs"]
mod predicates;
