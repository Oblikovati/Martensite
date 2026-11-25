// SPDX-License-Identifier: GPL-2.0-only
//! Admit one contractible aperture in the authoring chart before choosing the native cut.
use super::{
    super::{
        admission as cylinder_admission, cylinder_sweep, encoding, failure::*, history::*, semantic,
    },
    admission as band_admission, verification, *,
};
use martensite_tkbrep::topods;
use std::f64::consts::{PI, TAU};

mod crossed;
mod curved;
mod images;
pub use crossed::*;
pub use curved::{CurvedBandResult, normal_extrude_cylinder_band_with_curved_hole};

/// One authored rectangular through hole on a cylinder, expressed in the band's chart.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CylinderBandHole {
    /// Increasing arc lengths, in model units. They may straddle the authoring seam.
    pub arc_length: [f64; 2],
    /// Increasing axial bounds strictly inside the band's rims.
    pub axial: [f64; 2],
    /// Authored corner IDs, starting at the lower arc length and lower axial coordinate.
    pub corners: [SurfaceVertexId; 4],
    /// Authored edges between successive corners, including closure.
    pub boundaries: [SurfaceCurveId; 4],
}

/// One authored rectangular aperture in a developed conical band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConeBandHole {
    /// Increasing developed angles in radians, relative to the band's authored seam.
    pub developed_angle: [f64; 2],
    /// Increasing positive slant distances strictly inside the band.
    pub slant: [f64; 2],
    /// Authored corners in order `(angle0, l0), (angle1, l0), (angle1, l1), (angle0, l1)`.
    pub corners: [SurfaceVertexId; 4],
    /// Authored edges between successive corners, including closure.
    pub boundaries: [SurfaceCurveId; 4],
}

/// Extrude a complete cylindrical band with one through hole along the normal.
///
/// Keeps the aperture's authored contours in their original lift. The physical
/// body and authored identities are independent of the native construction cut.
/// Example: `let solid = normal_extrude_cylinder_band_with_hole(&band, &hole, [0.0, 1.0], options)?;`.
pub fn normal_extrude_cylinder_band_with_hole(
    band: &FullTurnBand,
    hole: &CylinderBandHole,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    build(band, hole, depths, options, None)
}

pub(super) fn build(
    band: &FullTurnBand,
    hole: &CylinderBandHole,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
    seam: Option<f64>,
) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
    let prepared = Cutout::admit(band, hole, depths, options, |aperture| {
        interior_cut(aperture, seam)
    })
    .map_err(|cause| SurfaceSketchFailure {
        region: band.region,
        stage: SurfaceStage::Admission,
        cause,
    })?;
    prepared.publish(band.rims)
}

pub(super) struct Cutout {
    pub(super) stock: band_admission::BandSection,
    pub(super) aperture: cylinder_admission::Admitted,
}

impl Cutout {
    fn admit(
        band: &FullTurnBand,
        hole: &CylinderBandHole,
        depths: [f64; 2],
        options: NormalExtrusionOptions,
        choose_seam: impl FnOnce(&cylinder_admission::Admitted) -> Checked<f64>,
    ) -> Checked<Self> {
        let (support, axial) = cylinder(band.support)?;
        let aperture = admit_aperture(band, hole, depths, options)?;
        let chart_origin = choose_seam(&aperture)?;
        let support = CylinderSupport {
            chart_origin,
            ..support
        };
        let native = FullTurnBand {
            support: FullTurnSupport::Cylinder { support, axial },
            ..*band
        };
        let stock = band_admission::admit(native, depths, options)?;
        Ok(Self { stock, aperture })
    }

    fn publish(
        mut self,
        rims: [SurfaceCurveId; 2],
    ) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
        let (proposal, stock, hole) = self.proposal(rims)?;
        let candidate = images::bind(proposal, stock, hole)
            .map_err(|cause| self.stock.sweep.failure(SurfaceStage::History, cause))?;
        let keys: Vec<_> = candidate.1.iter().map(|(key, _)| *key).collect();
        self.subtract_analytic_measure();
        let bytes = encoding::encode(&candidate.0, &candidate.1)
            .map_err(|cause| self.stock.sweep.failure(SurfaceStage::Encoding, cause))?;
        self.validate_archive(rims, bytes, keys)
    }

    fn subtract_analytic_measure(&mut self) {
        account_aperture(&mut self.stock.sweep, &self.aperture);
    }

    fn proposal(
        &self,
        rims: [SurfaceCurveId; 2],
    ) -> Result<(images::Candidate, Bindings, Bindings), SurfaceSketchFailure> {
        let sweep = &self.stock.sweep;
        let stock = export_stock(&self.stock, rims)?;
        let aperture = export_aperture(&self.aperture)?;
        let proposal = images::subtract(&stock.solid, &aperture.solid)
            .map_err(|cause| sweep.failure(SurfaceStage::Construction, cause))?;
        Ok((proposal, stock.entities, aperture.entities))
    }

    fn validate_archive(
        &self,
        rims: [SurfaceCurveId; 2],
        bytes: Vec<u8>,
        keys: Vec<HistoryKey>,
    ) -> Result<NormalExtrusionResult, SurfaceSketchFailure> {
        let sweep = &self.stock.sweep;
        let (body, bindings) = encoding::decode(&bytes, &keys)
            .map_err(|cause| sweep.failure(SurfaceStage::Encoding, cause))?;
        let check = |bindings: &Bindings| {
            verification::validate_aperture(&self.stock, rims, &self.aperture, bindings)
        };
        let checked = validation::check_one_periodic_hole(&body, &bindings, sweep, &check)
            .map_err(|cause| sweep.failure(SurfaceStage::Validation, cause))?;
        let (volume, errors) = volume::measure(&body, sweep)
            .map_err(|cause| sweep.failure(SurfaceStage::Volume, cause))?;
        Ok(result::assemble(
            sweep, bytes, keys, checked, volume, errors,
        ))
    }
}

