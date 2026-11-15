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
