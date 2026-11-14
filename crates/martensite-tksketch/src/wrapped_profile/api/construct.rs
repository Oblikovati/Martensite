//! One entry point composes source resolution, the existing geometry pipeline and publication.
use super::*;
use std::collections::BTreeMap;

/// Extrude selected polynomial profiles along cylinder normals and retain achieved evidence.
/// The physical frame is normalized/projected from the requested axis and radial vectors.
/// Example: `let result = normal_extrude_profile(&request)?;`.
pub fn normal_extrude_profile(request: &WrappedNormalRequest) -> Checked<WrappedNormalResult> {
    normal_extrude_profile_with_cancel(request, || false)
}
/// Cooperatively cancel at source, proof and handoff boundaries without returning partial geometry.
/// Example: `normal_extrude_profile_with_cancel(&request, || cancellation.is_requested())`.
pub fn normal_extrude_profile_with_cancel(
    request: &WrappedNormalRequest,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<WrappedNormalResult> {
    input::poll(&mut cancelled, WrappedStage::Admission)?;
    let inputs = input::read(request, &mut cancelled)?;
    let family = construct_family(&inputs, request, &mut cancelled)?;
    input::poll(&mut cancelled, WrappedStage::History)?;
    let history = project_history(&family, &inputs, request.support.normal)?;
    let mut result = publish(request, family, history)?;
    if let Some(plan) = inputs.placement {
        placement::publish(&mut result, *plan, &mut cancelled)
            .map_err(|failure| failure_projection::selected(failure, request))?;
    }
    input::poll(&mut cancelled, WrappedStage::Publication)?;
    Ok(result)
}
fn publish(
    request: &WrappedNormalRequest,
    family: family::BudgetedFamily,
    history: history_projection::HistoryPublication,
) -> Checked<WrappedNormalResult> {
    let reports = member_reports(&family)?;
    let budget = family_budget(request, &family);
    Ok(WrappedNormalResult {
        family,
        history,
        reports,
        budget,
        support: request.support,
        depths: request.depths,
        options: request.options,
        placement: None,
    })
}
fn family_budget(
    request: &WrappedNormalRequest,
    family: &family::BudgetedFamily,
) -> WrappedBudgetReport {
    reports::budget(
        &family.budget,
        request.options.volume,
        family.family.selection.prepared.planar.certificate,
        family.family.selection.prepared.planar.proof_work,
    )
}

fn member_reports(
    family: &family::BudgetedFamily,
) -> Checked<BTreeMap<SurfaceRegionId, WrappedMemberReport>> {
    family
        .family
        .members
        .iter()
        .map(|(region, retained)| member_report(family, *region, retained))
        .collect()
}
fn member_report(
    family: &family::BudgetedFamily,
    region: SurfaceRegionId,
    retained: &pipeline::RetainedMember,
) -> Checked<(SurfaceRegionId, WrappedMemberReport)> {
    let bounds = family
        .budget
        .members
        .get(&region)
        .ok_or_else(|| missing_source(region))?;
    let planar = family
        .family
        .selection
        .prepared
        .planar
        .members
        .get(&region)
        .ok_or_else(|| missing_source(region))?;
    Ok((region, reports::member(retained, bounds, planar)))
}
fn missing_source(region: SurfaceRegionId) -> WrappedNormalFailure {
    error::failure(
        WrappedStage::Publication,
        WrappedFailureClass::Incomplete,
        Some(region),
        error::Cause::Family(family::FamilyFailure::Budget(Box::new(
            publication_budget::BudgetFailure::Source(region),
        ))),
    )
}
fn construct_family(
    inputs: &input::Inputs,
    request: &WrappedNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<family::BudgetedFamily> {
    family::construct_budgeted_with_work(
        &inputs.requests,
        request.options.volume,
        request.options.planar_work,
        |_| cancelled(),
    )
    .map_err(|cause| failure_projection::family(cause, &inputs.requests))
}
fn project_history(
    family: &family::BudgetedFamily,
    inputs: &input::Inputs,
    normal: crate::surface_sketch::NormalSense,
) -> Checked<history_projection::HistoryPublication> {
    history_projection::prepare(
        &family.family.keys,
        &family.family.history,
        &inputs.directions,
        normal,
    )
    .map_err(|cause| {
        error::failure(
            WrappedStage::History,
            WrappedFailureClass::Incomplete,
            None,
            error::Cause::History(cause),
        )
    })
}
