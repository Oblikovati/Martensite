//! Generic worker-wide replay selection leaves every original body unchanged.
use martensite_vulkan::RenderPointRasterBehavior;
use std::{ffi::OsStr, io};
pub(crate) const SELECTION: &str = "MARTENSITE_CASE_POINT_RASTER_BEHAVIOR";
pub(crate) fn selected(value: Option<&OsStr>) -> io::Result<RenderPointRasterBehavior> {
    match value {
        None => Ok(Default::default()),
        Some(value) if value == "hardware-quad-v1" => Ok(RenderPointRasterBehavior::HardwareQuadV1),
        Some(value) if value == "fixed-grid-aliased-v2" => {
            Ok(RenderPointRasterBehavior::FixedGridAliasedV2)
        }
        Some(value) if value == "fixed-grid-unit12-v3" => {
            Ok(RenderPointRasterBehavior::FixedGridUnit12V3)
        }
        Some(value) if value == "fixed-grid-sprites12-v4" => {
            Ok(RenderPointRasterBehavior::FixedGridSprites12V4)
        }
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, hardware-quad-v1, fixed-grid-aliased-v2 fixed-grid-unit12-v3 or fixed-grid-sprites12-v4"
            ),
        )),
    }
}
pub(super) fn metadata(behavior: RenderPointRasterBehavior) -> String {
    format!(
        "{{\"point_raster_behavior\":\"{behavior:?}\",\"behavior_version\":{}}}",
        behavior.behavior_version()
    )
}
#[cfg(test)]
support_unit_tests! {
    #[path = "point_raster_selection_tests.rs"]
    mod tests;
}
