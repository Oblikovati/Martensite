//! Clear pending ownership only after its own fence completes or reports actual device loss.
use super::retirement_attempt::RetirementAttempt;
use super::{
    device_lifecycle::contains_device_loss,
    device_retirement::{CompletionDispatch, retire_until_stopped},
    runtime::VulkanRuntime,
    window_fence_retirement::{WindowFenceCalls, WindowFenceDispatch, WindowFenceOwner},
};
use crate::RenderError;
use ash::vk;
/// Try the pending fence once; `wait_window_fence(runtime, fence, &mut pending, owner)?`.
pub(super) fn wait_window_fence(
    runtime: &VulkanRuntime,
    fence: vk::Fence,
    pending: &mut bool,
    owner: WindowFenceOwner,
) -> Result<(), RenderError> {
    let mut calls = WindowFenceCalls::new(runtime, fence, owner);
    wait_pending_fence(&mut calls, pending)
}
/// Qualify one pending fence; `wait_pending_fence(&mut calls, &mut pending)?` preserves unknown ownership.
pub(super) fn wait_pending_fence(
    calls: &mut impl WindowFenceDispatch,
    pending: &mut bool,
) -> Result<(), RenderError> {
    if !*pending {
        return Ok(());
    }
    let outcome = calls.wait_completion();
    if outcome.as_ref().err().is_some_and(contains_device_loss) {
        *pending = false;
    }
    outcome?;
    *pending = false;
    calls.reset_completed()
}

/// Qualify this actual fence independently; `attempt_window_fence(runtime, fence, pending, owner)` retains coverage.
pub(super) fn attempt_window_fence(
    runtime: &VulkanRuntime,
    fence: vk::Fence,
    pending: &mut bool,
    owner: WindowFenceOwner,
) -> RetirementAttempt {
    let mut calls = WindowFenceCalls::new(runtime, fence, owner);
    attempt_pending_fence(&mut calls, pending)
}
/// Keep pending ownership distinct from diagnostics; `attempt_pending_fence(calls, pending)` refuses unknown completion.
pub(super) fn attempt_pending_fence(
    calls: &mut impl WindowFenceDispatch,
    pending: &mut bool,
) -> RetirementAttempt {
    let outcome = wait_pending_fence(calls, pending);
    match (*pending, outcome) {
        (false, outcome) => RetirementAttempt::completed(outcome),
        (true, Err(error)) => RetirementAttempt::incomplete(error),
        (true, Ok(())) => RetirementAttempt::incomplete(RenderError::rejected(
            true,
            "a cleared pending flag after successful fence completion",
        )),
    }
}
/// Block last-owner release on this exact fence; `retire_window_fence(runtime, fence, &mut pending, owner)`.
pub(super) fn retire_window_fence(
    runtime: &VulkanRuntime,
    fence: vk::Fence,
    pending: &mut bool,
    owner: WindowFenceOwner,
) -> Result<(), RenderError> {
    let mut calls = WindowFenceCalls::new(runtime, fence, owner);
    retire_pending_fence(&mut calls, pending)
}
/// Keep the owner alive through the exact fence; `retire_pending_fence(calls, pending)` may block.
pub(super) fn retire_pending_fence(
    calls: &mut impl CompletionDispatch,
    pending: &mut bool,
) -> Result<(), RenderError> {
    if !*pending {
        return Ok(());
    }
    let outcome = retire_until_stopped(calls);
    *pending = false;
    outcome
}
#[cfg(test)]
#[path = "window_sync_retirement_tests.rs"]
mod tests;
