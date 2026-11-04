//! Pure controls for composing source capabilities with the existing coordinate replay.
use super::run_coordinate_policies;
use martensite_vulkan::{RenderCoordinateConvention, RenderPipelinePolicies, RenderRasterPolicy};

#[test]
fn ordinary_run_preserves_default_coordinate_policies() {
    let (policies, coordinates) = run_coordinate_policies(false);
    assert_eq!(policies, RenderPipelinePolicies::default());
    assert_eq!(coordinates, RenderCoordinateConvention::default());
}

#[test]
fn bottom_up_run_preserves_last_provoking_coordinate_policies() {
    let (policies, coordinates) = run_coordinate_policies(true);
    assert_eq!(policies.raster, RenderRasterPolicy::LastProvokingVertexV2);
    assert_eq!(coordinates, RenderCoordinateConvention::LegacyBottomUpV1);
}
