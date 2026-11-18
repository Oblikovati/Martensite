//! One declared budget accompanies the frozen family; publication qualification remains separate.
use super::*;
#[cfg(test)]
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
    operation::construct_budgeted_with_work(
        requests,
        volume,
        martensite_tkbo::profile_extrusion::PlanarProofWorkLimit::CurrentDefault,
        cancelled,
    )
}
