//! Parse one explicit worker-wide triangle coverage choice without inspecting cases.
use martensite_vulkan::{
    RenderTriangleCoverageBehavior as Behavior, RenderTriangleGridOrigin as Origin,
};
use std::{ffi::OsStr, io};

pub(crate) const SELECTION: &str = "MARTENSITE_CASE_TRIANGLE_COVERAGE";

pub(crate) fn selected(value: Option<&OsStr>) -> io::Result<Behavior> {
    let Some(value) = value else {
        return Ok(Behavior::HardwareV1);
    };
    if value == "hardware-v1" {
        return Ok(Behavior::HardwareV1);
    }
    selected_fixed_grid(value)
        .or_else(|| selected_native_grid(value))
        .ok_or_else(|| invalid_selection(value))
}

fn selected_fixed_grid(value: &OsStr) -> Option<Behavior> {
    match value.to_str()? {
        "fixed-grid-12-v2-top-down" => Some(fixed_grid(Origin::TopDown)),
        "fixed-grid-12-v2-bottom-up" => Some(fixed_grid(Origin::BottomUp)),
        "fixed-grid-12-v3-top-down" => Some(Behavior::FixedGrid12V3 {
            origin: Origin::TopDown,
        }),
        "fixed-grid-12-v3-bottom-up" => Some(Behavior::FixedGrid12V3 {
            origin: Origin::BottomUp,
        }),
        _ => None,
    }
}

fn selected_native_grid(value: &OsStr) -> Option<Behavior> {
    match value.to_str()? {
        "native-grid-12-v4-top-down" => Some(Behavior::NativeGrid12V4 {
            origin: Origin::TopDown,
        }),
        "native-grid-12-v4-bottom-up" => Some(Behavior::NativeGrid12V4 {
            origin: Origin::BottomUp,
        }),
        _ => None,
    }
}

fn invalid_selection(value: &OsStr) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("{SELECTION}={value:?}; expected a supported version and declared origin"),
    )
}

fn fixed_grid(origin: Origin) -> Behavior {
    Behavior::FixedGrid12V2 { origin }
}

pub(super) fn metadata(behavior: Behavior) -> String {
    format!(
        "{{\"triangle_coverage\":\"{behavior:?}\",\"behavior_version\":{}}}",
        behavior.behavior_version()
    )
}

#[cfg(test)]
support_unit_tests! {
    #[path = "triangle_coverage_selection_tests.rs"]
    mod tests;
}
