//! Explicit interpretation of stored analytic surface frames (#432).
/// Selects analytic surface frame reconstruction, independently of binary format version.
/// `SurfaceFrameReadPolicy::PreserveStoredV1` retains validated origin/main/X/Y bits.
/// Curves and standalone directions retain legacy reconstruction under both policies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SurfaceFrameReadPolicy {
    /// Reconstructs frames with normalization and orthogonalization for replay.
    #[default]
    LegacyReplay,
    /// Checks the stored frame against outward reconstruction enclosures and retains it.
    PreserveStoredV1,
}
