//! Retained parameter transitions and native OIT buffers replay on its own.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[path = "../examples/support/native_golden.rs"]
mod native_golden;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/surface_texture_scene.rs"]
mod surface_texture_scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use surface_texture_scene as texture_scene;
#[allow(dead_code)]
#[path = "support/capping_texture_scene.rs"]
mod capping_texture_scene;
#[allow(dead_code)]
#[path = "support/ffp_reference_profile.rs"]
mod ffp_reference_profile;
#[path = "support/oit_basic_assets.rs"]
mod oit_basic_assets;
#[path = "support/oit_finite_assets.rs"]
mod oit_finite_assets;
#[path = "support/oit_layer_assets.rs"]
mod oit_layer_assets;
#[path = "support/oit_reference_profile.rs"]
mod oit_reference_profile;
#[allow(dead_code)]
#[path = "support/oit_scene.rs"]
mod oit_scene;
#[path = "support/oit_transition_assets.rs"]
mod oit_transition_assets;
#[path = "support/recording_device.rs"]
mod recording_device;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_DEFAULT, Graphic3dCViewMembers, Graphic3dGraphicDriver,
};
use martensite_vulkan::{
    FrameCapture, Graphic3dRenderDriver, RenderLightingPipeline, RenderTextureCoordinatePipeline,
    RenderTransparency,
};

#[test]
fn retained_unordered_weighted_and_peeling_native_buffers_replay_standalone() {
    for fixed in [false, true] {
        let driver = oit_driver(fixed);
        assert_basic_oit_assets(&driver, fixed);
        assert!(
            driver.validation_errors().unwrap().is_empty(),
            "{:?}",
            driver.validation_errors().unwrap()
        );
    }
}
fn assert_basic_oit_assets(driver: &Handle<Graphic3dRenderDriver>, fixed: bool) {
    let software = oit_reference_profile::software_reference(driver);
    for &(method, policy, host_rgba, host_depth, soft_rgba, soft_depth) in
        oit_basic_assets::OIT_BASIC_ASSETS
    {
        if policy != fixed {
            continue;
        }
        let (rgba, depth) = if software {
            (soft_rgba, soft_depth)
        } else {
            (host_rgba, host_depth)
        };
        native_golden::assert_capture(&capture_oit_basic(driver, method), rgba, depth, method);
    }
}
fn oit_driver(fixed: bool) -> Handle<Graphic3dRenderDriver> {
    Graphic3dRenderDriver::with_render_pipelines(
        device_options::validation_options(),
        if fixed {
            RenderTextureCoordinatePipeline::GeneratedCompatibility
        } else {
            RenderTextureCoordinatePipeline::Programmable
        },
        Default::default(),
        if fixed {
            RenderLightingPipeline::FixedFunction
        } else {
            RenderLightingPipeline::Programmable
        },
    )
    .unwrap()
}
fn capture_oit_basic(driver: &Handle<Graphic3dRenderDriver>, method: &str) -> FrameCapture {
    let fixture = oit_scene::OitScene::new(driver, false);
    fixture.parameters(method, 0, 0., 4);
    fixture
        .scene
        .cview()
        .set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    fixture.scene.cview().set_z_layer_redraw_mode(true);
    fixture.scene.render_view().try_redraw().unwrap();
    let capture = fixture.scene.render_view().capture().unwrap();
    fixture.scene.view.remove();
    capture
}

#[test]
fn actual_view_parameter_replacements_preserve_previous_submissions() {
    let backend = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(backend.clone(), 128).unwrap();
    let fixture = oit_scene::OitScene::new(&driver, false);
    fixture.parameters("weighted", 4, 0.75, 4);
    fixture.scene.render_view().try_redraw().unwrap();
    let before = backend.last_frame();
    fixture.parameters("peeling", 0, 0.25, 7);
    fixture.scene.render_view().try_redraw().unwrap();
    assert_parameter_snapshots(&before, &backend);
    fixture.scene.view.remove();
}

fn assert_parameter_snapshots(
    before: &martensite_vulkan::RenderFrame,
    backend: &recording_device::RecordingDevice,
) {
    assert_eq!(
        before.transparency(),
        RenderTransparency::Weighted { depth_factor: 0.75 }
    );
    assert_eq!(
        backend.last_frame().transparency(),
        RenderTransparency::DepthPeeling { layers: 7 }
    );
    assert_eq!(
        backend.frames.read()[0].transparency(),
        before.transparency()
    );
}

