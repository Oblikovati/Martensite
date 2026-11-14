//! One declared budget accompanies the frozen family; publication qualification remains separate.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit;
#[cfg(test)]
mod tests;
pub(in super::super) struct BudgetedFamily {
    pub family: UnqualifiedFamily,
    pub budget: publication_budget::BudgetEvidence,
}
#[cfg(test)]
pub(in super::super) fn construct_budgeted(
    requests: &[Request],
    volume: CylindricalMeasureLimit,
) -> Checked<BudgetedFamily> {
    construct_budgeted_with_cancel(requests, volume, |_| false)
}
#[cfg(test)]
pub(in super::super) fn construct_budgeted_with_cancel(
    requests: &[Request],
    volume: CylindricalMeasureLimit,
    cancelled: impl FnMut(Checkpoint) -> bool + Send,
) -> Checked<BudgetedFamily> {
    construct_budgeted_with_work(
        requests,
        volume,
        martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
        cancelled,
    )
}
pub(in super::super) fn construct_budgeted_with_work(
    requests: &[Request],
    volume: CylindricalMeasureLimit,
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    mut cancelled: impl FnMut(Checkpoint) -> bool + Send,
) -> Checked<BudgetedFamily> {
    let limits = publication_budget::requested_limits(requests, volume).map_err(budget_failure)?;
    let mut control = Control::new(&mut cancelled);
    let family = construct_controlled(requests, work, &mut control)?;
    let budget = control.run(FamilyStage::Budget, None, |_| {
        publication_budget::certify_with_limits(requests, &family, limits).map_err(budget_failure)
    })?;
    control.run(FamilyStage::Completion, None, |_| {
        Ok(BudgetedFamily { family, budget })
    })
}
fn budget_failure(cause: publication_budget::BudgetFailure) -> FamilyFailure {
    FamilyFailure::Budget(Box::new(cause))
}
