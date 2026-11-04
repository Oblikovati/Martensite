//! Scoped strict native edge/line assets replay on its own; full precision diagnostics are separate.
#[path = "support/cap_edge_infinity_reference.rs"]
mod cap_edge_infinity_reference;
#[path = "support/capping_texture_policy.rs"]
mod cap_policy;
#[allow(dead_code)]
#[path = "support/capping_texture_scene.rs"]
mod capping_scene;
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/triangle_edge_scene.rs"]
mod edge_scene;
#[path = "support/ffp_reference_profile.rs"]
mod ffp_reference_profile;
#[path = "../examples/support/native_golden.rs"]
pub(crate) mod native_golden;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/surface_texture_scene.rs"]
mod texture_scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::*;
macro_rules! edge_asset {
    ($mode:literal) => {
        (
            $mode,
            include_bytes!(concat!("fixtures/triangle-edges/host/", $mode, ".rgba")).as_slice(),
            include_bytes!(concat!("fixtures/triangle-edges/host/", $mode, ".depth")).as_slice(),
            include_bytes!(concat!("fixtures/triangle-edges/software/", $mode, ".rgba")).as_slice(),
            include_bytes!(concat!(
                "fixtures/triangle-edges/software/",
                $mode,
                ".depth"
            ))
            .as_slice(),
        )
    };
}
type NativeTriangleEdgeAsset = (
    &'static str,
    &'static [u8],
    &'static [u8],
    &'static [u8],
    &'static [u8],
);
const STRICT_EDGE_ASSETS: &[NativeTriangleEdgeAsset] = &[
    edge_asset!("back-cull"),
    edge_asset!("blend-alpha"),
    edge_asset!("bounds-dashed"),
    edge_asset!("dashed"),
    edge_asset!("feather-two"),
    edge_asset!("feather-zero"),
    edge_asset!("front-cull"),
    edge_asset!("gouraud"),
    edge_asset!("line"),
    edge_asset!("line-blend"),
    edge_asset!("line-facet"),
    edge_asset!("line-gouraud"),
    edge_asset!("line-mask"),
    edge_asset!("line-mask-blend"),
    edge_asset!("line-node-alpha"),
    edge_asset!("line-node-alpha-blend"),
    edge_asset!("line-node-alpha-mask"),
    edge_asset!("line-node-alpha-mask-blend"),
    edge_asset!("line-phong"),
    edge_asset!("line-phong-mat-alpha"),
    edge_asset!("line-phong-mat-alpha-blend"),
    edge_asset!("line-phong-no-modulate"),
    edge_asset!("line-phong-node-alpha-mask-blend"),
    edge_asset!("line-phong-unmapped"),
    edge_asset!("line-resolution-144"),
    edge_asset!("line-unmapped"),
    edge_asset!("line-vertex"),
    edge_asset!("mask-blend-alpha"),
    edge_asset!("polygon"),
    edge_asset!("polygon-alpha"),
    edge_asset!("polygon-background"),
    edge_asset!("polygon-mask-blend"),
    edge_asset!("polygon-wide"),
    edge_asset!("polyline"),
    edge_asset!("resolution-144"),
    edge_asset!("vertex"),
    edge_asset!("negative-alpha"),
];
#[test]
fn scoped_native_edge_and_mapped_line_assets_replay_standalone() {
    for policy in [
        RenderMeshEdgePipeline::Programmable,
        RenderMeshEdgePipeline::PolygonCompatibility,
    ] {
        let driver = Graphic3dRenderDriver::with_mesh_edge_pipeline(
            device_options::validation_options(),
            policy,
        )
        .unwrap();
        replay_edge_policy(&driver, policy);
        assert!(
            driver.validation_errors().unwrap().is_empty(),
            "{:?}",
            driver.validation_errors().unwrap()
        );
    }
}
fn replay_edge_policy(driver: &Handle<Graphic3dRenderDriver>, policy: RenderMeshEdgePipeline) {
    for (mode, host_rgba, host_depth, software_rgba, software_depth) in STRICT_EDGE_ASSETS {
        if mode.contains("polygon") != (policy == RenderMeshEdgePipeline::PolygonCompatibility) {
            continue;
        }
        let (rgba, depth) = if driver.device_name().contains("llvmpipe") {
            (*software_rgba, *software_depth)
        } else {
            (*host_rgba, *host_depth)
        };
        native_golden::assert_capture(&capture_edge_scene(driver, mode), rgba, depth, mode);
    }
}
fn capture_edge_scene(driver: &Handle<Graphic3dRenderDriver>, mode: &str) -> FrameCapture {
    let fixture = edge_scene::TriangleEdgeScene::new(driver, mode);
    let view = fixture.fixture.scene.cview();
    view.set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    view.set_z_layer_redraw_mode(true);
    let render = fixture.fixture.scene.render_view();
    render.try_redraw().unwrap();
    let capture = render.capture().unwrap();
    fixture.fixture.scene.view.remove();
    capture
}