#[test]
fn masked_nodal_overlay_stays_before_the_deferred_fill_batch() {
    let backend = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(backend.clone(), 128).unwrap();
    for fixture_name in ["sprite-mask", "line-mask"] {
        let fixture = oit_scene::OitScene::with_scene(&driver, false, fixture_name);
        fixture.parameters("weighted", 0, 0., 4);
        fixture.scene.render_view().try_redraw().unwrap();
        let frame = backend.last_frame();
        assert_eq!(frame.draws().len(), 3, "{fixture_name}: {frame:?}");
        assert_ne!(
            frame.draws()[0].topology(),
            martensite_vulkan::PrimitiveTopology::Triangles
        );
        fixture.scene.view.remove();
    }
}

#[test]
fn invalid_active_view_parameters_fail_before_backend_submission_and_can_recover() {
    let backend = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(backend.clone(), 128).unwrap();
    let fixture = oit_scene::OitScene::new(&driver, false);
    fixture.parameters("peeling", 0, 0., 0);
    let error = fixture.scene.render_view().try_redraw().unwrap_err();
    assert!(error.to_string().contains("positive"), "{error}");
    assert!(backend.frames.read().is_empty());
    fixture.parameters("peeling", 0, 0., 3);
    fixture.scene.render_view().try_redraw().unwrap();
    assert_eq!(
        backend.last_frame().transparency(),
        RenderTransparency::DepthPeeling { layers: 3 }
    );
    fixture.scene.view.remove();
}

