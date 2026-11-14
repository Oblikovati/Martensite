//! Explicit conic frame decoding, independent of format version (#455).
/// Selects conic frame decoding; `CurveFrameReadPolicy::PreserveStoredV1`.
/// Includes checked line and offset reference directions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CurveFrameReadPolicy {
    /// Reconstructs the frame and recomputes its Y direction.
    #[default]
    LegacyReplay,
    /// Retains checked direct origin/main/X/Y coefficients and unit directions.
    PreserveStoredV1,
}
