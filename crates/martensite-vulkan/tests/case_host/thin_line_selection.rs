//! One worker-wide line policy never inspects case names or driver vendors.
use martensite_vulkan::RenderThinLineCoverage as Coverage;
use std::{ffi::OsStr, io};

pub(crate) const SELECTION: &str = "MARTENSITE_CASE_THIN_LINE_COVERAGE";

pub(crate) fn selected(value: Option<&OsStr>) -> io::Result<Coverage> {
    let Some(value) = value else {
        return Ok(Coverage::HardwareV1);
    };
    parse_explicit(value).ok_or_else(|| io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("{SELECTION}={value:?}; expected hardware-v1, diamond-exit-v2, diamond-exit-major-axis-v3 or diamond-exit-fused-projection-v4"),
    ))
}

fn parse_explicit(value: &OsStr) -> Option<Coverage> {
    match value.to_str()? {
        "hardware-v1" => Some(Coverage::HardwareV1),
        "diamond-exit-v2" => Some(Coverage::DiamondExitV2),
        "diamond-exit-major-axis-v3" => Some(Coverage::DiamondExitMajorAxisV3),
        "diamond-exit-fused-projection-v4" => Some(Coverage::DiamondExitFusedProjectionV4),
        _ => None,
    }
}

pub(crate) fn metadata(behavior: Coverage) -> String {
    format!(
        "{{\"thin_line_coverage\":\"{behavior:?}\",\"behavior_version\":{}}}",
        behavior.behavior_version()
    )
}

#[cfg(test)]
#[path = "thin_line_selection_tests.rs"]
mod tests;
