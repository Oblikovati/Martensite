//! Explicit Bezier weight retention, independent of frame and binary format policies.
/// Chooses whether stored nonconstant Bezier weights may be demoted (#464).
/// `BezierSurfaceWeightReadPolicy::PreserveStoredV1` refuses constant non-unit grids.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BezierSurfaceWeightReadPolicy {
    /// Retains epsilon classification and weight normalization for replay.
    #[default]
    LegacyReplay,
    /// Retains nonconstant positive finite grids and all-unit grids exactly.
    PreserveStoredV1,
}
