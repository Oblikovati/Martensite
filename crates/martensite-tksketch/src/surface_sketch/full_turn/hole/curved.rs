// SPDX-License-Identifier: GPL-2.0-only
//! Publish an actual periodic cut of an independently certified fitted aperture.
use super::*;
use crate::surface_sketch::full_turn::hole::crossed::CrossedBandResult;

mod admission;
mod binding;
mod proof;
mod rational;
use admission::{AperturePlan, Scope, SourceModel};
pub use rational::*;
use std::collections::BTreeMap;

/// Immutable curved aperture body, split provenance and achieved fitting bounds.
#[derive(Clone, Debug)]
pub struct CurvedBandResult<T = WrappedMemberReport> {
    publication: CrossedBandResult,
    tool: T,
    trace_bound: f64,
    native_parameter_error: f64,
}

impl<T> CurvedBandResult<T> {
    /// Geometry and exact authored crossing addresses. Example: `result.publication().crossings()`.
    pub fn publication(&self) -> &CrossedBandResult {
        &self.publication
    }

    /// The matched fitter and input evidence. Example: `result.tool().coverage.faces`.
    pub fn tool(&self) -> &T {
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
/// canonical +Z cylinder frame, with two transverse crossings of one signed lift of the authored zero meridian.
/// Uses the same physical depths as the wrapped aperture request.
/// Example: `let cut = normal_extrude_cylinder_band_with_curved_hole(&band, &profile, options)?;`.
pub fn normal_extrude_cylinder_band_with_curved_hole(
    band: &FullTurnBand,
    request: &WrappedNormalRequest,
    options: NormalExtrusionOptions,
) -> Result<CurvedBandResult, SurfaceSketchFailure> {
    let plan = AperturePlan::admit(
        band,
        request,
        options,
        Scope::OneCrossed,
        SourceModel::Polynomial,
    )
    .map_err(|cause| admission_failure(band, cause))?;
    let cutter = fitted_cutter(&plan, request)?;
    let report = report(&plan, cutter.reports())?;
    let evidence = FitEvidence::cylinder(&plan, &report, request.support.radius)
        .map_err(|cause| plan.stock.sweep.failure(SurfaceStage::Validation, cause))?;
    let detached = cutter
        .export_native()
        .map_err(|cause| export_failure(&plan, cause))?;
    finish(plan, report, evidence, &detached)
}

/// Make a curved aperture across the actual seam of a canonical full conical band.
/// Admits one loop of lines and clamped polynomial quadratics in a positive-X developed
/// chart, with two transverse crossings of the zero meridian ray.
/// Example: `let cut = normal_extrude_cone_band_with_curved_hole(&band, &profile, options)?;`.
pub fn normal_extrude_cone_band_with_curved_hole(
    band: &FullTurnBand,
    request: &WrappedConeNormalRequest,
    options: NormalExtrusionOptions,
) -> Result<CurvedBandResult<WrappedConeMemberReport>, SurfaceSketchFailure> {
    let plan = AperturePlan::admit_cone(
        band,
        request,
        options,
        Scope::OneCrossed,
        SourceModel::Polynomial,
    )
    .map_err(|cause| admission_failure(band, cause))?;
    let cutter =
        normal_extrude_cone_profile(request).map_err(|cause| tool_failure(&plan, cause))?;
    let report = report(&plan, cutter.reports())?;
    let evidence = FitEvidence::cone(&plan, &report)
        .map_err(|cause| plan.stock.sweep.failure(SurfaceStage::Validation, cause))?;
    let detached = cutter
        .export_native()
        .map_err(|cause| export_failure(&plan, cause))?;
    finish(plan, report, evidence, &detached)
}

/// Cut multiple meridian-separated fitted apertures in a canonical cylindrical band.
/// At least one selected outer loop contains a curved span. The rest may be rectangles.
/// Each aperture can clear or cross the authored seam.
/// Example: `let cut = normal_extrude_cylinder_band_with_curved_holes(&band, &profile, options)?;`.
pub fn normal_extrude_cylinder_band_with_curved_holes(
    band: &FullTurnBand,
    request: &WrappedNormalRequest,
    options: NormalExtrusionOptions,
) -> Result<CurvedBandResult<BTreeMap<SurfaceRegionId, WrappedMemberReport>>, SurfaceSketchFailure>
{
    let request = canonical_cylinder_request(request);
    let plan = AperturePlan::admit(
        band,
        &request,
        options,
        Scope::Batch,
        SourceModel::Polynomial,
    )
    .map_err(|cause| admission_failure(band, cause))?;
    let cutter = fitted_cutter(&plan, &request)?;
    let reports = cutter.reports();
    let evidence = batch_evidence(&plan, reports, |report| {
        FitEvidence::cylinder(&plan, report, request.support.radius)
    })?;
    let detached = cutter
        .export_native()
        .map_err(|cause| export_failure(&plan, cause))?;
    finish(plan, reports.clone(), evidence, &detached)
}

/// Cut multiple meridian-separated fitted apertures in a canonical conical band.
/// The aperture batch may mix curved and straight contours clear of or crossing the zero ray.
/// Example: `let cut = normal_extrude_cone_band_with_curved_holes(&band, &profile, options)?;`.
pub fn normal_extrude_cone_band_with_curved_holes(
    band: &FullTurnBand,
    request: &WrappedConeNormalRequest,
    options: NormalExtrusionOptions,
) -> Result<
    CurvedBandResult<BTreeMap<SurfaceRegionId, WrappedConeMemberReport>>,
    SurfaceSketchFailure,
> {
    let mut request = request.clone();
    request.selections.sort_by_key(|selection| selection.region);
    let plan = AperturePlan::admit_cone(
        band,
        &request,
        options,
        Scope::Batch,
        SourceModel::Polynomial,
    )
    .map_err(|cause| admission_failure(band, cause))?;
    let cutter =
        normal_extrude_cone_profile(&request).map_err(|cause| tool_failure(&plan, cause))?;
    let reports = cutter.reports();
    let evidence = batch_evidence(&plan, reports, |report| FitEvidence::cone(&plan, report))?;
    let detached = cutter
        .export_native()
        .map_err(|cause| export_failure(&plan, cause))?;
    finish(plan, reports.clone(), evidence, &detached)
}

fn canonical_cylinder_request(request: &WrappedNormalRequest) -> WrappedNormalRequest {
    let mut owned = request.clone();
    owned.selections.sort_by_key(|selection| selection.region);
    owned
}

fn finish<T>(
    mut plan: AperturePlan,
    tool: T,
    evidence: FitEvidence,
    detached: &DetachedWrappedBodies,
) -> Result<CurvedBandResult<T>, SurfaceSketchFailure> {
    let stock = export_stock(&plan.stock, plan.band.rims)?;
    let bound = binding::bind(&plan, &stock, detached, evidence.angular_error)
        .map_err(|cause| plan.stock.sweep.failure(SurfaceStage::History, cause))?;
    let authored = evidence.authored_volume;
    plan.stock.sweep.expected_volume -= authored.lower.midpoint(authored.upper);
    publish(&plan, tool, evidence.angular_error, detached, bound)
}

struct FitEvidence {
    authored_volume: WrappedCertifiedRange,
    angular_error: f64,
}

impl FitEvidence {
    fn cylinder(plan: &AperturePlan, report: &WrappedMemberReport, radius: f64) -> Checked<Self> {
        let spatial = (report.spatial.profile_error.upper / radius).next_up();
        let chart = (report.spatial.cap_angular_error.upper + spatial).next_up();
        let trace = angle_from_chord(plan.minimum_radius(), report.spatial.cap_trace_error.upper)?;
        let angular_error = (chart + trace).next_up();
        Ok(Self {
            authored_volume: report.measures.authored_volume,
            angular_error,
        })
    }

    fn cone(plan: &AperturePlan, report: &WrappedConeMemberReport) -> Checked<Self> {
        let space = (report.boundary_error.upper + report.cap_trace_error.upper).next_up();
        let storage = report.depth_storage_error[0]
            .upper
            .max(report.depth_storage_error[1].upper);
        let angular_error = angle_from_chord(plan.minimum_radius(), (space + storage).next_up())?;
        Ok(Self {
            authored_volume: report.authored_volume,
            angular_error,
        })
    }
}

fn angle_from_chord(radius: f64, distance: f64) -> Checked<f64> {
    let lower = (radius - distance).next_down();
    cylinder_admission::require(
        lower > 0.,
        "curved angular discrepancy reference radius",
        lower,
        "positive lower radius after the fit or trace discrepancy",
    )?;
    let ratio = (distance / lower).next_up();
    Ok((ratio * (PI.next_up() / 2.)).next_up())
}

fn report<T: Clone>(
    plan: &AperturePlan,
    reports: &BTreeMap<SurfaceRegionId, T>,
) -> Result<T, SurfaceSketchFailure> {
    let [contour] = plan.contours.as_slice() else {
        return Err(plan.stock.sweep.failure(
            SurfaceStage::Admission,
            incomplete(
                "one selected fitted aperture",
                format!("{}", plan.contours.len()),
            ),
        ));
    };
    reports
        .get(&contour.region)
        .cloned()
        .ok_or_else(|| missing_report(plan, contour.region))
}

fn batch_evidence<T>(
    plan: &AperturePlan,
    reports: &BTreeMap<SurfaceRegionId, T>,
    transfer: impl Fn(&T) -> Checked<FitEvidence>,
) -> Result<FitEvidence, SurfaceSketchFailure> {
    let mut lower: f64 = 0.;
    let mut upper: f64 = 0.;
    let mut angle: f64 = 0.;
    for contour in &plan.contours {
        let report = reports
            .get(&contour.region)
            .ok_or_else(|| missing_report(plan, contour.region))?;
        let proof = transfer(report)
            .map_err(|cause| plan.stock.sweep.failure(SurfaceStage::Validation, cause))?;
        lower = (lower + proof.authored_volume.lower).next_down();
        upper = (upper + proof.authored_volume.upper).next_up();
        angle = angle.max(proof.angular_error);
    }
    Ok(FitEvidence {
        authored_volume: WrappedCertifiedRange { lower, upper },
        angular_error: angle,
    })
}

fn missing_report(plan: &AperturePlan, owner: SurfaceRegionId) -> SurfaceSketchFailure {
    plan.stock.sweep.failure(
        SurfaceStage::Validation,
        incomplete("certified aperture cutter report", format!("{owner:?}")),
    )
}

fn admission_failure(band: &FullTurnBand, cause: SurfaceFailureCause) -> SurfaceSketchFailure {
    SurfaceSketchFailure {
        region: band.region,
        stage: SurfaceStage::Admission,
        cause,
    }
}

fn export_failure(plan: &AperturePlan, cause: WrappedNormalFailure) -> SurfaceSketchFailure {
    plan.stock.sweep.failure(
        SurfaceStage::Encoding,
        incomplete("detached certified curved aperture", format!("{cause:?}")),
    )
}

fn tool_failure(plan: &AperturePlan, cause: WrappedNormalFailure) -> SurfaceSketchFailure {
    plan.stock.sweep.failure(
        SurfaceStage::Construction,
        incomplete("certified wrapped aperture", format!("{cause:?}")),
    )
}

fn fitted_cutter(
    plan: &AperturePlan,
    request: &WrappedNormalRequest,
) -> Result<WrappedNormalResult, SurfaceSketchFailure> {
    normal_extrude_profile(request).map_err(|cause| tool_failure(plan, cause))
}

fn publish<T>(
    plan: &AperturePlan,
    tool: T,
    angular_error: f64,
    detached: &DetachedWrappedBodies,
    bound: binding::BoundCut,
) -> Result<CurvedBandResult<T>, SurfaceSketchFailure> {
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
    let checked = proof::validate(
        plan,
        &decoded,
        &entities,
        detached,
        &bound.sources,
        angular_error,
    )
    .map_err(|cause| sweep.failure(SurfaceStage::Validation, cause))?;
    let (volume, errors) = volume::measure(&decoded, sweep)
        .map_err(|cause| sweep.failure(SurfaceStage::Volume, cause))?;
    let publication = CrossedBandResult {
        geometry: result::assemble(sweep, bytes, keys, checked.body, volume, errors),
        crossings: plan.crossings().map(|crossing| crossing.report).collect(),
    };
    Ok(CurvedBandResult {
        publication,
        tool,
        trace_bound: checked.trace_bound,
        native_parameter_error: checked.native_parameter_error,
    })
}
