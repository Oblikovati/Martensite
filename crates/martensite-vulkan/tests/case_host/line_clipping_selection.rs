//! One explicit worker-wide version selection leaves original case bodies unchanged.
use martensite_vulkan::RenderLineClippingBehavior;
use std::{ffi::OsStr, io};
pub(super) const LINE_CLIPPING_SELECTION: &str = "MARTENSITE_CASE_LINE_CLIPPING_BEHAVIOR";

pub(super) fn selected_behavior(value: Option<&OsStr>) -> io::Result<RenderLineClippingBehavior> {
    match value {
        None => Ok(RenderLineClippingBehavior::VulkanDriverV1),
        Some(value) if value == "vulkan-driver-v1" => {
            Ok(RenderLineClippingBehavior::VulkanDriverV1)
        }
        Some(value) if value == "platform-native-v2" => {
            Ok(RenderLineClippingBehavior::PlatformNativeV2)
        }
        Some(value) if value == "platform-native-v3" => {
            Ok(RenderLineClippingBehavior::PlatformNativeV3)
        }
        actual => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{LINE_CLIPPING_SELECTION}={actual:?}; expected unset, vulkan-driver-v1, platform-native-v2 or platform-native-v3"
            ),
        )),
    }
}
pub(super) fn selection_metadata(behavior: RenderLineClippingBehavior) -> String {
    format!(
        "{{\"line_clipping_behavior\":\"{behavior:?}\",\"behavior_version\":{}}}",
        behavior.behavior_version()
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_and_explicit_legacy_selections_preserve_version_one() {
        for value in [None, Some(OsStr::new("vulkan-driver-v1"))] {
            assert_eq!(
                selected_behavior(value).unwrap(),
                RenderLineClippingBehavior::VulkanDriverV1
            );
        }
    }
    #[test]
    fn explicit_candidate_and_metadata_record_version_two() {
        let selected = selected_behavior(Some(OsStr::new("platform-native-v2"))).unwrap();
        assert_eq!(selected, RenderLineClippingBehavior::PlatformNativeV2);
        assert_eq!(
            selection_metadata(selected),
            "{\"line_clipping_behavior\":\"PlatformNativeV2\",\"behavior_version\":2}"
        );
    }
    #[test]
    fn explicit_source_clipping_candidate_records_version_three() {
        let selected = selected_behavior(Some(OsStr::new("platform-native-v3"))).unwrap();
        assert_eq!(selected, RenderLineClippingBehavior::PlatformNativeV3);
        assert_eq!(
            selection_metadata(selected),
            "{\"line_clipping_behavior\":\"PlatformNativeV3\",\"behavior_version\":3}"
        );
    }
    #[test]
    fn malformed_selections_are_refused_before_driver_creation() {
        for value in ["", "true", "platform-native-v9", "bug31425"] {
            let failure = selected_behavior(Some(OsStr::new(value))).unwrap_err();
            assert_eq!(failure.kind(), io::ErrorKind::InvalidInput);
            assert!(failure.to_string().contains(value));
        }
    }
}
