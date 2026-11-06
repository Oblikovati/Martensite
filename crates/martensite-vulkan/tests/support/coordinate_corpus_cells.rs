//! Four independent selections replay existing fixtures without changing their assertions.
use martensite_tkernel::standard::Handle;
use martensite_vulkan::*;

/// One provoking/attachment selection; `(RenderRasterPolicy::LegacyFirstV1, coordinates)`.
pub(crate) type CoordinateCell = (RenderRasterPolicy, RenderCoordinateConvention);

pub(crate) fn replay_cells(
    policies: RenderPipelinePolicies,
    replay: impl Fn(&Handle<Graphic3dRenderDriver>),
) {
    let failures = failed_cells(policies, &coordinate_cells(), replay);
    assert!(
        failures.is_empty(),
        "failed coordinate cells {failures:?}; expected every original fixture assertion"
    );
}

/// Replay each cell and return the failing ones; `failed_cells(policies, &cells, replay)`.
pub(crate) fn failed_cells(
    policies: RenderPipelinePolicies,
    cells: &[CoordinateCell],
    replay: impl Fn(&Handle<Graphic3dRenderDriver>),
) -> Vec<CoordinateCell> {
    let mut failures = Vec::new();
    for &(raster, coordinates) in cells {
        let selected = RenderPipelinePolicies { raster, ..policies };
        if !observed_cell(selected, coordinates, &replay) {
            failures.push((raster, coordinates));
        }
    }
    failures
}

/// Cells using one attachment mapping under both provoking conventions.
/// Example: `provoking_cells(RenderCoordinateConvention::TopLeftLogicalV2)`.
pub(crate) fn provoking_cells(coordinates: RenderCoordinateConvention) -> [CoordinateCell; 2] {
    [
        (RenderRasterPolicy::LegacyFirstV1, coordinates),
        (RenderRasterPolicy::LastProvokingVertexV2, coordinates),
    ]
}

fn observed_cell(
    policies: RenderPipelinePolicies,
    coordinates: RenderCoordinateConvention,
    replay: &impl Fn(&Handle<Graphic3dRenderDriver>),
) -> bool {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        replay_cell(policies, coordinates, replay);
    }));
    println!(
        "{{\"raster\":\"{:?}\",\"coordinates\":\"{coordinates:?}\",\"passed\":{}}}",
        policies.raster,
        result.is_ok()
    );
    result.is_ok()
}

fn replay_cell(
    policies: RenderPipelinePolicies,
    coordinates: RenderCoordinateConvention,
    replay: &impl Fn(&Handle<Graphic3dRenderDriver>),
) {
    let driver =
        Graphic3dRenderDriver::with_coordinate_configuration(cell_options(), policies, coordinates)
            .unwrap();
    replay(&driver);
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}

fn coordinate_cells() -> [CoordinateCell; 4] {
    use RenderCoordinateConvention::{LegacyBottomUpV1, TopLeftLogicalV2};
    use RenderRasterPolicy::{LastProvokingVertexV2, LegacyFirstV1};
    [
        (LegacyFirstV1, LegacyBottomUpV1),
        (LegacyFirstV1, TopLeftLogicalV2),
        (LastProvokingVertexV2, LegacyBottomUpV1),
        (LastProvokingVertexV2, TopLeftLogicalV2),
    ]
}

fn cell_options() -> VulkanOptions {
    // All independent cells retain the same real device and validation requirements.
    VulkanOptions {
        validation: true,
        device_index: Some(0),
    }
}

pub(crate) fn record_capture(
    driver: &Handle<Graphic3dRenderDriver>,
    capture: &FrameCapture,
    label: &str,
) {
    let Some(root) = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR") else {
        return;
    };
    let directory = capture_directory(root, driver);
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join(format!("{label}.rgba")), &capture.rgba).unwrap();
    let depth: Vec<u8> = capture
        .depth
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    std::fs::write(directory.join(format!("{label}.depth")), depth).unwrap();
}

fn capture_directory(
    root: std::ffi::OsString,
    driver: &Handle<Graphic3dRenderDriver>,
) -> std::path::PathBuf {
    // Each immutable policy cell retains its own raw bytes, including failed comparisons.
    std::path::PathBuf::from(root).join(format!(
        "{:?}-{:?}",
        driver.raster_policy().unwrap(),
        driver.coordinate_convention().unwrap()
    ))
}
