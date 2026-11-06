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
mod tests {
    use super::*;
    #[test]
    fn explicit_selection_preserves_strict_default_and_metadata() {
        for v in [None, Some(OsStr::new("hardware-format-v1"))] {
            assert_eq!(selected(v).unwrap(), Behavior::HardwareFormatV1);
            assert!(require_capture_default(v).is_ok());
        }
        let v = Some(OsStr::new("native24-units-v2"));
        assert_eq!(selected(v).unwrap(), Behavior::Native24UnitsV2);
        assert!(require_capture_default(v).is_err());
        assert!(metadata(selected(v).unwrap()).contains("\"behavior_version\":2"));
    }
    #[test]
    fn malformed_selections_never_fall_back() {
        for v in ["", "true", "identic", "native24-units-v1"] {
            let e = selected(Some(OsStr::new(v))).unwrap_err();
            assert_eq!(e.kind(), io::ErrorKind::InvalidInput);
            assert!(e.to_string().contains(v));
        }
    }
}