#[test]
fn sprite_alpha_context_persists_in_its_view_and_does_not_leak_to_adjacent_views() {
    let driver = oit_driver(false);
    let fixture = oit_scene::OitScene::with_scene(&driver, false, "sprite-mask");
    fixture.parameters("weighted", 0, 0., 4);
    assert_context_pixel(&fixture, [0, 0, 255, 255]);
    fixture.replace_groups("two");
    assert_context_pixel(&fixture, [0, 0, 255, 255]);
    let adjacent = oit_scene::OitScene::new(&driver, false);
    adjacent.parameters("weighted", 0, 0., 4);
    assert_context_pixel(&adjacent, [142, 204, 99, 255]);
    fixture.scene.view.remove();
    adjacent.scene.view.remove();
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
fn assert_context_pixel(fixture: &oit_scene::OitScene, expected: [u8; 4]) {
    assert_eq!(
        capture_retained_fixture(fixture).pixel(20, 20),
        Some(expected)
    );
}
fn capture_retained_fixture(fixture: &oit_scene::OitScene) -> FrameCapture {
    fixture
        .scene
        .cview()
        .set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    fixture.scene.cview().set_z_layer_redraw_mode(true);
    fixture.scene.render_view().try_redraw().unwrap();
    fixture.scene.render_view().capture().unwrap()
}

#[test]
fn native_sprite_cutoff_and_opaque_restore_control_the_later_oit_composite() {
    let driver = oit_driver(false);
    for name in ["sprite-mask", "sprite-low-mask", "sprite-opaque"] {
        for mode in ["weighted", "peeling"] {
            assert_sprite_composite_control(&driver, name, mode);
        }
    }
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
fn assert_sprite_composite_control(driver: &Handle<Graphic3dRenderDriver>, name: &str, mode: &str) {
    let fixture = oit_scene::OitScene::with_scene(driver, false, name);
    fixture.parameters(mode, 0, 0., 4);
    let expected = if name == "sprite-mask" || (name == "sprite-low-mask" && mode == "peeling") {
        [0, 0, 255, 255]
    } else if mode == "weighted" {
        [142, 204, 99, 255]
    } else if driver.device_name().contains("llvmpipe") {
        [187, 165, 0, 255]
    } else {
        [188, 165, 0, 255]
    };
    assert_eq!(
        capture_retained_fixture(&fixture).pixel(20, 20),
        Some(expected),
        "{name}/{mode}"
    );
    fixture.scene.view.remove();
}
#[test]
fn sprite_mask_strict_native_buffers_retain_the_coverage_diagnostic() {
    let driver = oit_driver(false);
    let loci = sprite_row_ties();
    for &(mode, host_rgba, host_depth, soft_rgba, soft_depth) in
        oit_basic_assets::SPRITE_ALPHA_ASSETS
    {
        let fixture = oit_scene::OitScene::with_scene(&driver, false, "sprite-mask");
        fixture.parameters(mode, 0, 0., 4);
        let capture = capture_retained_fixture(&fixture);
        let (rgba, depth) = if driver.device_name().contains("llvmpipe") {
            (soft_rgba, soft_depth)
        } else {
            (host_rgba, host_depth)
        };
        let grade = crate::exact_tie_reference::grade_native_capture(&capture, rgba, depth, &loci);
        assert!(
            grade.is_ok(),
            "{mode}: {grade:?}; expected native or proven ties"
        );
        fixture.scene.view.remove();
    }
    assert!(driver.validation_errors().unwrap().is_empty());
}

/// Horizontal content edges of the overlay sprite on pixel centres; `sprite_row_ties()`.
///
/// oit_overlay draws a Plus marker of scale 3 at world (0, 0), the window centre (32, 32) of the
/// 64 px orthographic view of height 20 (view_setup::configure_camera). A sprite of odd side puts
/// its sides and every texel-row boundary on pixel centres: GL (y-up) and Vulkan (y-down) break
/// both coverage and nearest-texel ties on opposite rows. The loci are the sides and the
/// boundaries between texel rows of different alpha, read from the submitted marker image.
fn sprite_row_ties() -> Vec<crate::exact_tie_reference::TieLocus> {
    use martensite_tkernel::ncollection::Vec4;
    use martensite_tkservice::graphic3d::Graphic3dMarkerImage;
    let marker = Graphic3dMarkerImage::standard_marker(
        martensite_tkservice::aspect::AspectTypeOfMarker::Plus,
        3.,
        Vec4::new(1., 1., 1., 0.5),
    )
    .unwrap();
    let image = marker.get_image();
    let image = image.get().unwrap();
    let alpha_row = |row: usize| -> Vec<f32> {
        let columns = 0..image.width() as i32;
        columns
            .map(|x| image.pixel_color(x, row as i32, false).unwrap().alpha())
            .collect()
    };
    let side = image.height();
    let edges = (0..=side).filter(|&k| k == 0 || k == side || alpha_row(k - 1) != alpha_row(k));
    let top = 32. - side as f64 / 2.;
    edges
        .map(|k| crate::exact_tie_reference::TieLocus::fill_row(top + k as f64))
        .collect()
}

#[test]
fn the_sprite_tie_loci_are_symmetric_pixel_centre_rows() {
    let rows: Vec<f64> = sprite_row_ties()
        .into_iter()
        .map(|locus| match locus {
            crate::exact_tie_reference::TieLocus::FillEdge { c, .. } => -c,
            other => panic!("received {other:?}; expected fill rows"),
        })
        .collect();
    // An odd side: every locus is a pixel centre, and the Plus is symmetric about row 32.
    assert!(rows.iter().all(|y| y.fract() == 0.5), "{rows:?}");
    assert!(rows.iter().all(|y| rows.contains(&(64. - y))), "{rows:?}");
}

#[test]
fn native_actual_layer_sharing_and_depth_clear_flush_replay_standalone() {
    // Each native producer owns a fresh viewer/driver; layer identifiers are driver-global.
    for asset in oit_layer_assets::OIT_LAYER_ASSETS {
        let driver = oit_driver(asset.fixed);
        assert_retained_layer_asset(&driver, asset);
        assert!(
            driver.validation_errors().unwrap().is_empty(),
            "{:?}",
            driver.validation_errors().unwrap()
        );
    }
}

fn assert_retained_layer_asset(
    driver: &Handle<Graphic3dRenderDriver>,
    asset: &oit_layer_assets::OitLayerAsset,
) {
    let fixture = oit_scene::OitScene::with_scene(driver, false, asset.fixture);
    fixture.parameters(asset.mode, 0, 0., 4);
    fixture.scene.render_view().try_redraw().unwrap();
    let capture = fixture.scene.render_view().capture().unwrap();
    let (rgba, depth) = if oit_reference_profile::software_reference(driver) {
        asset.software
    } else {
        asset.host
    };
    native_golden::assert_capture(&capture, rgba, depth, asset.fixture);
    fixture.scene.view.remove();
}

#[test]
fn actual_cap_alpha_mutation_changes_the_next_oit_submission_only() {
    let backend = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(backend.clone(), 128).unwrap();
    let fixture = oit_scene::OitScene::with_scene(&driver, false, "cap-source-mask-alpha-updated");
    fixture.parameters("weighted", 4, 0., 4);
    fixture.scene.render_view().try_redraw().unwrap();
    let before = backend.last_frame();
    fixture.mutate_inputs("cap-source-mask-alpha-updated");
    fixture.scene.render_view().try_redraw().unwrap();
    let after = backend.last_frame();
    assert_ne!(before.draws(), after.draws());
    assert_eq!(before.transparency(), after.transparency());
    // The recording backend has no validation layer; reports that as typed Unavailable.
    let failure = driver.validation_errors().unwrap_err();
    assert_eq!(
        failure.cause(),
        martensite_vulkan::RenderValidationObservationCause::Unavailable
    );
    assert!(!failure.was_enabled());
    fixture.scene.view.remove();
}

#[test]
fn actual_colored_glyph_and_live_cap_inputs_replay_native_buffers_standalone() {
    for asset in oit_finite_assets::OIT_FINITE_ASSETS {
        assert_finite_oit_asset(asset);
    }
}
fn assert_finite_oit_asset(asset: &oit_finite_assets::OitFiniteAsset) {
    let driver = oit_driver(asset.fixed);
    let fixture = oit_scene::OitScene::with_scene(&driver, false, asset.fixture);
    fixture.parameters(asset.method, asset.samples, 0., 4);
    fixture.scene.render_view().try_redraw().unwrap();
    fixture.mutate_inputs(asset.fixture);
    let capture = capture_retained_fixture(&fixture);
    let (rgba, depth) = if oit_reference_profile::software_reference(&driver) {
        asset.software
    } else {
        asset.host
    };
    native_golden::assert_capture(&capture, rgba, depth, asset.fixture);
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
    fixture.scene.view.remove();
}

#[test]
fn actual_same_view_sampling_and_persistent_peeling_fallback_replay_native_assets() {
    let driver = oit_driver(false);
    let fixture = oit_scene::OitScene::new(&driver, false);
    for stage in oit_transition_assets::NATIVE_OIT_TRANSITIONS {
        fixture.parameters(stage.mode, stage.samples, 0., 4);
        let capture = capture_retained_fixture(&fixture);
        assert_transition_contract(&driver, stage, &capture);
    }
    fixture.scene.view.remove();
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
fn assert_transition_contract(
    driver: &Handle<Graphic3dRenderDriver>,
    stage: &oit_transition_assets::NativeOitTransition,
    capture: &FrameCapture,
) {
    let (rgba, depth) = if oit_reference_profile::software_reference(driver) {
        stage.software
    } else {
        stage.host
    };
    if stage.strict {
        native_golden::assert_capture(capture, rgba, depth, stage.name);
        return;
    }
    // Native/Vulkan six/eight-sample edge coverage is a retained raw diagnostic, not a promoted gate.
    assert_eq!(
        capture.pixel(20, 20),
        Some(rgba[(20 * 64 + 20) * 4..][..4].try_into().unwrap())
    );
}

#[test]
fn owned_layer_fixture_teardown_allows_fresh_viewers_on_the_same_driver() {
    let backend = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(backend, 128).unwrap();
    // Native layer-lifetime control returns UNDERLAY, DEFAULT, TOP, TOPMOST, OVERLAY.
    let defaults = vec![-5, 0, -2, -3, -4];
    assert_eq!(driver.z_layers(), defaults);
    for name in ["layer-clear", "layer-shared", "layer-clear"] {
        let fixture = oit_scene::OitScene::with_scene(&driver, false, name);
        assert_eq!(driver.z_layers().len(), defaults.len() + 1);
        fixture.finish();
        assert_eq!(driver.z_layers(), defaults, "{name}");
    }
}

#[test]
fn coordinate_cells_keep_native_weighted_and_peeling_physical_texel_addresses() {
    for fixed in [false, true] {
        let policies = coordinate_oit_policies(fixed);
        crate::coordinate_corpus_cells::replay_cells(policies, |driver| {
            assert_basic_oit_assets(driver, fixed);
        });
    }
}
use martensite_vulkan::{RenderPipelinePolicies, RenderTextureCoordinatePipeline as Texture};
fn coordinate_oit_policies(fixed: bool) -> RenderPipelinePolicies {
    let texture_coordinates = if fixed {
        Texture::GeneratedCompatibility
    } else {
        Texture::Programmable
    };
    let lighting = if fixed {
        RenderLightingPipeline::FixedFunction
    } else {
        RenderLightingPipeline::Programmable
    };
    RenderPipelinePolicies {
        texture_coordinates,
        lighting,
        ..Default::default()
    }
}
