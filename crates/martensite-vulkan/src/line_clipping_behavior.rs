//! Immutable replay versions separate driver-derived clipping from presentation aspects.
/// Choose line clipping without changing vertices, colours or line widths.
/// Example: `RenderLineClippingBehavior::VulkanDriverV1.behavior_version()`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderLineClippingBehavior {
    /// Replay the Vulkan implementation's fixed clip-volume/guard-band decisions.
    #[default]
    VulkanDriverV1,
    /// Enforce viewport clipping where the selected software driver requires it.
    PlatformNativeV2,
    /// As version two, and that driver stage clips the source segments: native GL derives the
    /// clipped ends from source clip coordinates, so the CPU guard band's object-space rounding
    /// (`line_guard_band`) would change the slope of every far line it touches.
    PlatformNativeV3,
}
impl RenderLineClippingBehavior {
    /// Record the selected replay contract; `behavior.behavior_version()`.
    pub const fn behavior_version(self) -> u32 {
        match self {
            Self::VulkanDriverV1 => 1,
            Self::PlatformNativeV2 => 2,
            Self::PlatformNativeV3 => 3,
        }
    }
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<RenderLineClippingBehavior>();
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replay_versions_remain_distinct_and_explicit() {
        assert_eq!(
            RenderLineClippingBehavior::VulkanDriverV1.behavior_version(),
            1
        );
        assert_eq!(
            RenderLineClippingBehavior::PlatformNativeV2.behavior_version(),
            2
        );
        assert_eq!(
            RenderLineClippingBehavior::PlatformNativeV3.behavior_version(),
            3
        );
        assert_eq!(
            RenderLineClippingBehavior::default(),
            RenderLineClippingBehavior::VulkanDriverV1
        );
    }
}
