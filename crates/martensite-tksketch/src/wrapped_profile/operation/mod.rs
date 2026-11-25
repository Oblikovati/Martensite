//! The operation owns stage order; support-specific construction does not publish results.
use super::*;
use api::{error, history_projection, input, placement, reports};
pub(super) mod cone;
pub(super) mod cylinder;
mod execute;
pub(super) mod frozen;
mod handedness;
pub(super) mod members;
mod publication;
pub(super) mod source;
pub(super) mod stages;
pub(super) mod topology;
#[cfg(test)]
pub(super) use cylinder::{construct_budgeted_with_work, construct_controlled};
type Checked<T> = Result<T, WrappedNormalFailure>;

/// Extrude selected polynomial profiles along cylinder normals and retain achieved evidence.
/// Weighted spans are admitted when a same-parameter polynomial identity is exact.
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
    from_inputs(request, inputs, &mut cancelled)
}
pub(super) fn from_inputs(
    request: &WrappedNormalRequest,
    inputs: input::Inputs,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedNormalResult> {
    from_inputs_with_proof(
        request,
        inputs,
        martensite_tkbo::profile_extrusion::PlanarProofVersion::CarrierV2,
        cancelled,
    )
}

pub(super) fn from_inputs_with_proof(
    request: &WrappedNormalRequest,
    inputs: input::Inputs,
    version: martensite_tkbo::profile_extrusion::PlanarProofVersion,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedNormalResult> {
    from_inputs_with_preparation(request, inputs, version.into(), cancelled)
}

pub(super) fn from_inputs_with_preparation(
    request: &WrappedNormalRequest,
    inputs: input::Inputs,
    preparation: selection::Preparation,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<WrappedNormalResult> {
    let family = cylinder::construct_family(&inputs, request, preparation, cancelled)?;
    input::poll(cancelled, WrappedStage::History)?;
    let history = publication::project_history(&family, &inputs, request.support.normal)?;
    let mut result = publication::publish(request, family, history)?;
    if let Some(plan) = inputs.placement {
        placement::publish(&mut result, *plan, cancelled)?;
    }
    input::poll(cancelled, WrappedStage::Publication)?;
    Ok(result)
}
