//! Validation uses physical solids; published canonical and world bounds stay distinct.
use super::*;
use crate::surface_sketch::HistoryRole;
pub(in super::super) fn publish(
    result: &mut WrappedNormalResult,
    plan: Plan,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<()> {
    let physical = apply(result, plan, cancelled)?;
    result.placement = Some(Box::new(Placed {
        bytes: physical.bytes,
        report: WrappedPlacementReport {
            spatial_budget: physical.budget,
            members: physical.members,
        },
    }));
    Ok(())
}
pub(super) struct PhysicalPublication<B> {
    pub bytes: Vec<u8>,
    pub budget: B,
    pub members: BTreeMap<SurfaceRegionId, WrappedWorldSpatialReport>,
}
pub(super) fn apply<B: PlacementBudget>(
    result: &mut WrappedNormalResult,
    plan: Plan<B>,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<PhysicalPublication<B>> {
    input::poll(cancelled, WrappedStage::Placement)?;
    let physical = freeze::place(&result.family.family, &plan)?;
    finish(result, plan, physical, cancelled)
}
pub(super) fn finish<B: PlacementBudget>(
    result: &mut WrappedNormalResult,
    plan: Plan<B>,
    physical: freeze::Physical,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<PhysicalPublication<B>> {
    let members = validate_members(result, &plan, &physical, cancelled)?;
    measures::transport(result, &plan)?;
    result.budget.spatial_error = result.options.spatial_error;
    Ok(PhysicalPublication {
        bytes: physical.bytes,
        budget: plan.budget,
        members,
    })
}

fn validate_members<B: PlacementBudget>(
    result: &mut WrappedNormalResult,
    plan: &Plan<B>,
    physical: &freeze::Physical,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<BTreeMap<SurfaceRegionId, WrappedWorldSpatialReport>> {
    let mut members = BTreeMap::new();
    for (region, report) in &mut result.reports {
        input::poll(cancelled, WrappedStage::Placement)?;
        let shape = solid(*region, physical)?;
        validate(*region, report, shape, result.options.spatial_error)?;
        let trace = trace::certify(*region, retained(&result.family.family, *region)?, plan)?;
        members.insert(*region, world_report(report, plan, trace)?);
    }
    Ok(members)
}
fn solid(region: SurfaceRegionId, physical: &freeze::Physical) -> Checked<&Shape> {
    let key = BindingKey {
        source: Source::Region(region),
        role: HistoryRole::Solid,
    };
    physical
        .bindings
        .iter()
        .find(|(candidate, _)| *candidate == key)
        .map(|(_, shape)| shape)
        .ok_or_else(|| fail(PlacementFailure::Member(region)))
}
fn validate(
    region: SurfaceRegionId,
    report: &mut WrappedMemberReport,
    shape: &Shape,
    tolerance: f64,
) -> Checked<()> {
    let validation = physical_validation(region, shape, tolerance)?;
    let (value, integration_error) = at("physical native volume", pipeline::estimate(shape))?;
    report.validation = validation;
    report.measures.native_volume = WrappedNativeVolumeEstimate {
        value,
        integration_error,
    };
    Ok(())
}
fn physical_validation(
    region: SurfaceRegionId,
    shape: &Shape,
    tolerance: f64,
) -> Checked<martensite_tkbo::body_check::ValidationReport> {
    let validation = at(
        "physical ordered validation",
        pipeline::ordered_checks(shape, tolerance),
    )?;
    error::budget(publication_budget::certify_native(
        region,
        &validation,
        tolerance,
    ))?;
    Ok(validation)
}
fn world_report<B: PlacementBudget>(
    report: &WrappedMemberReport,
    plan: &Plan<B>,
    trace: WrappedCertifiedBound,
) -> Checked<WrappedWorldSpatialReport> {
    let spatial = &report.spatial;
    Ok(WrappedWorldSpatialReport {
        profile_error: world_bound(spatial.profile_error, plan)?,
        normal_storage_error: world_bound(spatial.normal_storage_error, plan)?,
        cap_trace_error: enforce_world(trace.upper, plan.budget.input().spatial_limit)?,
        cap_realization_error: world_bound(spatial.cap_realization_error, plan)?,
    })
}
fn world_bound<B: PlacementBudget>(
    bound: WrappedCertifiedBound,
    plan: &Plan<B>,
) -> Checked<WrappedCertifiedBound> {
    let quantity = publication_budget::Quantity::CapDomain;
    let scaled = error::budget(publication_budget::scale_upper(
        bound.upper,
        plan.budget.frame().stretch_bounds()[1],
        quantity,
    ))?;
    let world = error::budget(publication_budget::add_upper(
        scaled,
        plan.budget.placement_error_bound(),
        quantity,
    ))?;
    enforce_world(world, plan.budget.input().spatial_limit)
}
fn enforce_world(world: f64, requested: f64) -> Checked<WrappedCertifiedBound> {
    if world > requested {
        return error::budget(Err(publication_budget::BudgetFailure::Limit {
            quantity: publication_budget::Quantity::WorldSpatial,
            actual: world,
            limit: requested,
            region: None,
            span: None,
        }));
    }
    Ok(reports::bound(world))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn world_limit_refusal_reports_achieved_world_error_without_relabeling_it_as_placement_only() {
        assert_eq!(enforce_world(0.5, 0.5).unwrap().upper, 0.5);
        let message = enforce_world(0.75, 0.5).err().unwrap().to_string();
        assert!(message.contains("WorldSpatial=0.75"));
        assert!(message.contains("expected <= 0.5"));
    }
}
