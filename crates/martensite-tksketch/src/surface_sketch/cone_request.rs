//! Owned conical support and its annular sector in the developed plane (#417).
use super::request::{NormalSense, SurfaceCurveId, SurfaceRegionId, SurfaceVertexId};

/// Right-handed cone chart with a positive half-angle strictly below pi/2.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConeSupport {
    /// Original support apex, in model units.
    pub apex: [f64; 3],
    /// Unit axis pointing into the selected half-cone.
    pub axis: [f64; 3],
    /// Unit radial direction perpendicular to the axis at physical angle zero.
    pub radial: [f64; 3],
    /// Cone semi-angle in radians.
    pub semi_angle: f64,
    /// Unwrapped physical angular origin, in radians.
    pub chart_origin: f64,
    /// Normal used to interpret signed physical depth.
    pub normal: NormalSense,
}
/// Annular sector in developed slant-distance and angular coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConeSector {
    /// Owned analytic support.
    pub support: ConeSupport,
    /// Identity retained by the solid and depth faces.
    pub region: SurfaceRegionId,
    /// Increasing positive slant distances from the original apex, in model units.
    pub slant: [f64; 2],
    /// Increasing developed angles in radians; physical span divides by sin(semi-angle).
    pub developed_angle: [f64; 2],
    /// Corners `(phi0,l0), (phi1,l0), (phi1,l1), (phi0,l1)`.
    pub corners: [SurfaceVertexId; 4],
    /// Boundary from each corner to the next, wrapping at the last corner.
    pub boundaries: [SurfaceCurveId; 4],
}
