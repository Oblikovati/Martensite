//! Replay genuine fixed-lighting native assets on its own; retain precision failures separately.
#[path = "support/fixed_caps_assets_0.rs"]
mod cap_assets;
#[path = "support/cap_infinity_reference.rs"]
mod cap_infinity_reference;
#[allow(dead_code)]
#[path = "support/capping_texture_scene.rs"]
mod capping_texture_scene;
#[path = "../src/projected_segments/capture_tests.rs"]
mod capture_tests;
#[path = "support/clip_msaa_geometry.rs"]
mod clip_msaa_geometry;
#[path = "support/cross_driver_bound.rs"]
mod cross_driver_bound;
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[path = "support/ffp_reference_profile.rs"]
mod ffp_reference_profile;
#[path = "support/fixed_cap_lights.rs"]
mod fixed_cap_lights;
#[path = "support/fixed_lighting_amd_bounds.rs"]
mod fixed_lighting_amd_bounds;
#[allow(dead_code)]
#[path = "support/fixed_lighting_scene.rs"]
mod fixed_scene;
#[path = "support/fixed_texture_scene.rs"]
mod fixed_texture_scene;
#[path = "support/fixed_lighting_assets_0.rs"]
mod lighting_assets_0;
#[path = "support/fixed_lighting_assets_1.rs"]
mod lighting_assets_1;
#[path = "support/fixed_lighting_assets_2.rs"]
mod lighting_assets_2;
#[allow(dead_code)]
#[path = "support/lighting_scene.rs"]
mod lighting_scene;
#[path = "../examples/support/native_golden.rs"]
mod native_golden;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[path = "support/fixed_textures_assets_0.rs"]
mod texture_assets;
#[allow(dead_code)]
#[path = "support/surface_texture_scene.rs"]
mod texture_scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::{FrameCapture, Graphic3dRenderDriver};

type NativeFixedLightingAsset = (
    &'static str,
    bool,
    &'static [u8],
    &'static [u8],
    &'static [u8],
    &'static [u8],
);
#[test]
fn fixed_lighting_native_assets_replay_standalone() {
    replay_fixed_assets();
}
fn capture_clip_msaa_control(
    driver: &Handle<Graphic3dRenderDriver>,
    samples: i32,
    directory: &capture_tests::DashCaptureDirectory,
) {
    let mode = format!("lit-clip-msaa-quad-{samples}");
    let fixture = fixed_scene::fixed_lighting_scene(driver, &mode);
    fixture.base.render_view().try_redraw().unwrap();
    assert_eq!(
        fixture.base.cview().rendering_params().nb_msaa_samples,
        samples
    );
    let capture = fixture.base.render_view().capture().unwrap();
    directory.save(&mode, &capture.rgba, &capture.depth);
    fixture.base.view.remove();
}

#[test]
#[ignore = "bounded FFP clipping/MSAA diagnostic; requires the canonical GPU lock"]
fn clip_msaa_quad_records_single_sample_and_eight_sample_images() {
    let directory = capture_tests::DashCaptureDirectory::requested()
        .expect("MARTENSITE_GEOMETRY_ARTIFACT_DIR; expected an owned capture directory");
    let driver = fixed_scene::fixed_lighting_driver(device_options::validation_options()).unwrap();
    assert!(driver.device_limits().unwrap().max_msaa_samples() >= 8);
    capture_clip_msaa_control(&driver, 0, &directory);
    capture_clip_msaa_control(&driver, 8, &directory);
}

