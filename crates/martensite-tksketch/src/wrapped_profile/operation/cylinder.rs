//! Cylinder stages feed the operation's existing frozen evidence and publication contract.
use super::*;
use family::control::Checkpoint;
use family::{BudgetedFamily, FamilyFailure};
#[cfg(test)]
use family::{Control, UnqualifiedFamily};
use martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit;
mod stages;
pub(in crate::surface_sketch::wrapped_profile) use stages::CylinderStages;
type FamilyChecked<T> = Result<T, FamilyFailure>;

pub(super) fn construct_family(
    inputs: &input::Inputs,
    request: &WrappedNormalRequest,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<family::BudgetedFamily> {
    construct_budgeted_with_work(
        &inputs.requests,
        request.options.volume,
        request.options.planar_work,
        |_| cancelled(),
    )
    .map_err(api::failure_projection::family)
}
pub(in crate::surface_sketch::wrapped_profile) fn construct_budgeted_with_work(
    requests: &[Request],
    volume: CylindricalMeasureLimit,
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    mut cancelled: impl FnMut(Checkpoint) -> bool + Send,
) -> FamilyChecked<BudgetedFamily> {
    execute::budgeted(&CylinderStages { work, volume }, requests, &mut cancelled)
}
fn budget_failure(cause: publication_budget::BudgetFailure) -> FamilyFailure {
    FamilyFailure::Budget(Box::new(cause))
}
#[cfg(test)]
pub(in crate::surface_sketch::wrapped_profile) fn construct_controlled(
    requests: &[Request],
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    control: &mut Control<'_>,
) -> FamilyChecked<UnqualifiedFamily> {
    execute::construct_with_context(
        &CylinderStages {
            work,
            volume: CylindricalMeasureLimit::Report,
        },
        requests,
        control,
        &mut (),
    )
}
