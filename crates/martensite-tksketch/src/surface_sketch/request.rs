//! Owned analytic inputs and caller identities for cylinder-normal extrusion (#416).
use martensite_tkbo::profile_extrusion::Resolution;

/// Caller identity of a developed region; `SurfaceRegionId(1)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceRegionId(pub u64);
/// Caller identity of a boundary curve; `SurfaceCurveId(2)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCurveId(pub u64);
/// Caller identity of a corner; `SurfaceVertexId(3)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceVertexId(pub u64);

/// Chosen support normal, independent of solid boundary orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NormalSense {
    /// Increasing physical radius.
    Outward,
    /// Decreasing physical radius without reversing the developed chart.
    Inward,
}

/// Owned right-handed cylinder chart; axis and radial vectors must be orthonormal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylinderSupport {
    /// Axis origin, in model units.
    pub origin: [f64; 3],
    /// Unit axis direction.
    pub axis: [f64; 3],
    /// Unit radial direction at chart angle zero, perpendicular to the axis.
    pub radial: [f64; 3],
    /// Positive support radius.
    pub radius: f64,
    /// Unwrapped angular origin, in radians.
    pub chart_origin: f64,
    /// Normal used to interpret signed physical depth.
    pub normal: NormalSense,
}

/// Rectangle in developed arc-length and axial coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylinderRectangle {
    /// Owned analytic support.
    pub support: CylinderSupport,
    /// Identity retained by the solid and its depth faces.
    pub region: SurfaceRegionId,
    /// Increasing developed arc-length endpoints.
    pub s: [f64; 2],
    /// Increasing axial endpoints.
    pub z: [f64; 2],
    /// Corners `(s0,z0), (s1,z0), (s1,z1), (s0,z1)`.
    pub corners: [SurfaceVertexId; 4],
    /// Boundary from each corner to the next, wrapping at the last corner.
    pub boundaries: [SurfaceCurveId; 4],
}

/// Requested construction tolerances and independent volume convergence.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalExtrusionOptions {
    /// Spatial/angular/display tolerances and explicit achieved-tolerance cap.
    pub resolution: Resolution,
    /// Positive relative Gauss-Kronrod convergence, between 64 epsilon and 0.001.
    pub volume_relative_error: f64,
}

/// Effective admission limits, including the native builder's precision floors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalExtrusionLimits {
    /// Dimensions, axis clearance and end-face separation must exceed this distance.
    pub linear_separation: f64,
    /// Positive span and full-turn clearance must exceed this angle.
    pub angular_clearance: f64,
}

pub(super) fn effective_limits(options: NormalExtrusionOptions) -> NormalExtrusionLimits {
    NormalExtrusionLimits {
        linear_separation: 2.
            * options
                .resolution
                .weld
                .max(martensite_tkernel::precision::confusion()),
        angular_clearance: options
            .resolution
            .angular
            .max(martensite_tkernel::precision::angular()),
    }
}
