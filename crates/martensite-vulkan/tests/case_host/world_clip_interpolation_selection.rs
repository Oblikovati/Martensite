//! Select the raster-contract world-plane interpolation without changing source cases.
use martensite_vulkan::RenderWorldClipInterpolationBehavior as Behavior;
use std::{ffi::OsStr, io};

pub(crate) const SELECTION: &str = "MARTENSITE_CASE_WORLD_CLIP_INTERPOLATION";

pub(crate) fn selected(value: Option<&OsStr>) -> io::Result<Behavior> {
    match value {
        None => Ok(Behavior::HardwareV1),
        Some(value) if value == "hardware-v1" => Ok(Behavior::HardwareV1),
        Some(value) if value == "projected-grid12-v1" => Ok(Behavior::ProjectedGrid12V1),
        Some(value) if value == "projected-grid12-v2" => Ok(Behavior::ProjectedGrid12V2),
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, hardware-v1, projected-grid12-v1 or projected-grid12-v2"
            ),
        )),
    }
}

pub(crate) fn metadata(behavior: Behavior) -> String {
    format!(
        "{{\"world_clip_interpolation\":\"{behavior:?}\",\"behavior_version\":{}}}",
        behavior.behavior_version()
    )
}

#[cfg(test)]
#[path = "world_clip_interpolation_selection_tests.rs"]
mod tests;
