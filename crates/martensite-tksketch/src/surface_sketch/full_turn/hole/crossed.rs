// SPDX-License-Identifier: GPL-2.0-only
//! Publish the aperture when the actual native cut passes through its material boundary.
use super::*;

mod binding;
mod names;
mod placement;
mod plan;
mod tool;
pub use names::*;
pub(in crate::surface_sketch::full_turn) use plan::{ApprovedHole, CrossedPlan};
pub(in crate::surface_sketch::full_turn) use tool::Contour;

/// Published periodic band and its independently named periodic edge fragments.
#[derive(Clone, Debug)]
pub struct CrossedBandResult {
    geometry: NormalExtrusionResult<PeriodicImageKey>,
    crossings: Vec<PeriodicCrossing>,
}

impl CrossedBandResult {
    /// Immutable geometry, volume, validation and named native export.
    /// Example: `result.geometry().export_brep()?`.
    pub fn geometry(&self) -> &NormalExtrusionResult<PeriodicImageKey> {
        &self.geometry
    }

    /// Lower and upper crossings per crossed aperture, ordered by authored contour IDs.
    /// Example: `result.crossings()[0].normalized_parameter`.
    pub fn crossings(&self) -> &[PeriodicCrossing] {
        &self.crossings
    }
}

/// Extrude one cylindrical band with a rectangular aperture across its actual native seam.
///
/// The aperture must cross one lift of the authored meridian transversely.
/// The native seam is kept at the authored datum. All split pieces receive
/// stable source intervals and the generated vertices retain their signed
/// crossing identities.
/// Example: `let result = normal_extrude_cylinder_band_at_authored_seam(&band, &hole, [-1.0, 2.0], options)?;`.
pub fn normal_extrude_cylinder_band_at_authored_seam(
    band: &FullTurnBand,
    hole: &CylinderBandHole,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<CrossedBandResult, SurfaceSketchFailure> {
    cylinder(band.support).map_err(|cause| failure_in_admission(band, cause))?;
    extrude(
        band,
        vec![Contour::from(*hole)],
        depths,
        options,
        SeamScope::CrossingsOnly,
    )
}

/// Extrude a cylindrical band with axially separated rectangular through apertures.
///
/// Contours can be clear of the authored meridian or cross it transversely.
/// Apertures may cover the entire angular period in projection
/// because separation is proved on the axial coordinate.
/// Example: `let result = normal_extrude_cylinder_band_holes_at_authored_seam(&band, &holes, [-1.0, 2.0], options)?;`.
pub fn normal_extrude_cylinder_band_holes_at_authored_seam(
    band: &FullTurnBand,
    holes: &[CylinderBandHole],
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<CrossedBandResult, SurfaceSketchFailure> {
    cylinder(band.support).map_err(|cause| failure_in_admission(band, cause))?;
    extrude(
        band,
        holes.iter().copied().map(Contour::from).collect(),
        depths,
        options,
        SeamScope::AllowClear,
    )
}

/// Extrude a full conical band with one aperture crossing the authored seam.
/// The aperture must cross one lift of the authored meridian transversely.
/// Example: `let result = normal_extrude_cone_band_at_authored_seam(&band, &hole, [-0.25, 0.5], options)?;`.
pub fn normal_extrude_cone_band_at_authored_seam(
    band: &FullTurnBand,
    hole: &ConeBandHole,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<CrossedBandResult, SurfaceSketchFailure> {
    require_cone(band)?;
    extrude(
        band,
        vec![Contour::from(*hole)],
        depths,
        options,
        SeamScope::CrossingsOnly,
    )
}

/// Extrude a conical band with slant-separated apertures using its authored cut.
/// Contours may clear or cross the cut. They can cover the angular period in projection.
/// Example: `let result = normal_extrude_cone_band_holes_at_authored_seam(&band, &holes, [-0.25, 0.5], options)?;`.
pub fn normal_extrude_cone_band_holes_at_authored_seam(
    band: &FullTurnBand,
    holes: &[ConeBandHole],
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Result<CrossedBandResult, SurfaceSketchFailure> {
    require_cone(band)?;
    extrude(
        band,
        holes.iter().copied().map(Contour::from).collect(),
        depths,
        options,
        SeamScope::AllowClear,
    )
}

fn require_cone(band: &FullTurnBand) -> Result<(), SurfaceSketchFailure> {
    if let FullTurnSupport::Cylinder { support, .. } = band.support {
        let cause = SurfaceFailureCause::InvalidInput {
            field: "cylinder radius",
            value: support.radius,
            expected: "conical support for a developed aperture",
        };
        return Err(failure_in_admission(band, cause));
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SeamScope {
    CrossingsOnly,
    AllowClear,
}

fn failure_in_admission(band: &FullTurnBand, cause: SurfaceFailureCause) -> SurfaceSketchFailure {
    SurfaceSketchFailure {
        region: band.region,
        stage: SurfaceStage::Admission,
        cause,
    }
}

fn extrude(
    band: &FullTurnBand,
    holes: Vec<Contour>,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
    scope: SeamScope,
) -> Result<CrossedBandResult, SurfaceSketchFailure> {
    let mut prepared = CrossedPlan::admit(*band, holes, depths, options, scope)
        .map_err(|cause| failure_in_admission(band, cause))?;
    let cut = prepared.subtract()?;
    let (body, bindings) = binding::bind(&prepared, cut)
        .map_err(|cause| prepared.stock.sweep.failure(SurfaceStage::History, cause))?;
    prepared.subtract_analytic_measures();
    let geometry = publish(&prepared, body, bindings)?;
    let crossings = prepared
        .holes
        .iter()
        .filter_map(|hole| hole.crossings.as_ref())
        .flat_map(|crossing| crossing.roots)
        .collect();
    Ok(CrossedBandResult {
        geometry,
        crossings,
    })
}

fn publish(
    prepared: &CrossedPlan,
    body: topods::Shape,
    bindings: binding::SplitBindings,
) -> Result<NormalExtrusionResult<PeriodicImageKey>, SurfaceSketchFailure> {
    let sweep = &prepared.stock.sweep;
    let keys = bindings.iter().map(|(key, _)| *key).collect::<Vec<_>>();
    let frozen = encoding::encode(&body, &bindings)
        .map_err(|cause| sweep.failure(SurfaceStage::Encoding, cause))?;
    let (decoded, references) = encoding::decode(&frozen, &keys)
        .map_err(|cause| sweep.failure(SurfaceStage::Encoding, cause))?;
    let checked = verification::validate_crossed(prepared, &decoded, &references)
        .map_err(|cause| sweep.failure(SurfaceStage::Validation, cause))?;
    let (volume, errors) = volume::measure(&decoded, sweep)
        .map_err(|cause| sweep.failure(SurfaceStage::Volume, cause))?;
    Ok(result::assemble(
        sweep, frozen, keys, checked, volume, errors,
    ))
}
