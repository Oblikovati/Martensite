//! Owned solved-profile selection and explicit error policies for normal extrusion.
use crate::surface_sketch::{CylinderSupport, SurfaceRegionId};
use martensite_tkbo::profile_extrusion::{ProfileLoop, SketchProfile};
pub use martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit as WrappedVolumePolicy;

/// One explicitly selected material region; holes remain unfilled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedRegionSelection {
    /// Caller-owned region identity, independent of planar capture order.
    pub region: SurfaceRegionId,
    /// Directed outer boundary from the solved profile.
    pub outer: ProfileLoop,
    /// Directed inner boundaries from the solved profile.
    pub holes: Vec<ProfileLoop>,
}

/// Requested spatial accuracy, geometric subdivision limit and volume policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedNormalOptions {
    /// Positive finite world-space modelling epsilon; placement reports its canonical allocation.
    pub spatial_error: f64,
    /// Finite planar proof work, independent of spatial accuracy.
    pub planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    /// Maximum geometric dyadic fit level; current admission supports `0..=8`.
    pub maximum_fit_level: u8,
    /// Explicit report or refusal policy for retained-flux versus ideal normal volume.
    pub volume: WrappedVolumePolicy,
}

/// Owned input for an extrusion of selected developed profiles along cylinder normals.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedNormalRequest {
    /// Solved profile in developed world-XY coordinates `[arc_length, height]`.
    pub profile: SketchProfile,
    /// Explicit material choices; bounded cells are never selected implicitly.
    pub selections: Vec<WrappedRegionSelection>,
    /// Analytic support; axis/radial vectors define a normalized, projected right-handed frame.
    pub support: CylinderSupport,
    /// Increasing physical depths along the chosen support normal.
    pub depths: [f64; 2],
    /// Explicit accuracy and measure policies.
    pub options: WrappedNormalOptions,
}
