// SPDX-License-Identifier: GPL-2.0-only
//! Publish an actual periodic cut of an independently certified fitted aperture.
use super::*;
use crate::surface_sketch::full_turn::hole::crossed::CrossedBandResult;

mod admission;
mod binding;
mod proof;
use admission::AperturePlan;

/// Immutable curved aperture body, split provenance and achieved fitting bounds.
#[derive(Clone, Debug)]
pub struct CurvedBandResult {
    publication: CrossedBandResult,
    tool: WrappedMemberReport,
    trace_bound: f64,
    native_parameter_error: f64,
}

impl CurvedBandResult {
    /// Geometry and exact authored crossing addresses. Example: `result.publication().crossings()`.
    pub fn publication(&self) -> &CrossedBandResult {
        &self.publication
    }

    /// The matched fitter and input evidence. Example: `result.tool().coverage.faces`.
    pub fn tool(&self) -> &WrappedMemberReport {
        &self.tool
    }

    /// Certified full-interval trace bound for the actual decoded stock-side aperture edges.
    /// Example: `assert!(result.trace_bound() < requested_weld)`.
    pub fn trace_bound(&self) -> f64 {
        self.trace_bound
    }

    /// Bound on the native cut's deviation from each exact original normalized source root.
    /// Example: `assert!(result.native_parameter_error() < 1e-6)`.
    pub fn native_parameter_error(&self) -> f64 {
        self.native_parameter_error
    }
}

/// Make one curved aperture on a canonical full cylindrical band by cutting its actual seam.
/// Admits one simply connected loop of lines and clamped quadratic polynomial segments in a
/// canonical +Z cylinder frame, with two transverse crossings of the authored zero meridian.
/// Uses the same physical depths as the wrapped aperture request.
/// Example: `let cut = normal_extrude_cylinder_band_with_curved_hole(&band, &profile, options)?;`.
pub fn normal_extrude_cylinder_band_with_curved_hole(
    band: &FullTurnBand,
    request: &WrappedNormalRequest,
    options: NormalExtrusionOptions,
) -> Result<CurvedBandResult, SurfaceSketchFailure> {
    let plan =
        AperturePlan::admit(band, request, options).map_err(|cause| SurfaceSketchFailure {
            region: band.region,
            stage: SurfaceStage::Admission,
            cause,
        })?;
    run(plan, request)
}

fn run(
    mut plan: AperturePlan,
    request: &WrappedNormalRequest,
) -> Result<CurvedBandResult, SurfaceSketchFailure> {
    let stock = export_stock(&plan.stock, plan.band.rims)?;
    let cutter = fitted_cutter(&plan, request)?;
    let tool = cutter.reports().get(&plan.region).cloned().ok_or_else(|| {
        plan.stock.sweep.failure(
            SurfaceStage::Validation,
            incomplete(
                "one certified aperture cutter report",
                format!("{:?}", plan.region),
            ),
        )
    })?;
    let detached = cutter.export_native().map_err(|cause| {
        plan.stock.sweep.failure(
            SurfaceStage::Encoding,
            incomplete("detached certified curved aperture", format!("{cause:?}")),
        )
    })?;
    let bound = binding::bind(&plan, &stock, &detached, &tool)
        .map_err(|cause| plan.stock.sweep.failure(SurfaceStage::History, cause))?;
    let authored = &tool.measures.authored_volume;
    plan.stock.sweep.expected_volume -= authored.lower.midpoint(authored.upper);
    publish(&plan, tool, &detached, bound)
}

fn fitted_cutter(
    plan: &AperturePlan,
    request: &WrappedNormalRequest,
) -> Result<WrappedNormalResult, SurfaceSketchFailure> {
    normal_extrude_profile(request).map_err(|cause| {
        plan.stock.sweep.failure(
            SurfaceStage::Construction,
            incomplete("certified wrapped aperture", format!("{cause:?}")),
        )
    })
}

fn publish(
    plan: &AperturePlan,
    tool: WrappedMemberReport,
    detached: &DetachedWrappedBodies,
    bound: binding::BoundCut,
) -> Result<CurvedBandResult, SurfaceSketchFailure> {
    let sweep = &plan.stock.sweep;
    let keys = bound
        .bindings
        .iter()
        .map(|(key, _)| *key)
        .collect::<Vec<_>>();
    let bytes = encoding::encode(&bound.body, &bound.bindings)
        .map_err(|cause| sweep.failure(SurfaceStage::Encoding, cause))?;
    let (decoded, entities) = encoding::decode(&bytes, &keys)
        .map_err(|cause| sweep.failure(SurfaceStage::Encoding, cause))?;
    let checked = proof::validate(plan, &decoded, &entities, detached, &bound.sources, &tool)
        .map_err(|cause| sweep.failure(SurfaceStage::Validation, cause))?;
    let (volume, errors) = volume::measure(&decoded, sweep)
        .map_err(|cause| sweep.failure(SurfaceStage::Volume, cause))?;
    let publication = CrossedBandResult {
        geometry: result::assemble(sweep, bytes, keys, checked.body, volume, errors),
        crossings: plan.crossings.map(|crossing| crossing.report).to_vec(),
    };
    Ok(CurvedBandResult {
        publication,
        tool,
        trace_bound: checked.trace_bound,
        native_parameter_error: checked.native_parameter_error,
    })
}
