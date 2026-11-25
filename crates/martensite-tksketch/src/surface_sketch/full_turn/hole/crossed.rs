// SPDX-License-Identifier: GPL-2.0-only
//! Publish the aperture when the actual native cut passes through its material boundary.
use super::*;

mod binding;
mod names;
pub(in crate::surface_sketch::full_turn) use binding::seam_endpoints;
pub use names::*;

/// Published cylindrical band and its independently named periodic edge fragments.
#[derive(Clone, Debug)]
pub struct CrossedBandResult {
    geometry: NormalExtrusionResult<PeriodicImageKey>,
    crossings: [PeriodicCrossing; 2],
}

impl CrossedBandResult {
    /// Immutable geometry, volume, validation and named native export.
    /// Example: `result.geometry().export_brep()?`.
    pub fn geometry(&self) -> &NormalExtrusionResult<PeriodicImageKey> {
        &self.geometry
    }

    /// Lower and upper aperture crossings in authored contour order.
    /// Example: `result.crossings()[0].normalized_parameter`.
    pub fn crossings(&self) -> &[PeriodicCrossing] {
        &self.crossings
    }
}

/// Extrude one cylindrical band with a rectangular aperture across its actual native seam.
///
/// The aperture must cross the occupied chart's zero-arc meridian transversely.
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
    let mut prepared =
        Cutout::admit(band, hole, depths, options, authored_cut).map_err(|cause| {
            SurfaceSketchFailure {
                region: band.region,
                stage: SurfaceStage::Admission,
                cause,
            }
        })?;
    let crossings = names::crossings(hole)
        .map_err(|cause| prepared.stock.sweep.failure(SurfaceStage::Admission, cause))?;
    let (proposal, stock, aperture) = prepared.proposal(band.rims)?;
    let (body, bindings) = binding::bind(proposal, &stock, &aperture, band, hole, &crossings)
        .map_err(|cause| prepared.stock.sweep.failure(SurfaceStage::History, cause))?;
    prepared.subtract_analytic_measure();
    let geometry = publish(&prepared, band, hole, &crossings, body, bindings)?;
    Ok(CrossedBandResult {
        geometry,
        crossings,
    })
}

fn authored_cut(aperture: &cylinder_admission::Admitted) -> Checked<f64> {
    let rectangle = aperture.rectangle;
    let seam = rectangle.support.chart_origin;
    let rounding = 64. * f64::EPSILON * (seam.abs() + aperture.angle + TAU);
    let reserve = (effective_limits(aperture.options).angular_clearance + rounding)
        * rectangle.support.radius;
    let clearance = (-rectangle.s[0]).min(rectangle.s[1]);
    cylinder_admission::require(
        clearance > reserve,
        "authored seam crossing clearance",
        clearance,
        "two transverse aperture rim crossings strictly inside their original intervals",
    )?;
    Ok(seam)
}

fn publish(
    prepared: &Cutout,
    band: &FullTurnBand,
    hole: &CylinderBandHole,
    crossings: &[PeriodicCrossing; 2],
    body: topods::Shape,
    bindings: binding::SplitBindings,
) -> Result<NormalExtrusionResult<PeriodicImageKey>, SurfaceSketchFailure> {
    let sweep = &prepared.stock.sweep;
    let keys = bindings.iter().map(|(key, _)| *key).collect::<Vec<_>>();
    let frozen = encoding::encode(&body, &bindings)
        .map_err(|cause| sweep.failure(SurfaceStage::Encoding, cause))?;
    let (decoded, references) = encoding::decode(&frozen, &keys)
        .map_err(|cause| sweep.failure(SurfaceStage::Encoding, cause))?;
    let checked =
        verification::validate_crossed(prepared, band, hole, crossings, &decoded, &references)
            .map_err(|cause| sweep.failure(SurfaceStage::Validation, cause))?;
    let (volume, errors) = volume::measure(&decoded, sweep)
        .map_err(|cause| sweep.failure(SurfaceStage::Volume, cause))?;
    Ok(result::assemble(
        sweep, frozen, keys, checked, volume, errors,
    ))
}
