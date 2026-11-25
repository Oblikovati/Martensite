//! Rebind trace limits and qualify the actual located cone solid for each authored member.
use super::*;
use martensite_tkbo::body_check::ValidationReport;
pub(super) fn certify(
    base: &WrappedConeNormalResult,
    plan: &ConePlan,
    physical: &freeze::Physical,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<BTreeMap<SurfaceRegionId, WrappedPlacedConeMemberReport>> {
    let certificate = qualification::capture(base, physical)?;
    base.reports()
        .iter()
        .map(|(region, report)| {
            input::poll(cancelled, WrappedStage::Placement)?;
            let solid = publication::solid(*region, physical)?;
            let trace = cap_trace(*region, base, plan, cancelled)?;
            let validation = qualification::validate(certificate.as_ref(), *region, solid, plan)?;
            Ok((*region, member(report, solid, trace, plan, validation)?))
        })
        .collect()
}
fn member(
    source: &WrappedConeMemberReport,
    solid: &Shape,
    cap_trace_error: WrappedCertifiedBound,
    plan: &ConePlan,
    validation: ValidationReport,
) -> Checked<WrappedPlacedConeMemberReport> {
    let (value, integration_error) = at("physical native cone volume", pipeline::estimate(solid))?;
    let boundary_flux = measures::product(
        source.boundary_flux,
        plan.budget.frame().determinant_bounds(),
    )?;
    let discrepancy = measures::difference(boundary_flux, source.reference_volume)?;
    Ok(WrappedPlacedConeMemberReport {
        validation,
        boundary_error: publication::world_bound(source.boundary_error, plan)?,
        ruled_storage_error: publication::world_bound(source.ruled_storage_error, plan)?,
        cap_trace_error,
        cap_realization_error: publication::world_bound(
            source.material.cap_realization_error,
            plan,
        )?,
        boundary_flux,
        proxy_reference_discrepancy: reports::bound(discrepancy),
        native_volume: WrappedNativeVolumeEstimate {
            value,
            integration_error,
        },
    })
}
fn cap_trace(
    region: SurfaceRegionId,
    base: &WrappedConeNormalResult,
    plan: &ConePlan,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedCertifiedBound> {
    let owned = base
        .family
        .family
        .members
        .get(&region)
        .ok_or_else(|| fail(PlacementFailure::Member(region)))?;
    let mut maximum = 0_f64;
    for span in &owned.geometry.spans {
        input::poll(cancelled, WrappedStage::Placement)?;
        for (depth, trim) in span.trims.iter().enumerate() {
            let actual = trace::transport(
                trim.residual_bounds()[2],
                plan.budget.frame().stretch_bounds()[1],
                plan.budget.evaluation_error_bound(),
            )?;
            let checked = trace::enforce(
                region,
                span.span,
                depth,
                actual,
                trim.requested_spatial_limit(),
            )?;
            maximum = maximum.max(checked);
        }
    }
    publication::enforce_world(maximum, plan.budget.input().spatial_limit)
}