#[test]
#[ignore = "bounded authored FFP world-clipping diagnostic; requires the canonical GPU lock"]
fn clip_msaa_quad_raster_clip_v2_records_single_and_eight_sample_images() {
    let directory = capture_tests::DashCaptureDirectory::requested()
        .expect("MARTENSITE_GEOMETRY_ARTIFACT_DIR; expected an owned capture directory");
    let behavior = martensite_vulkan::RenderBehaviorConfiguration::default()
        .with_world_clipping(martensite_vulkan::RenderWorldClippingBehavior::RasterClipDistanceV2);
    let driver = fixed_scene::fixed_lighting_driver_with_behavior(
        device_options::validation_options(),
        behavior,
    )
    .unwrap();
    assert!(driver.device_limits().unwrap().max_msaa_samples() >= 8);
    capture_clip_msaa_control(&driver, 0, &directory);
    capture_clip_msaa_control(&driver, 8, &directory);
}
#[test]
fn fixed_lighting_manual_coordinates_keep_encoded_texture_environment() {
    let driver = Graphic3dRenderDriver::with_render_pipelines(
        device_options::validation_options(),
        martensite_vulkan::RenderTextureCoordinatePipeline::Programmable,
        martensite_vulkan::RenderMeshEdgePipeline::Programmable,
        martensite_vulkan::RenderLightingPipeline::FixedFunction,
    )
    .unwrap();
    let asset = texture_assets::ASSETS
        .iter()
        .find(|row| row.0 == "lit-nearest")
        .unwrap();
    let (_, _, host_rgba, host_depth, software_rgba, software_depth) = *asset;
    let (rgba, depth) = if driver.device_name().contains("llvmpipe") {
        (software_rgba, software_depth)
    } else {
        (host_rgba, host_depth)
    };
    native_golden::assert_capture(
        &capture_fixed_texture(&driver, "lit-nearest"),
        rgba,
        depth,
        "lit-nearest-manual-coordinates",
    );
}
/// Cap vertices at infinity follow the GL 2.1 §2.14.1 point-at-infinity rule (see
/// `support/cap_infinity_reference.rs`); the frame is graded against its analytic image.
#[test]
fn fixed_cap_point_source_preserves_homogeneous_lighting() {
    let driver = fixed_scene::fixed_lighting_driver(device_options::validation_options()).unwrap();
    let mode = "cap-oblique-fixed-phong-source-point";
    let fixture = capping_texture_scene::CappingTextureScene::new(&driver, mode);
    fixed_cap_lights::fixed_cap_light_source(&fixture.scene, mode);
    let inputs = cap_infinity_inputs(&fixture);
    fixture.scene.render_view().try_redraw().unwrap();
    fixture.mutate(mode);
    let capture = capture_fixed_view(&fixture.scene);
    let violation = cap_infinity_reference::cap_infinity_violation(&capture, &inputs);
    assert_eq!(violation, None, "{mode} on {}", driver.device_name());
    assert!(driver.validation_errors().unwrap().is_empty());
}
/// Read the reference inputs through the public API; `cap_infinity_inputs(&fixture)`.
fn cap_infinity_inputs(
    fixture: &capping_texture_scene::CappingTextureScene,
) -> cap_infinity_reference::CapInfinityInputs {
    use martensite_tkernel::quantity::{Color, ColorSpace};
    let aspect = fixture.plane.capping_aspect();
    let material = aspect.front_material();
    assert_eq!(material.specular_color().values(ColorSpace::Rgb), [0.; 3]);
    let interior = aspect.interior_color().values(ColorSpace::Rgb);
    // Native FFP rendering is not sRGB: GlMaterial encodes material⊙interior.
    let encoded = |color: Color| {
        let rgb = color.values(ColorSpace::Rgb);
        [0, 1, 2].map(|k| Color::linear_to_srgb(rgb[k] * interior[k]))
    };
    let (light, scene_ambient) = cap_point_light_and_ambient(&fixture.scene);
    cap_infinity_reference::CapInfinityInputs {
        plane: fixture.plane.get_equation().data(),
        light_position: light.position_coords(),
        attenuation: light.attenuation(),
        light_color: light.color().values(ColorSpace::Rgb),
        scene_ambient,
        emission: encoded(material.emissive_color()),
        ambient: encoded(material.ambient_color()),
        diffuse: encoded(material.diffuse_color()),
        texture_scale: f64::from(fixture.texture.get_params().scale().x()),
        texels: cap_texels(),
    }
}
/// The 2x2 cap image ("nearest" mode) as normalised RGB texels, row-major.
fn cap_texels() -> [[f64; 3]; 4] {
    let bytes = texture_scene::modes::texture_mode_pixels("nearest");
    [0, 1, 2, 3].map(|texel| [0, 1, 2].map(|k| f64::from(bytes[texel * 4 + k]) / 255.))
}
fn cap_point_light_and_ambient(scene: &scene::ViewerScene) -> (Handle<Graphic3dCLight>, [f64; 3]) {
    use martensite_tkernel::quantity::ColorSpace;
    let lights = scene.cview().lights().get().unwrap().lights();
    let mut ambient = [0.; 3];
    let mut point = None;
    for light in lights {
        match light.light_type() {
            Graphic3dTypeOfLightSource::Ambient => {
                let rgb = light.color().values(ColorSpace::Rgb);
                ambient = [0, 1, 2].map(|k| ambient[k] + rgb[k]);
            }
            Graphic3dTypeOfLightSource::Positional => point = Some(light),
            kind => panic!("light {kind:?}; expected only ambient and one positional source"),
        }
    }
    (point.expect("one positional source"), ambient)
}
fn capture_fixed_cap(driver: &Handle<Graphic3dRenderDriver>, mode: &str) -> FrameCapture {
    let fixture = capping_texture_scene::CappingTextureScene::new(driver, mode);
    fixed_cap_lights::fixed_cap_light_source(&fixture.scene, mode);
    fixture.scene.render_view().try_redraw().unwrap();
    fixture.mutate(mode);
    capture_fixed_view(&fixture.scene)
}
fn replay_fixed_assets() {
    let driver = fixed_scene::fixed_lighting_driver(device_options::validation_options()).unwrap();
    let software = driver.device_name().contains("llvmpipe");
    for &(mode, texture, host_rgba, host_depth, software_rgba, software_depth) in fixed_assets() {
        let capture = capture_fixed_asset(&driver, mode, texture);
        let (rgba, depth) = if software {
            (software_rgba, software_depth)
        } else {
            (host_rgba, host_depth)
        };
        let (rgba, depth) = ffp_reference_profile::profiled_driver_asset(
            &driver,
            mode,
            (host_rgba, host_depth),
            (software_rgba, software_depth),
        )
        .unwrap_or((rgba, depth));
        assert_graded_capture(&driver, &capture, (rgba, depth), mode);
    }
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
fn fixed_assets() -> impl Iterator<Item = &'static NativeFixedLightingAsset> {
    lighting_assets_0::ASSETS
        .iter()
        .chain(lighting_assets_1::ASSETS)
        .chain(lighting_assets_2::ASSETS)
        .chain(texture_assets::ASSETS)
        .chain(cap_assets::ASSETS)
}
fn capture_fixed_asset(
    driver: &Handle<Graphic3dRenderDriver>,
    mode: &str,
    texture: bool,
) -> FrameCapture {
    if mode.starts_with("cap-") {
        return capture_fixed_cap(driver, mode);
    }
    if texture {
        return capture_fixed_texture(driver, mode);
    }
    let fixture = fixed_scene::fixed_lighting_scene(driver, mode);
    fixture.base.render_view().try_redraw().unwrap();
    fixture.mutate(mode);
    capture_fixed_view(&fixture.base)
}
fn capture_fixed_texture(driver: &Handle<Graphic3dRenderDriver>, mode: &str) -> FrameCapture {
    let fixture = fixed_texture_scene::fixed_texture_scene(driver, mode);
    fixture.scene.render_view().try_redraw().unwrap();
    fixture.mutate(mode.strip_prefix("lit-").unwrap_or(mode));
    capture_fixed_view(&fixture.scene)
}
fn capture_fixed_view(scene: &scene::ViewerScene) -> FrameCapture {
    let view = scene.cview();
    view.set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    view.set_z_layer_redraw_mode(true);
    scene.render_view().try_redraw().unwrap();
    let capture = scene.render_view().capture().unwrap();
    scene.view.remove();
    capture
}

