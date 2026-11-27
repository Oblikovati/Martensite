//! Explicit attachment conventions preserve the logical top-left capture contract.
/// Select attachment mapping independently from provoking-vertex behavior.
/// Both versions expose logical top-left captures; attachment addressing is separate from visual Y.
/// Example: `RenderCoordinateConvention::TopLeftLogicalV2.behavior_version()`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderCoordinateConvention {
    /// Positive-height rasterization with output row reversal preserves earlier replay.
    /// Select it explicitly to reproduce images recorded before behavior version 2.
    LegacyBottomUpV1,
    /// Negative-height rasterization uses logical visual Y and physical attachment addressing.
    /// Default since 2026-10-27 (decision D1): it matches the native strict cap assets
    /// that the legacy mapping fails on both software and AMD drivers.
    #[default]
    TopLeftLogicalV2,
}

impl RenderCoordinateConvention {
    /// Identify the coordinate contract; both versions expose top-left captures.
    /// Example: `RenderCoordinateConvention::default().behavior_version()` returns two.
    pub fn behavior_version(self) -> u32 {
        match self {
            Self::LegacyBottomUpV1 => 1,
            Self::TopLeftLogicalV2 => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RenderCoordinateConvention;

    #[test]
    fn coordinate_versions_default_to_top_left_and_keep_legacy_selectable() {
        let selected = RenderCoordinateConvention::default();
        assert_eq!(selected, RenderCoordinateConvention::TopLeftLogicalV2);
        assert_eq!(selected.behavior_version(), 2);
        assert_eq!(
            RenderCoordinateConvention::LegacyBottomUpV1.behavior_version(),
            1
        );
        let _: &(dyn Send + Sync) = &selected;
    }
}
