//! Versioned coverage for single-sample, one-pixel independent lines.
/// Select line coverage independently from thin-axis and point coverage.
/// Example: `RenderThinLineCoverage::DiamondExitV2.behavior_version()` returns two.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RenderThinLineCoverage {
    /// Preserve the implementation-selected line rasterizer for replay.
    #[default]
    HardwareV1,
    /// Use ideal open-diamond exit coverage and screen-axis stipple.
    /// Projected endpoints use a 16-bit fractional pixel grid with ties-to-even rounding.
    /// That quantization moves each projected endpoint by at most 0.5/65536 pixel.
    /// Fragment attributes use the OpenGL projected-center parameter.
    /// Interpolated NDC depth is clamped to [-1, 1] before private-quad emission.
    DiamondExitV2,
    /// Use dominant-axis fragment interpolation with open-diamond coverage.
    /// Color and world attributes use perspective correction; depth remains linear in the same
    /// unclamped parameter, then NDC depth is clamped to [-1, 1] before private-quad emission.
    /// Example: select with `RenderThinLineCoverage::DiamondExitMajorAxisV3`.
    DiamondExitMajorAxisV3,
    /// V3 coverage and attributes with explicit fused clip projection in every XYZW component.
    /// Also accepts solid, opaque one-pixel strips, preserving ordered half-open segments.
    /// Example: select `RenderThinLineCoverage::DiamondExitFusedProjectionV4`.
    DiamondExitFusedProjectionV4,
}

impl RenderThinLineCoverage {
    /// Identify the explicit coverage contract.
    /// Example: `RenderThinLineCoverage::default().behavior_version()` returns one.
    pub fn behavior_version(self) -> u32 {
        match self {
            Self::HardwareV1 => 1,
            Self::DiamondExitV2 => 2,
            Self::DiamondExitMajorAxisV3 => 3,
            Self::DiamondExitFusedProjectionV4 => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RenderThinLineCoverage;

    #[test]
    fn hardware_replay_remains_the_default() {
        assert_eq!(
            RenderThinLineCoverage::default(),
            RenderThinLineCoverage::HardwareV1
        );
        assert_eq!(RenderThinLineCoverage::DiamondExitV2.behavior_version(), 2);
        assert_eq!(
            RenderThinLineCoverage::DiamondExitMajorAxisV3.behavior_version(),
            3
        );
        let _: &(dyn Send + Sync) = &RenderThinLineCoverage::DiamondExitV2;
    }
}