#[path = "support/marker_back_lighting.rs"]
mod marker_back_lighting;

#[test]
fn coordinate_cells_keep_native_fixed_lighting_derivative_and_facing() {
    let policies = martensite_vulkan::RenderPipelinePolicies {
        lighting: martensite_vulkan::RenderLightingPipeline::FixedFunction,
        texture_coordinates:
            martensite_vulkan::RenderTextureCoordinatePipeline::GeneratedCompatibility,
        ..Default::default()
    };
    crate::coordinate_corpus_cells::replay_cells(policies, |driver| {
        for asset in lighting_assets_0::ASSETS {
            assert_coordinate_lighting_asset(driver, *asset);
        }
    });
}
fn assert_coordinate_lighting_asset(
    driver: &Handle<Graphic3dRenderDriver>,
    asset: NativeFixedLightingAsset,
) {
    let (mode, texture, host, host_depth, soft, soft_depth) = asset;
    let (rgba, depth) = if driver.device_name().contains("llvmpipe") {
        (soft, soft_depth)
    } else {
        (host, host_depth)
    };
    let capture = capture_fixed_asset(driver, mode, texture);
    crate::coordinate_corpus_cells::record_capture(driver, &capture, mode);
    assert_graded_capture(driver, &capture, (rgba, depth), mode);
}
/// Exact native bytes, except a reviewed cross-driver bound on the proven native AMD profile.
fn assert_graded_capture(
    driver: &Handle<Graphic3dRenderDriver>,
    capture: &FrameCapture,
    (rgba, depth): (&[u8], &[u8]),
    mode: &str,
) {
    let identity = driver.driver_identity().unwrap();
    let amd = ffp_reference_profile::ffp_native_reference_profile(&identity)
        == Ok(ffp_reference_profile::FfpNativeReferenceProfile::NativeAmdMesa26);
    let graded = fixed_lighting_amd_bounds::FIXED_LIGHTING_AMD_CROSS_DRIVER_CASES.contains(&mode);
    if amd && graded {
        cross_driver_bound::assert_cross_driver(capture, rgba, depth, mode);
    } else {
        native_golden::assert_capture(capture, rgba, depth, mode);
    }
}
