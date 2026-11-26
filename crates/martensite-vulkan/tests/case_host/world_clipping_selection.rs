//! Select the fragment-discard or raster-stage world clipping path once per worker.
use martensite_vulkan::RenderWorldClippingBehavior;
use std::{ffi::OsStr, io};

pub(super) const SELECTION: &str = "MARTENSITE_CASE_WORLD_CLIPPING_BEHAVIOR";

pub(super) fn selected(value: Option<&OsStr>) -> io::Result<RenderWorldClippingBehavior> {
    match value {
        None => Ok(RenderWorldClippingBehavior::FragmentV1),
        Some(value) if value == "fragment-v1" => Ok(RenderWorldClippingBehavior::FragmentV1),
        Some(value) if value == "raster-clip-distance-v2" => {
            Ok(RenderWorldClippingBehavior::RasterClipDistanceV2)
        }
        Some(value) if value == "raster-clip-distance-v3" => {
            Ok(RenderWorldClippingBehavior::RasterClipDistanceV3)
        }
        Some(value) if value == "raster-fixed-function-v4" => {
            Ok(RenderWorldClippingBehavior::RasterFixedFunctionV4)
        }
        Some(value) if value == "raster-fixed-function-v5" => {
            Ok(RenderWorldClippingBehavior::RasterFixedFunctionClipSpaceV5)
        }
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, fragment-v1, raster-clip-distance-v2, raster-clip-distance-v3, raster-fixed-function-v4 or raster-fixed-function-v5"
            ),
        )),
    }
}

pub(super) fn metadata(behavior: RenderWorldClippingBehavior) -> String {
    format!(
        "{{\"world_clipping_behavior\":\"{behavior:?}\",\"behavior_version\":{}}}",
        behavior.behavior_version()
    )
}

#[cfg(test)]
support_unit_tests! {
    #[path = "world_clipping_selection_tests.rs"]
    mod tests;
}
