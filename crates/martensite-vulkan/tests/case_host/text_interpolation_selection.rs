use martensite_vulkan::RenderTextInterpolationBehavior;
use std::{ffi::OsStr, io};

pub(crate) const SELECTION: &str = "MARTENSITE_CASE_TEXT_INTERPOLATION";

pub(super) fn selected(value: Option<&OsStr>) -> io::Result<RenderTextInterpolationBehavior> {
    match value {
        None => Ok(RenderTextInterpolationBehavior::HardwareV1),
        Some(value) if value == "hardware-v1" => Ok(RenderTextInterpolationBehavior::HardwareV1),
        Some(value) if value == "projected-grid12-v2" => {
            Ok(RenderTextInterpolationBehavior::ProjectedGrid12V2)
        }
        Some(value) if value == "projected-grid12-flat-corners-v4" => {
            Ok(RenderTextInterpolationBehavior::ProjectedGrid12FlatCornersV4)
        }
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{SELECTION}={actual:?}; expected unset, hardware-v1, projected-grid12-v2 or projected-grid12-flat-corners-v4"
            ),
        )),
    }
}

pub(super) fn metadata(behavior: RenderTextInterpolationBehavior) -> String {
    format!(
        "{{\"text_interpolation\":\"{behavior:?}\",\"behavior_version\":{},\"scope\":\"plain glyph text\"}}",
        behavior.behavior_version()
    )
}

#[cfg(test)]
support_unit_tests! {
    #[path = "text_interpolation_selection_tests.rs"]
    mod tests;
}
