//! Publish retained evidence without converting or copying the existing report owners.
use super::*;
use std::collections::BTreeMap;

pub(super) fn publish(
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
pub(super) fn project_history(
    family: &family::BudgetedFamily,
    inputs: &input::Inputs,
    normal: crate::surface_sketch::NormalSense,
) -> Checked<history_projection::HistoryPublication> {
    history_projection::prepare_physical(
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
