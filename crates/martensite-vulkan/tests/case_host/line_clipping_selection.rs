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
support_unit_tests! {
    #[path = "line_clipping_selection_tests.rs"]
    mod tests;
}
