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
mod tests {
    use super::*;
    #[test]
    fn default_and_explicit_v1_preserve_replay() {
        for value in [None, Some(OsStr::new("hardware-quad-v1"))] {
            assert_eq!(
                selected(value).unwrap(),
                RenderPointRasterBehavior::HardwareQuadV1
            );
        }
    }
    #[test]
    fn fixed_grid_is_explicit_and_versioned() {
        let behavior = selected(Some(OsStr::new("fixed-grid-aliased-v2"))).unwrap();
        assert_eq!(behavior, RenderPointRasterBehavior::FixedGridAliasedV2);
        assert_eq!(
            metadata(behavior),
            "{\"point_raster_behavior\":\"FixedGridAliasedV2\",\"behavior_version\":2}"
        );
    }
    #[test]
    fn unit12_grid_is_explicit_and_versioned() {
        let behavior = selected(Some(OsStr::new("fixed-grid-unit12-v3"))).unwrap();
        assert_eq!(behavior, RenderPointRasterBehavior::FixedGridUnit12V3);
        assert_eq!(behavior.behavior_version(), 3);
    }
    #[test]
    fn malformed_selection_refuses_before_driver_creation() {
        for value in ["", "true", "sphere", "fixed-grid-aliased-v3"] {
            let error = selected(Some(OsStr::new(value))).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
            assert!(error.to_string().contains(value));
        }
    }
}