const STRICT_CAP_BINDING_ASSETS: &[NativeTriangleEdgeAsset] = &[
    edge_asset!("cap-oblique-mapped-line"),
    edge_asset!("cap-oblique-mapped-line-image-updated"),
    edge_asset!("cap-oblique-mapped-line-image-updated-line-synchronized"),
    edge_asset!("cap-oblique-mapped-line-image-refreshed-line-synchronized"),
];
#[test]
fn mapped_line_shared_cap_readiness_replays_native_stale_and_synchronized_assets() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    for (mode, host_rgba, host_depth, software_rgba, software_depth) in STRICT_CAP_BINDING_ASSETS {
        let (rgba, depth) = if driver.device_name().contains("llvmpipe") {
            (*software_rgba, *software_depth)
        } else {
            (*host_rgba, *host_depth)
        };
        native_golden::assert_capture(&capture_cap_boundary(&driver, mode), rgba, depth, mode);
    }
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
fn capture_cap_boundary(driver: &Handle<Graphic3dRenderDriver>, mode: &str) -> FrameCapture {
    let fixture = capping_scene::CappingTextureScene::new(driver, mode);
    fixture
        .scene
        .cview()
        .set_z_layer_target(GRAPHIC3D_ZLAYER_ID_DEFAULT);
    fixture.scene.cview().set_z_layer_redraw_mode(true);
    let render = fixture.scene.render_view();
    render.try_redraw().unwrap();
    fixture.mutate(mode);
    render.try_redraw().unwrap();
    let capture = render.capture().unwrap();
    fixture.scene.view.remove();
    capture
}
macro_rules! cap_asset {
    ($mode:literal) => {
        (
            $mode,
            include_bytes!(concat!("fixtures/texture-cap-edges/host/", $mode, ".rgba")).as_slice(),
            include_bytes!(concat!("fixtures/texture-cap-edges/host/", $mode, ".depth")).as_slice(),
            include_bytes!(concat!(
                "fixtures/texture-cap-edges/software/",
                $mode,
                ".rgba"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "fixtures/texture-cap-edges/software/",
                $mode,
                ".depth"
            ))
            .as_slice(),
        )
    };
}
#[test]
fn generated_cap_coordinates_preserve_native_fixed_function_mesh_selection() {
    let driver = Graphic3dRenderDriver::with_surface_pipelines(
        device_options::validation_options(),
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::Programmable,
    )
    .unwrap();
    let mode = "cap-section-edges-oblique-generated";
    let (_, rgba, depth) =
        select_cap_asset(&driver, cap_asset!("cap-section-edges-oblique-generated"));
    native_golden::assert_capture(&capture_cap_boundary(&driver, mode), rgba, depth, mode);
    assert!(driver.validation_errors().unwrap().is_empty());
}

