// SPDX-License-Identifier: GPL-2.0-only
//! Adapt verified sector operands to the common periodic-image construction.
use super::*;
use crate::surface_sketch::{cone_admission, cone_faces, cone_math, cone_semantic};

#[derive(Clone, Copy)]
pub(in crate::surface_sketch::full_turn) struct Contour {
    pub lift: [f64; 2],
    pub meridian: [f64; 2],
    pub corners: [SurfaceVertexId; 4],
    pub boundaries: [SurfaceCurveId; 4],
}

impl From<CylinderBandHole> for Contour {
    fn from(hole: CylinderBandHole) -> Self {
        Self {
            lift: hole.arc_length,
            meridian: hole.axial,
            corners: hole.corners,
            boundaries: hole.boundaries,
        }
    }
}

impl From<ConeBandHole> for Contour {
    fn from(hole: ConeBandHole) -> Self {
        Self {
            lift: hole.developed_angle,
            meridian: hole.slant,
            corners: hole.corners,
            boundaries: hole.boundaries,
        }
    }
}

pub(in crate::surface_sketch::full_turn) enum ApertureOperand {
    Cylinder(Box<cylinder_admission::Admitted>),
    Cone(Box<cone_admission::AdmittedCone>),
}

impl ApertureOperand {
    pub(super) fn admit(
        band: &FullTurnBand,
        contour: Contour,
        depths: [f64; 2],
        options: NormalExtrusionOptions,
    ) -> Checked<Self> {
        match band.support {
            FullTurnSupport::Cylinder { .. } => {
                let hole = CylinderBandHole {
                    arc_length: contour.lift,
                    axial: contour.meridian,
                    corners: contour.corners,
                    boundaries: contour.boundaries,
                };
                Ok(Self::Cylinder(Box::new(admit_aperture(
                    band, &hole, depths, options,
                )?)))
            }
            FullTurnSupport::Cone { support, slant } => {
                require_rim_clearance(slant, contour.meridian, options)?;
                require_distinct_sources(band.rims, contour.boundaries)?;
                let sector = contour.sector(support, band.region);
                Ok(Self::Cone(Box::new(cone_admission::admit(
                    &sector, depths, options,
                )?)))
            }
        }
    }

    pub(super) fn export(&self) -> Result<DetachedNormalBody, SurfaceSketchFailure> {
        match self {
            Self::Cylinder(a) => export_aperture(a),
            Self::Cone(a) => {
                pipeline::build(&a.sweep, &|bindings| cone_semantic::validate(a, bindings))?
                    .export_brep()
            }
        }
    }

    pub(super) fn account(&self, sweep: &mut sweep::Sweep) {
        match self {
            Self::Cylinder(a) => account_aperture(sweep, a),
            Self::Cone(a) => account_operand(sweep, &a.sweep),
        }
    }

    pub(in crate::surface_sketch::full_turn) fn meridian(
        &self,
        depth: PeriodicDepth,
        rim: usize,
    ) -> [f64; 2] {
        let depth = depth.index();
        match self {
            Self::Cylinder(a) => [a.radii[depth], a.rectangle.z[rim]],
            Self::Cone(a) => cone_math::meridian(a, a.sector.slant[rim], a.depths[depth]),
        }
    }

    pub(in crate::surface_sketch::full_turn) fn check_boundary(
        &self,
        bindings: &Bindings,
    ) -> Checked<()> {
        match self {
            Self::Cylinder(a) => semantic::validate(a, bindings),
            Self::Cone(a) => cone_semantic::validate(a, bindings),
        }
    }

    pub(in crate::surface_sketch::full_turn) fn check_wall(
        &self,
        key: HistoryKey,
        face: &topods::Shape,
    ) -> Checked<()> {
        match self {
            Self::Cylinder(a) => semantic::check_face_carrier(a, key, face),
            Self::Cone(a) => cone_faces::check_carrier(a, key, face),
        }
    }
}

impl Contour {
    fn sector(self, support: ConeSupport, region: SurfaceRegionId) -> ConeSector {
        ConeSector {
            support,
            region,
            developed_angle: self.lift,
            slant: self.meridian,
            corners: self.corners,
            boundaries: self.boundaries,
        }
    }
}
