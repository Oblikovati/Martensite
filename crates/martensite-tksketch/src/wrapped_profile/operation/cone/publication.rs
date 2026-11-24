//! A complete publication retains the shared archive, history and qualified evidence once.
use super::*;
use crate::surface_sketch::wrapped_profile::api::error as public_error;
mod reports;
#[cfg(test)]
mod tests;
#[cfg(test)]
pub(super) use tests::assert_publication;

pub(super) fn publish(
    request: &WrappedConeNormalRequest,
    inputs: Inputs,
    family: ConeBudgeted,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedConeNormalResult> {
    input::poll(cancelled, WrappedStage::History)?;
    let history = project_history(request, &inputs, &family)?;
    let reports = reports::members(&family).map_err(ConeFailure::public)?;
    let budget = reports::budget(request, &family);
    super::material::validate_publication(&family).map_err(ConeFailure::public)?;
    input::poll(cancelled, WrappedStage::Publication)?;
    Ok(assemble(request, family, history, reports, budget))
}

fn assemble(
    request: &WrappedConeNormalRequest,
    family: ConeBudgeted,
    history: history_projection::HistoryPublication,
    reports: BTreeMap<SurfaceRegionId, WrappedConeMemberReport>,
    budget: WrappedConeBudgetReport,
) -> WrappedConeNormalResult {
    WrappedConeNormalResult {
        family,
        history,
        reports,
        budget,
        support: request.support,
        depths: request.depths,
        options: request.options,
        proof: request.proof,
    }
}

fn project_history(
    request: &WrappedConeNormalRequest,
    inputs: &Inputs,
    family: &ConeBudgeted,
) -> Checked<history_projection::HistoryPublication> {
    history_projection::prepare_physical(
        &family.family.keys,
        &family.family.history,
        &inputs.directions,
        request.support.normal,
    )
    .map_err(history_failure)
}

fn history_failure(cause: history_projection::HistoryMapFailure) -> WrappedNormalFailure {
    public_error::failure(
        WrappedStage::History,
        WrappedFailureClass::Incomplete,
        None,
        public_error::Cause::History(cause),
    )
}
