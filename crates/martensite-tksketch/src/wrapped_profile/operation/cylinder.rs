//! Cylinder stages feed the operation's existing frozen evidence and publication contract.
use super::*;
use family::control::Checkpoint;
use family::{BudgetedFamily, Control, FamilyFailure, FamilyStage, UnqualifiedFamily};
use martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit;
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
pub(in crate::surface_sketch::wrapped_profile) fn construct_controlled(
    requests: &[Request],
    work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    control: &mut Control<'_>,
) -> FamilyChecked<UnqualifiedFamily> {
    let selection = control.run(FamilyStage::Selection, None, |poll| {
        selection::certify_with_work(requests, Default::default(), work, poll)
            .map_err(|cause| FamilyFailure::Selection(Box::new(cause)))
    })?;
    let frozen = family::freeze::construct_with_control(&selection.prepared.prepared, control)?;
    family::qualify_frozen(requests, selection, frozen, control)
}
