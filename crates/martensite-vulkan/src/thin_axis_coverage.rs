//! Versioned coverage for the closed single-sample, solid, unlit axis-line operation.
//!
//! Grid origin defines coverage arithmetic independently of attachment row orientation.
//! Finite positive-w nonaxis draws retain hardware coverage. Unclassifiable projections and
//! selected axis draws with unsupported clipping/attributes return typed refusal, never guessed
//! cells. CPU-projected inputs are not evidence of the post-vertex-shader positions of another GPU.
/// Fixed-point precision is explicit rather than inferred from the device vendor.
/// Example: `RenderLineGrid::Fractional12` selects a 1/4096-pixel grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderLineGrid {
    /// Snap to 1/256-pixel coordinates.
    Fractional8,
    /// Snap to 1/4096-pixel coordinates.
    Fractional12,
}
impl RenderLineGrid {
    pub(crate) fn scale(self) -> f64 {
        match self {
            Self::Fractional8 => 256.,
            Self::Fractional12 => 4096.,
        }
    }
}
/// Coordinate direction in which the half-open fixed grid is evaluated.
/// Example: `RenderLineOrigin::BottomUp` evaluates image rows from the bottom.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderLineOrigin {
    /// Evaluate y from the lower logical framebuffer edge.
    BottomUp,
    /// Evaluate y from the upper logical framebuffer edge.
    TopDown,
}
/// Earlier hardware coverage remains selectable for replay.
/// Example: `RenderThinAxisCoverage::FixedGridV2 { grid: RenderLineGrid::Fractional12, origin: RenderLineOrigin::BottomUp }`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderThinAxisCoverage {
    /// Preserve the existing device line rasterizer.
    #[default]
    HardwareV1,
    /// Encode covered pixel cells for solid, single-sample unlit axis lines.
    FixedGridV2 {
        /// The explicit fractional coordinate grid.
        grid: RenderLineGrid,
        /// The explicit direction of the half-open coordinate grid.
        origin: RenderLineOrigin,
    },
}
const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<RenderThinAxisCoverage>();
};
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_replays_hardware_and_precision_is_explicit() {
        assert_eq!(
            RenderThinAxisCoverage::default(),
            RenderThinAxisCoverage::HardwareV1
        );
        assert_eq!(RenderLineGrid::Fractional8.scale(), 256.);
        assert_eq!(RenderLineGrid::Fractional12.scale(), 4096.);
    }
}
