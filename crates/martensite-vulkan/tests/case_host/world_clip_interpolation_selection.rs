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
mod tests {
    use super::*;

    #[test]
    fn absent_and_explicit_values_keep_versioned_replay() {
        assert_eq!(selected(None).unwrap(), Behavior::HardwareV1);
        assert_eq!(
            selected(Some(OsStr::new("hardware-v1"))).unwrap(),
            Behavior::HardwareV1
        );
        let v1 = selected(Some(OsStr::new("projected-grid12-v1"))).unwrap();
        assert_eq!(v1, Behavior::ProjectedGrid12V1);
        assert!(metadata(v1).ends_with("\"behavior_version\":2}"));
        let v2 = selected(Some(OsStr::new("projected-grid12-v2"))).unwrap();
        assert_eq!(v2, Behavior::ProjectedGrid12V2);
        assert!(metadata(v2).ends_with("\"behavior_version\":3}"));
    }

    #[test]
    fn unknown_values_are_rejected_with_the_actual_selector() {
        for value in ["", "true", "projected-grid12-v3", "bug26973"] {
            let error = selected(Some(OsStr::new(value))).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
            assert!(error.to_string().contains(value));
        }
    }
}