fn admit_aperture(
    band: &FullTurnBand,
    hole: &CylinderBandHole,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<cylinder_admission::Admitted> {
    let (support, axial) = cylinder(band.support)?;
    let aperture = cylinder_admission::admit(&hole.source(band.region, support), depths, options)?;
    require_rim_clearance(axial, hole.axial, options)?;
    require_distinct_sources(band.rims, hole.boundaries)?;
    Ok(aperture)
}

fn export_stock(
    stock: &band_admission::BandSection,
    rims: [SurfaceCurveId; 2],
) -> Result<DetachedNormalBody, SurfaceSketchFailure> {
    pipeline::build(&stock.sweep, &|bindings| {
        verification::validate(stock, rims, bindings)
    })?
    .export_brep()
}

fn export_aperture(
    aperture: &cylinder_admission::Admitted,
) -> Result<DetachedNormalBody, SurfaceSketchFailure> {
    let sweep = cylinder_sweep::prepare(aperture);
    pipeline::build(&sweep, &|bindings| semantic::validate(aperture, bindings))?.export_brep()
}

fn account_aperture(sweep: &mut sweep::Sweep, aperture: &cylinder_admission::Admitted) {
    account_operand(sweep, &cylinder_sweep::prepare(aperture));
}

fn account_operand(sweep: &mut sweep::Sweep, cutter: &sweep::Sweep) {
    sweep.expected_volume -= cutter.expected_volume;
    sweep.surface_area += cutter.surface_area;
    sweep.coordinate_error = sweep.coordinate_error.max(cutter.coordinate_error);
}

impl CylinderBandHole {
    fn source(self, region: SurfaceRegionId, support: CylinderSupport) -> CylinderRectangle {
        CylinderRectangle {
            support,
            region,
            s: self.arc_length,
            z: self.axial,
            corners: self.corners,
            boundaries: self.boundaries,
        }
    }
}

fn cylinder(support: FullTurnSupport) -> Checked<(CylinderSupport, [f64; 2])> {
    match support {
        FullTurnSupport::Cylinder { support, axial } => Ok((support, axial)),
        FullTurnSupport::Cone { support, .. } => Err(SurfaceFailureCause::InvalidInput {
            field: "cone semi-angle",
            value: support.semi_angle,
            expected: "cylindrical support for one rectangular aperture",
        }),
    }
}

fn require_periodic_chord(angle: f64, radius: f64, floor: f64) -> Checked<()> {
    // For the nearest cut the physical angle is at most pi. Concavity
    // gives 2r sin(angle/2) >= (2/pi) r angle on this interval.
    let chord_factor = (2. / PI.next_up()).next_down();
    let chord = ((angle.min(PI) * chord_factor).next_down() * radius).next_down();
    cylinder_admission::require(
        chord > floor,
        "aperture seam fragment clearance",
        chord,
        "certified physical chord greater than effective spatial separation",
    )
}

fn require_rim_clearance(
    band: [f64; 2],
    hole: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<()> {
    let minimum = effective_limits(options).linear_separation / 2.;
    cylinder_admission::separated("lower aperture clearance", hole[0] - band[0], minimum)?;
    cylinder_admission::separated("upper aperture clearance", band[1] - hole[1], minimum)
}

fn require_distinct_sources(rims: [SurfaceCurveId; 2], edges: [SurfaceCurveId; 4]) -> Checked<()> {
    if let Some(rim) = rims.into_iter().find(|rim| edges.contains(rim)) {
        return Err(SurfaceFailureCause::DuplicateIdentity {
            kind: "rim and aperture edge",
            value: rim.0,
        });
    }
    Ok(())
}

fn interior_cut(aperture: &cylinder_admission::Admitted, seam: Option<f64>) -> Checked<f64> {
    let midpoint = aperture.start + aperture.angle / 2.;
    let seam = seam.unwrap_or(midpoint + PI);
    let relative = (midpoint - seam).rem_euclid(TAU);
    let separation = relative.min(TAU - relative);
    let rounding = 64. * f64::EPSILON * (midpoint.abs() + seam.abs() + TAU);
    let reserve = effective_limits(aperture.options).angular_clearance + rounding;
    let margin = separation - aperture.angle / 2.;
    cylinder_admission::require(
        margin.is_finite() && margin > reserve,
        "aperture to native seam clearance",
        margin,
        "strictly clear of the admitted aperture and angular reserve",
    )?;
    Ok(seam)
}
