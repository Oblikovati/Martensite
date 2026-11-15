//! Worker-wide named equivalence behavior; every original retains its unchanged body.
use martensite_vulkan::RenderDepthBiasBehavior as Behavior;
use std::{ffi::OsStr, io};
pub(crate) const SELECTION: &str = "MARTENSITE_CASE_DEPTH_BIAS_BEHAVIOR";
pub(super) fn selected(value: Option<&OsStr>) -> io::Result<Behavior> {
    match value {
        None => Ok(Behavior::HardwareFormatV1),
        Some(v) if v == "hardware-format-v1" => Ok(Behavior::HardwareFormatV1),
        Some(v) if v == "native24-units-v2" => Ok(Behavior::Native24UnitsV2),
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, hardware-format-v1 or native24-units-v2"
            ),
        )),
    }
}
pub(super) fn metadata(behavior: Behavior) -> String {
    let unit = if behavior == Behavior::Native24UnitsV2 {
        "native24-2^-23"
    } else {
        "hardware-format"
    };
    format!(
        "{{\"depth_bias_behavior\":\"{behavior:?}\",\"behavior_version\":{},\"unit_profile\":\"{unit}\"}}",
        behavior.behavior_version()
    )
}
pub(crate) fn require_capture_default(value: Option<&OsStr>) -> io::Result<()> {
    let behavior = selected(value)?;
    if behavior == Behavior::HardwareFormatV1 {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "vertex capture depth bias={behavior:?}; expected HardwareFormatV1 admitted capture behavior"
        ),
    ))
}
#[cfg(test)]
#[path = "depth_bias_selection_tests.rs"]
mod tests;
