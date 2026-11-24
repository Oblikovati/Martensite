//! Last-owner teardown may block, but never destroys resources with unqualified execution.
use super::queue_order::{QueueOrder, QueueTurn};
use super::{device_lifecycle::contains_device_loss, runtime::vk_error};
use crate::{RenderError, error::gui_observed_failure::combine_observed_result};
use std::sync::Arc;

pub(super) trait CompletionDispatch {
    fn wait_completion(&mut self) -> Result<(), RenderError>;
    fn retry_pause(&mut self);
}
/// Return only after idle or actual loss; `retire_until_stopped(&mut calls)` retains failure history.
pub(super) fn retire_until_stopped(calls: &mut impl CompletionDispatch) -> Result<(), RenderError> {
    let mut outcome = Ok(());
    loop {
        match calls.wait_completion() {
            Ok(()) => return outcome,
            Err(error) if contains_device_loss(&error) => {
                return combine_observed_result(outcome, Err(error));
            }
            Err(error) => outcome = combine_observed_result(outcome, Err(error)),
        }
        calls.retry_pause();
    }
}
/// Avoid a busy retry loop; `retirement_retry_pause()` does not impose a teardown deadline.
pub(super) fn retirement_retry_pause() {
    // A persistently failing driver can block Drop; freeing pending resources would be invalid.
    std::thread::sleep(std::time::Duration::from_millis(10));
}
pub(super) struct DeviceIdleCalls<'a> {
    device: &'a ash::Device,
    order: &'a Arc<QueueOrder>,
    held: Option<&'a QueueTurn>,
}
impl<'a> DeviceIdleCalls<'a> {
    /// Borrow the real device dispatch; `DeviceIdleCalls::new(device, &runtime.queue_order)`
    /// never takes its ownership.
    pub fn new(device: &'a ash::Device, order: &'a Arc<QueueOrder>) -> Self {
        // Destruction stays with the runtime, after its actual resource leases end.
        Self {
            device,
            order,
            held: None,
        }
    }
    /// Wait inside a queue turn the caller already holds, as the presentation thread does;
    /// `DeviceIdleCalls::new(device, order).within(Some(&turn))`.
    pub fn within(mut self, held: Option<&'a QueueTurn>) -> Self {
        self.held = held;
        self
    }
}
impl CompletionDispatch for DeviceIdleCalls<'_> {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        // An idle wait synchronizes every queue, so it also takes its place in the queue order.
        let _turn = self.held.is_none().then(|| self.order.next_turn());
        // SAFETY: the device is live, and the queue turn above or the caller's held turn keeps every other queue operation out.
        unsafe { self.device.device_wait_idle() }
            .map_err(|error| vk_error("retire device resources", error))
    }
    fn retry_pause(&mut self) {
        // Share the same nonbusy teardown policy with command and window ownership.
        retirement_retry_pause();
    }
}
#[cfg(test)]
#[path = "device_retirement_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "device_retirement_owner_tests.rs"]
mod owner_tests;
