// SPDX-License-Identifier: GPL-2.0-only
//! Full-turn normal extrusion uses a closed meridian revolution and authored rim identities.
use super::*;

mod admission;
mod history;
mod hole;
mod verification;

pub(super) use history::collect;
pub use hole::*;

/// Support and meridian bounds of a full-turn band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FullTurnSupport {
    /// A cylinder with constant lower and upper axial coordinates.
    Cylinder {
        /// Cylinder and seam frame.
        support: CylinderSupport,
        /// Strictly increasing axial bounds in model units.
        axial: [f64; 2],
    },
    /// A cone with constant lower and upper slant distances.
    Cone {
        /// Cone and seam frame.
        support: ConeSupport,
        /// Strictly increasing positive slant distances from the apex.
        slant: [f64; 2],
    },
}

/// An explicitly periodic region bounded by two closed rims and no holes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FullTurnBand {
    /// Support and meridional extent.
    pub support: FullTurnSupport,
    /// Authored region identity.
    pub region: SurfaceRegionId,
    /// Authored lower and upper rim identities in meridian order.
    pub rims: [SurfaceCurveId; 2],
}

/// Extrude one full-turn band through two increasing signed normal depths.
///
/// The seam is generated from the support frame. Moving it preserves the region
/// and rim history keys. The seam contributes edge and vertex entities only.
/// A region depth-edge key names the seam of that depth face. Rim depth-vertex
/// keys name circle anchors. On a conical rim side face, the rim's normal-edge
/// key names its seam. The two directed uses are retained in the native face.
/// Example: `let result = normal_extrude_full_turn_band(&band, [-1.0, 2.0], options)?;`.
pub fn normal_extrude_full_turn_band(
    band: &FullTurnBand,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let section =
        admission::admit(*band, depths, options).map_err(|cause| SurfaceSketchFailure {
            region: band.region,
            stage: SurfaceStage::Admission,
            cause,
        })?;
    pipeline::build(&section.sweep, &|bindings| {
        verification::validate(&section, band.rims, bindings)
    })
}

#[cfg(test)]
mod tests;