fn select_cap_asset(
    driver: &Handle<Graphic3dRenderDriver>,
    asset: NativeTriangleEdgeAsset,
) -> (&'static str, &'static [u8], &'static [u8]) {
    let (mode, host_rgba, host_depth, software_rgba, software_depth) = asset;
    if let Some((rgba, depth)) = ffp_reference_profile::profiled_driver_asset(
        driver,
        mode,
        (host_rgba, host_depth),
        (software_rgba, software_depth),
    ) {
        return (mode, rgba, depth);
    }
    if driver.device_name().contains("llvmpipe") {
        return (mode, software_rgba, software_depth);
    }
    (mode, host_rgba, host_depth)
}
#[test]
fn native_polygon_texture_environment_and_eye_matrix_replay_standalone() {
    for asset in STRICT_TEXTURE_ENVIRONMENT_ASSETS {
        replay_cap_configured_asset(asset);
    }
}
const STRICT_TEXTURE_ENVIRONMENT_ASSETS: [NativeTriangleEdgeAsset; 16] = [
    cap_asset!("cap-section-edges-oblique-dashed"),
    cap_asset!("cap-section-edges-oblique-dashed-both-offset"),
    cap_asset!("cap-section-edges-oblique-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated"),
    cap_asset!("cap-section-edges-oblique-generated-eye"),
    cap_asset!("cap-section-edges-oblique-generated-eye-polygon"),
    cap_asset!("cap-section-edges-oblique-generated-eye-polygon-thin-alpha-updated"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha-f-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-gray-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-gray-no-modulate"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-rgb-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-rgb-no-modulate"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-no-modulate"),
    cap_asset!("cap-section-edges-oblique-shader-eye-polygon-thin-alpha-updated"),
];

#[test]
fn full_textured_cap_edge_strict_native_diagnostic() {
    let mut failures = Vec::new();
    for asset in FULL_CAP_EDGE_ASSETS {
        // Programmable cap edges reach w = 0; cap_infinity grades them by their analytic image.
        if cap_infinity::INFINITE_EDGE_MODES.contains(&asset.0) {
            continue;
        }
        if let Err(failure) = assess_cap_configured_asset(*asset) {
            eprintln!("{failure}");
            failures.push(failure);
        }
    }
    assert!(
        failures.is_empty(),
        "{} cap failures: {failures:#?}",
        failures.len()
    );
}
/// Grade one cap asset: native-exact or proven ties only; `assess_cap_configured_asset(a)`.
fn assess_cap_configured_asset(asset: NativeTriangleEdgeAsset) -> Result<(), String> {
    let (coordinates, edges) = cap_policy::cap_fixture_pipelines(asset.0);
    let driver = Graphic3dRenderDriver::with_surface_pipelines(
        device_options::validation_options(),
        coordinates,
        edges,
    )
    .unwrap();
    let asset = cap_offset::line_offset_reference(&driver, asset);
    let (mode, rgba, depth) = select_cap_asset(&driver, asset);
    let capture = capture_cap_boundary(&driver, mode);
    let grade = cap_loci::grade_with_controls(&driver, mode, &capture, (rgba, depth))
        .unwrap_or_else(|| {
            crate::exact_tie_reference::grade_native_capture(&capture, rgba, depth, &[])
        });
    let errors = driver.validation_errors().unwrap();
    if !errors.is_empty() {
        return Err(format!("{mode}: validation {errors:?}; expected none"));
    }
    grade
        .map(|_| ())
        .map_err(|failure| format!("{mode}: {failure}"))
}
fn replay_cap_configured_asset(asset: NativeTriangleEdgeAsset) {
    let (coordinates, edges) = cap_policy::cap_fixture_pipelines(asset.0);
    let driver = Graphic3dRenderDriver::with_surface_pipelines(
        device_options::validation_options(),
        coordinates,
        edges,
    )
    .unwrap();
    let (mode, rgba, depth) = select_cap_asset(&driver, asset);
    native_golden::assert_capture(&capture_cap_boundary(&driver, mode), rgba, depth, mode);
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}
#[test]
fn native_fixed_filter_mipmap_and_alpha_assets_replay_standalone() {
    for asset in [
        cap_asset!(
            "cap-section-edges-generated-polygon-thin-fixed-bilinear-fixed-center-edges-disabled"
        ),
        cap_asset!(
            "cap-section-edges-generated-polygon-thin-fixed-trilinear-fixed-minified-edges-disabled"
        ),
        cap_asset!(
            "cap-section-edges-generated-polygon-thin-fixed-trilinear-fixed-minified-no-modulate-edges-disabled"
        ),
        cap_asset!(
            "cap-section-edges-generated-polygon-thin-fixed-trilinear-fixed-minified-image-refreshed-edges-disabled"
        ),
        cap_asset!("cap-section-edges-generated-polygon-thin-blend-edges-disabled"),
        cap_asset!("cap-section-edges-generated-polygon-thin-mask-edges-disabled"),
        cap_asset!("cap-section-edges-generated-polygon-thin-mask-blend-edges-disabled"),
        cap_asset!("cap-section-edges-generated-polygon-thin-opaque-alpha-edges-disabled"),
    ] {
        replay_cap_configured_asset(asset);
    }
}

const FULL_CAP_EDGE_ASSETS: &[NativeTriangleEdgeAsset] = &[
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-sampler-updated"),
    cap_asset!(
        "cap-section-edges-generated-polygon-thin-fixed-bilinear-fixed-center-edges-disabled"
    ),
    cap_asset!(
        "cap-section-edges-generated-polygon-thin-fixed-trilinear-fixed-minified-edges-disabled"
    ),
    cap_asset!(
        "cap-section-edges-generated-polygon-thin-fixed-trilinear-fixed-minified-no-modulate-edges-disabled"
    ),
    cap_asset!(
        "cap-section-edges-generated-polygon-thin-fixed-trilinear-fixed-minified-image-refreshed-edges-disabled"
    ),
    cap_asset!("cap-section-edges-generated-polygon-thin-blend-edges-disabled"),
    cap_asset!("cap-section-edges-generated-polygon-thin-mask-edges-disabled"),
    cap_asset!("cap-section-edges-generated-polygon-thin-mask-blend-edges-disabled"),
    cap_asset!("cap-section-edges-generated-polygon-thin-opaque-alpha-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-bc1-rgb"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-bc1-rgba"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-bc2"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-bc3"),
    cap_asset!("cap-section-edges"),
    cap_asset!("cap-section-edges-oblique"),
    cap_asset!("cap-section-edges-oblique-dashed"),
    cap_asset!("cap-section-edges-oblique-skip-first"),
    cap_asset!("cap-section-edges-oblique-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-edge-alpha"),
    cap_asset!("cap-section-edges-oblique-negative-alpha"),
    cap_asset!("cap-section-edges-oblique-edge-background"),
    cap_asset!("cap-section-edges-oblique-mask"),
    cap_asset!("cap-section-edges-oblique-mask-blend"),
    cap_asset!("cap-section-edges-oblique-blend"),
    cap_asset!("cap-section-edges-oblique-phong"),
    cap_asset!("cap-section-edges-oblique-gouraud"),
    cap_asset!("cap-section-edges-oblique-facet"),
    cap_asset!("cap-section-edges-oblique-object-material"),
    cap_asset!("cap-section-edges-oblique-object-texture"),
    cap_asset!("cap-section-edges-oblique-no-modulate"),
    cap_asset!("cap-section-edges-oblique-polygon"),
    cap_asset!("cap-section-edges-oblique-polygon-fill-offset"),
    cap_asset!("cap-section-edges-oblique-dashed-fill-offset"),
    cap_asset!("cap-section-edges-oblique-dashed-line-offset"),
    cap_asset!("cap-section-edges-oblique-dashed-both-offset"),
    cap_asset!("cap-section-edges-oblique-generated"),
    cap_asset!("cap-section-edges-oblique-generated-eye"),
    cap_asset!("cap-section-edges-oblique-generated-polygon"),
    cap_asset!("cap-section-edges-oblique-generated-eye-polygon"),
    cap_asset!("cap-section-edges-oblique-image-updated"),
    cap_asset!("cap-section-edges-oblique-image-refreshed"),
    cap_asset!("cap-section-edges-oblique-sampler-updated"),
    cap_asset!("cap-section-edges-oblique-plane-updated"),
    cap_asset!("cap-section-edges-oblique-alpha-updated"),
    cap_asset!("cap-section-edges-oblique-bc1-rgba"),
    cap_asset!("cap-section-edges-oblique-bc3"),
    cap_asset!("cap-section-edges-oblique-source-mask"),
    cap_asset!("cap-section-edges-oblique-source-edges"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin"),
    cap_asset!("cap-section-edges-oblique-generated-eye-polygon-thin-alpha-updated"),
    cap_asset!("cap-section-edges-oblique-shader-eye-polygon-thin-alpha-updated"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha-f"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha-f-no-modulate"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha-no-modulate"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-gray"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-gray-no-modulate"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-rgb"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-rgb-no-modulate"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha-f-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-alpha-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-gray-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-format-rgb-edges-disabled"),
    cap_asset!("cap-section-edges-oblique-generated-polygon-thin-no-modulate"),
];
#[path = "triangle_edge_cap_cells.rs"]
mod cap_cells;
#[path = "triangle_edge_cap_infinity.rs"]
mod cap_infinity;
#[path = "triangle_edge_cap_loci.rs"]
mod cap_loci;
#[path = "triangle_edge_cap_offset.rs"]
mod cap_offset;
