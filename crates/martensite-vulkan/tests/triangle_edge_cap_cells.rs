//! The strict cap assets grade each attachment mapping separately (decision D1, 2026-10-27).
//! TopLeftLogicalV2, the default, must match every native asset under both provoking
//! conventions. LegacyBottomUpV1 stays selectable for replay; its recorded native mismatches
//! are pinned exactly so a change in either direction is visible.
use super::*;
use crate::coordinate_corpus_cells::{CoordinateCell, failed_cells, provoking_cells};

#[test]
fn default_coordinate_cells_pass_every_unchanged_strict_cap_asset() {
    let cells = provoking_cells(RenderCoordinateConvention::default());
    let mut failures = Vec::new();
    for asset in STRICT_TEXTURE_ENVIRONMENT_ASSETS {
        let failed = failed_cap_cells(asset, &cells);
        report(asset.0, "default", &failed);
        if !failed.is_empty() {
            failures.push((asset.0, failed));
        }
    }
    assert!(
        failures.is_empty(),
        "failed strict cap assets {failures:?}; expected unchanged native assertions"
    );
}

#[test]
fn legacy_coordinate_cells_keep_their_recorded_cap_asset_mismatches() {
    let expected = legacy_mismatches(&probe_device_name());
    let cells = provoking_cells(RenderCoordinateConvention::LegacyBottomUpV1);
    let mut observed = Vec::new();
    for asset in STRICT_TEXTURE_ENVIRONMENT_ASSETS {
        let failed = failed_cap_cells(asset, &cells);
        report(asset.0, "legacy", &failed);
        if !failed.is_empty() {
            observed.push((asset.0, failed.len()));
        }
    }
    assert_eq!(
        observed, expected,
        "legacy strict cap mismatches (asset, failed provoking cells) changed"
    );
}

fn failed_cap_cells(
    asset: NativeTriangleEdgeAsset,
    cells: &[CoordinateCell],
) -> Vec<CoordinateCell> {
    let (texture_coordinates, mesh_edges) = cap_policy::cap_fixture_pipelines(asset.0);
    let policies = RenderPipelinePolicies {
        texture_coordinates,
        mesh_edges,
        ..Default::default()
    };
    failed_cells(policies, cells, |driver| {
        let (mode, rgba, depth) = select_cap_asset(driver, asset);
        let capture = capture_cap_boundary(driver, mode);
        crate::coordinate_corpus_cells::record_capture(driver, &capture, mode);
        native_golden::assert_capture(&capture, rgba, depth, mode);
    })
}

fn report(asset: &str, selection: &str, failed: &[CoordinateCell]) {
    println!(
        "{{\"strict_cap_asset\":\"{asset}\",\"selection\":\"{selection}\",\"failed_cells\":{}}}",
        failed.len()
    );
}

fn probe_device_name() -> String {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    driver.device_name()
}

/// Native mismatches of the legacy mapping recorded in
/// the recorded controls (11/5 software, 13/3 AMD).
fn legacy_mismatches(device: &str) -> Vec<(&'static str, usize)> {
    // Reviewed 2026-10-27: with the native float clip chain (P * W * M in float, column sums in
    // order) AMD keeps both cells of the three shared mismatches. Reviewed 2026-10-28: with
    // 24-bit depth and OpenGL's clip-depth mapping (native GL_DEPTH24_STENCIL8; lavapipe offers
    // it, RADV does not) every legacy cell matches the native assets on llvmpipe.
    if device.contains("llvmpipe") {
        return Vec::new();
    }
    vec![
        ("cap-section-edges-oblique-dashed", 2),
        ("cap-section-edges-oblique-dashed-both-offset", 2),
        ("cap-section-edges-oblique-generated-polygon-thin", 2),
    ]
}
